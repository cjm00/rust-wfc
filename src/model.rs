#![allow(dead_code)]

use ndarray::prelude::*;
use bit_vec::BitVec;
use itertools::Itertools;

use patch::Patch2D;
use offset::Offset2D;
use possibilitycell::PossibilityCell;
use texturesource;
use pixel::Pixel;

use std::collections::HashMap;
use std::hash::Hash;

enum WrappingType {
    NoWrap,
    Torus,
}

pub struct Model2D<T: Patch2D> {
    data: Array2<PossibilityCell>,
    states: Vec<T>,
    state_counts: Vec<u32>,
    // wrap_type: WrappingType,
    compatibility_map: HashMap<(usize, Offset2D), BitVec>,
}

impl<T> Model2D<Array2<T>>
where
    T: Pixel + Hash + Eq + Copy,
{
    pub fn new() -> Self {
        unimplemented!()
    }

    fn from_texturesource(
        src: texturesource::TextureSource<T>,
        dims: (usize, usize),
        patch_size: (usize, usize),
    ) -> Self {
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
        }

    }
}

impl<T> Model2D<T>
where
    T: Patch2D,
{
    fn generate_compatibility_map<P: Patch2D>(
        states: &[P],
        patch_size: (usize, usize),
    ) -> HashMap<(usize, Offset2D), BitVec> {
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
}
