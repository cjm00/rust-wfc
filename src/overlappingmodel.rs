#![allow(dead_code)]


use utils::*;

use bit_vec::BitVec;
use sourceimage::{Color, SeedImage};
use ndarray::prelude::*;

use std::collections::HashMap;
use std::cell::RefCell;
use std::{f64, usize};
use std::hash::Hash;
use std::convert::{TryInto};

enum ModelError {
    NoValidStates((usize, usize)),
    UnexpectedNaN((usize, usize)),
    AllStatesDecided,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum WrappingType {
    NoWrap,
    Torus
}

#[derive(Debug)]
struct UncertainCell {
    possible_colors: RefCell<BitVec>,
    possible_states: RefCell<BitVec>,
}

impl UncertainCell {
    pub fn new(num_colors: usize, num_states: usize) -> UncertainCell {
        let possible_colors = RefCell::new(BitVec::from_elem(num_colors, true));
        let possible_states = RefCell::new(BitVec::from_elem(num_states, true));
        UncertainCell {
            possible_colors: possible_colors,
            possible_states: possible_states,
        }
    }

    pub fn entropy<T>(&self, concrete_states: &[(T, usize)]) -> Option<f64> {
        let possible_states = self.possible_states.borrow();
        debug_assert!(possible_states.len() == concrete_states.len());

        if possible_states.none() {
            return None;
        };
        if possible_states.iter().filter(|p| *p).count() == 1 {
            return Some(0.);
        };

        // Counts the number of possible states permitted by the UncertainCell
        let possible_state_count: usize = concrete_states.iter()
            .map(|&(_, count)| count)
            .zip(possible_states.iter())
            .filter(|&(_, p)| p)
            .map(|(count, _)| count)
            .sum();

        let possible_state_count = possible_state_count as f64;
        let entropy: f64 = concrete_states.iter()
            .map(|&(_, count)| count)
            .zip(possible_states.iter())
            .filter(|&(_, p)| p)
            .map(|(count, _)| {
                let x = count as f64 / possible_state_count;
                x * x.ln()
            })
            .sum();

        Some(-entropy)

    }

    pub fn collapse<T>(&self, concrete_states: &[(T, usize)]) {
        /// Marks all but a single state of the BitVec as forbidden, randomly chosen
        /// from the states still permitted and weighted by their frequency in the original image.
        let mut possible_states = self.possible_states.borrow_mut();
        let chosen_state = masked_weighted_choice(concrete_states, &*possible_states).unwrap();
        possible_states.clear();
        possible_states.set(chosen_state, true);
    }
}


struct OverlappingModel {
    model: Array2<UncertainCell>,
    palette: Vec<Color>,
    states: Vec<(Array2<Color>, usize)>,
    state_size: usize,
    wrap: WrappingType,
    color_changes: RefCell<Vec<(usize, usize)>>,
    state_changes: RefCell<Vec<(usize, usize)>>,
}

impl OverlappingModel {
    pub fn from_seed_image(seed_image: SeedImage,
                           output_dims: (usize, usize),
                           block_size: usize)
                           -> OverlappingModel {
        let palette = OverlappingModel::build_color_palette(&seed_image.image_data);
        let states = OverlappingModel::build_block_frequency_map(&seed_image.image_data,
                                                                 block_size);

        let num_colors = palette.len();
        let num_states = states.len();
        let (x, y) = output_dims;
        let mut model_data = Vec::<UncertainCell>::with_capacity(x * y);

        for _ in 0..(x * y) {
            model_data.push(UncertainCell::new(num_colors, num_states));
        }
        let model = Array::from_shape_vec((y, x), model_data).unwrap();

        //TODO add wrapping patches

        OverlappingModel {
            model: model,
            palette: palette,
            states: states,
            state_size: block_size,
            wrap: WrappingType::NoWrap,
            color_changes: RefCell::new(vec![]),
            state_changes: RefCell::new(vec![]),
        }
    }

    fn find_lowest_nonzero_entropy_coordinates(&self) -> Result<(usize, usize), ModelError> {
        let mut output: Option<(usize, usize)> = None;
        let mut entropy: f64 = f64::MAX;
        for (index, cell) in self.model.indexed_iter() {
            match cell.entropy(&self.states) {
                None => return Err(ModelError::NoValidStates(index)),
                Some(u) if u > 0. => {
                    if u <= entropy {
                        entropy = u;
                        output = Some(index);
                    } else if u.is_nan() {
                        return Err(ModelError::UnexpectedNaN(index));
                    };
                }
                Some(_) => continue,

            }
        }
        match output {
            None => Err(ModelError::AllStatesDecided),
            Some(u) => Ok(u),
        }
    }

    fn color_to_index(&self, color: &Color) -> usize {
        self.palette.binary_search(color).expect("Color not found in palette!")
    }


