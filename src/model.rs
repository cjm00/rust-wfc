#![allow(dead_code)]

use ndarray::prelude::*;
use bit_vec::BitVec;
use itertools::Itertools;
use rand;
use image::{self, Rgb};

use patch::Patch2D;
use offset::Offset2D;
use possibilitycell::PossibilityCell;
use texturesource;
use pixel::Pixel;
use shape::ShapeRange2D;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::f64;
use std::path::Path;


const NOISE_LEVEL: f64 = 1.;

enum WrappingType {
    NoWrap,
    Torus,
}

pub struct Model2D<T: Patch2D> {
    data: Array2<PossibilityCell>,
    states: Vec<T>,
    state_counts: Vec<u32>,
    wrap_type: WrappingType,
    compatibility_map: HashMap<(usize, Offset2D), BitVec>,
    patch_size: (usize, usize),
    updates: HashSet<(usize, usize)>,
}

impl<T> Model2D<T>
where
    T: Patch2D,
{

    pub fn run(&mut self) {
        while !self.data.iter().all(|s| s.decided()) {
            self.pick_a_point_and_collapse();
            self.propagate_changes();
        }
    }

    fn generate_compatibility_map<P: Patch2D>(states: &[P],
                                              patch_size: (usize, usize))
                                              -> HashMap<(usize, Offset2D), BitVec> {
        let y_min: isize = -(patch_size.0.saturating_sub(1) as isize);
        let y_max: isize = patch_size.0 as isize;

        let x_min: isize = -(patch_size.1.saturating_sub(1) as isize);
        let x_max: isize = patch_size.1 as isize;

        let mut map: HashMap<_, _> = HashMap::new();

        for offset in (y_min..y_max).cartesian_product(x_min..x_max) {
            for (i, s) in states.iter().enumerate() {
                let compatible_states: BitVec = states
                    .iter()
                    .map(|o| s.compare_with_offset(o, offset))
                    .collect();
                map.insert((i, offset.into()), compatible_states);
            }
        }

        map
    }

    fn propagate_changes(&mut self) {
        while !self.updates.is_empty() {
            let work: Vec<(usize, usize)> = self.updates.drain().collect();
            for position in work {
                self.update_states_from_position(position);
            }
        }
    }

    fn pick_a_point_and_collapse(&mut self) {
        let chosen_point = self.lowest_nonzero_entropy_coordinates().expect("Inconsistent state reached");
        self.data[chosen_point].collapse(&self.state_counts);
        self.updates.insert(chosen_point);
    }

    fn lowest_nonzero_entropy_coordinates(&self) -> Option<(usize, usize)> {
        let mut output: Option<(usize, usize)> = None;
        let mut entropy: f64 = f64::MAX;

        for (index, cell) in self.data.indexed_iter() {
            match cell.entropy(&self.state_counts) {
                None => return None,
                Some(u) if u > 0. => {
                    let noise = rand::random::<f64>() * NOISE_LEVEL;
                    let u = u + noise;
                    if u < entropy {
                        entropy = u;
                        output = Some(index);
                    }

                }
                Some(_) => continue,

            }
        }

        output
    }

    fn update_states_from_position(&mut self, position: (usize, usize)) {
        let effect_size = ((self.patch_size.0 * 2) - 1, (self.patch_size.1 * 2) - 1);
        let shift: Offset2D = Offset2D::from(self.patch_size) - Offset2D::from((1, 1));
        let cell_indices = ShapeRange2D::from_shape(effect_size)
            .shift_by(position)
            .shift_by(-shift)
            .clamp_to(self.data.dim());

        for index in cell_indices {
            let offset = Offset2D::from_difference(position, index);
            let permitted_states = self.states_permitted_from_position_by_offset(position, offset);
            let target = self.data.get_mut(index).unwrap();
            if target.possible_states.intersect(&permitted_states) {
                self.updates.insert(index);
            }
        }

    }

    fn states_permitted_from_position_by_offset(&self, position: (usize, usize), offset: Offset2D) -> BitVec {
        let mut output = BitVec::from_elem(self.states.len(), false);
        for (i, v) in self.data[position].possible_states.iter().enumerate() {
            if v {
                output.union(&self.compatibility_map[&(i, offset)]);
            }
        }

        output
    }
}


impl<T> Model2D<Array2<T>>
where
    T: Pixel + Hash + Eq + Copy,
{
    fn from_texturesource(src: texturesource::TextureSource<T>,
                          dims: (usize, usize),
                          patch_size: (usize, usize))
                          -> Self {
        let (states, state_counts) = src.states_and_counts(patch_size);
        assert_eq!(states.len(), state_counts.len());
        let data: Array2<_> = Array::from_elem(dims, PossibilityCell::new(states.len()));

        let compatibility_map: HashMap<(usize, Offset2D), BitVec> =
            Model2D::<Array2<T>>::generate_compatibility_map(&states, patch_size);

        Model2D {
            data,
            states,
            state_counts,
            compatibility_map,
            wrap_type: WrappingType::NoWrap,
            patch_size,
            updates: HashSet::with_capacity(64),
        }

    }
}

impl Model2D<Array2<Rgb<u8>>> {
     pub fn new<P: AsRef<Path>>(file_path: P, output_size: (usize, usize), patch_size: (usize, usize)) -> Self {
        let texture = texturesource::TextureSource::from_file(file_path).unwrap();
        Self::from_texturesource(texture, output_size, patch_size)
    }

    pub fn to_file<P: AsRef<Path>>(&self, file_path: P) {
        let output_y = self.data.dim().0 as u32;
        let output_x = self.data.dim().1 as u32;
        let mut raw_data: Vec<u8> = Vec::with_capacity(self.data.len() * 3);

        for rgb in self.data.iter().map(|s| s.to_output_state(&self.states)) {
            raw_data.push(rgb[0]);
            raw_data.push(rgb[1]);
            raw_data.push(rgb[2]);
        } 

        let final_image = image::RgbImage::from_raw(output_x, output_y, raw_data).unwrap();
        final_image.save(file_path).unwrap();
    }
}