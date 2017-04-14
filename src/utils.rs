use ndarray::prelude::*;
use rand::distributions::{Range, IndependentSample};
use rand;


pub fn rotate_90_clockwise<T: Copy>(image_data: &Array2<T>) -> Array2<T> {
    let mut output = image_data.t();
    output.invert_axis(Axis(1));
    output.to_owned()
}

pub fn masked_weighted_choice<T, M: IntoIterator<Item = bool> + Copy>(input: &[(T, usize)],
                                                                      mask: M)
                                                                      -> usize {
    /// Returns an index from the slice of (T, u) where u is the integer weight, i.e.
    /// [(1, 3), (2, 1), (3, 1)] returns 0 (the index of 1) with probability 3/5

    let total: usize = input.iter()
        .map(|&(_, u)| u)
        .zip(mask.into_iter())
        .filter(|&(_, m)| m)
        .map(|(u, _)| u)
        .sum();
    let between = Range::new(0, total);
    let mut rng = rand::thread_rng();
    let mut choice: usize = between.ind_sample(&mut rng);

    for ((index, u), mask) in input.iter().map(|&(_, u)| u).enumerate().zip(mask.into_iter()) {
        if mask {
            if choice < u {
                return index;
            }
            choice = choice.saturating_sub(u);
        } else {
            continue;
        }
    }
    unreachable!();
}