    fn valid_states_at_position(&self, position: (usize, usize)) -> BitVec {
        /// Queries an NxN grid with the top left at function argument "position" for the states
        /// that their current color possibilities allow, then takes the intersection of all of
        /// those possibilites.

        let s = self.state_size;
        let wrap = self.wrap;
        let mut patch_possibilites = Vec::<BitVec>::with_capacity(s*s);
        let cell_states = self.model[position].possible_states.borrow();

        for t in 0..s*s {
            let pixel_coords = (t / s, t % s);
            let cell_coords = (pixel_coords.0 + position.0, pixel_coords.1 + position.1);
            match wrap {
                WrappingType::NoWrap => if !self.valid_coord(cell_coords) {continue},
                WrappingType::Torus => unimplemented!()
            }


            let color_states = self.model[cell_coords].possible_colors.borrow();
            let new_cell_states: BitVec = cell_states.iter()
                .enumerate()
                .map(|(i, x)| if x {
                    let c = self.color_to_index(&self.states[i].0[pixel_coords]);
                    color_states.get(c).unwrap()
                } else {
                    false
                })
                .collect();

            patch_possibilites.push(new_cell_states);
        }

        mass_intersect(patch_possibilites).unwrap()
    }

    fn valid_colors_at_position(&self, position: (usize, usize)) -> BitVec {
        // Much cast, very wow
        let wrap = self.wrap;
        let s: isize = self.state_size.try_into().unwrap();
        let mut patch_possibilites = Vec::<BitVec>::with_capacity((s*s) as usize);
        let pos = (position.0 as isize, position.1 as isize);

        for t in 0..s*s {
            let pixel_coords = ((t / s) as usize, (t % s) as usize);
            let offset = (pixel_coords.0 as isize, pixel_coords.1 as isize);
            let cell_coords = (pos.0 - offset.0, pos.1 - offset.1);
            match wrap {
                WrappingType::NoWrap => if !self.valid_coord(cell_coords) {continue},
                WrappingType::Torus => unimplemented!()
            }
            let cell_coords = (cell_coords.0 as usize, cell_coords.1 as usize);

            let cell_states = self.model[cell_coords].possible_states.borrow();

            let mut new_color_states: BitVec = BitVec::from_elem(self.palette.len(), false);

            for state_index in cell_states.iter().enumerate().filter(|&(_, s)| s).map(|(i, _)| i) {
                let v = self.color_to_index(&self.states[state_index].0[pixel_coords]);
                new_color_states.set(v, true);
            }
            patch_possibilites.push(new_color_states);

        }

        mass_intersect(patch_possibilites).unwrap()
    }

    fn valid_coord<T: TryInto<usize>>(&self, coord: (T, T)) -> bool {
        let y: usize = match coord.0.try_into() {
            Ok(u) => u,
            Err(_) => return false
        };
        let x: usize = match coord.1.try_into() {
            Ok(u) => u,
            Err(_) => return false
        };
        let (safe_y, safe_x) = self.model.dim();

        (y < safe_y) && (x < safe_x)
    }

    fn wrap_coord<T: TryInto<usize>>(&self, coord: (T, T)) -> Result<(usize, usize), ()> {
        unimplemented!()
    }

    fn build_color_palette(image_data: &Array2<Color>) -> Vec<Color> {
        let mut palette: Vec<Color> = image_data.iter().cloned().collect();
        palette.sort();
        palette.dedup();
        palette
    }

    fn build_block_frequency_map<T: Copy + Eq + Hash>(image_data: &Array2<T>,
                                 block_size: usize)
                                 -> Vec<(Array2<T>, usize)> {
        let mut block_counts = HashMap::new();

        //TODO augment with rotations and reflections
        for block in image_data.windows((block_size, block_size)) {
            let block = block.to_owned();
            let count = block_counts.entry(block).or_insert(0);
            *count += 1;
        }

        block_counts.into_iter().collect()
    }
}

#[test]
fn color_palette_test() {
    let array = Array2::from_shape_vec((3, 3), vec![Color(0, 0, 0),
                                                    Color(1, 1, 1),
                                                    Color(1, 1, 1),
                                                    Color(0, 0, 0),
                                                    Color(0, 0, 1),
                                                    Color(0, 0, 1),
                                                    Color(0, 0, 1),
                                                    Color(0, 0, 1),
                                                    Color(0, 0, 2)]).unwrap();

    let p = vec![Color(0, 0, 0), Color(0, 0, 1), Color(0, 0, 2), Color(1, 1, 1)];
    let p_test = OverlappingModel::build_color_palette(&array);
    assert_eq!(p, p_test);
}

#[test]
fn build_block_frequency_map_test_1() {
    let array = Array2::from_shape_vec((3, 3), vec![Color(0, 0, 0),
                                                    Color(1, 1, 1),
                                                    Color(1, 1, 1),
                                                    Color(0, 0, 0),
                                                    Color(0, 0, 1),
                                                    Color(0, 0, 1),
                                                    Color(0, 0, 1),
                                                    Color(0, 0, 1),
                                                    Color(0, 0, 2)]).unwrap();
    let p_test = OverlappingModel::build_block_frequency_map(&array, 2);
    assert_eq!(p_test.len(), 4);
}

#[test]
fn build_block_frequency_map_test_2() {
    let array: Array2<usize> = Array2::eye(10);
    let p_test = OverlappingModel::build_block_frequency_map(&array, 2);
    let p_count: usize = p_test.iter().map(|&(_, u)| u).sum();
    assert_eq!(p_count, 81);
}
