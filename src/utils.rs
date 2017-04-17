use ndarray::prelude::*;
use rand::distributions::{Range, IndependentSample};
use rand;
use bit_vec::BitVec;

pub fn rotate_90_clockwise<T: Copy>(image_data: &Array2<T>) -> Array2<T> {
    let mut output = image_data.t();
    output.invert_axis(Axis(1));
    output.to_owned()
}

pub fn masked_weighted_choice<T, M>(input: &[(T, usize)], mask: &M) -> Option<usize>
    where for<'a> &'a M: IntoIterator<Item = bool>
{
    /// Returns an index from the slice of (T, u) where u is the integer weight, i.e.
    /// [('a', 3), ('b', 1), ('c', 1)] returns 0 (the index of 'a') with probability 3/5

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
                return Some(index);
            }
            choice = choice.saturating_sub(u);
        }
    }
    None
}

pub fn mass_intersect(sets: Vec<BitVec>) -> Option<BitVec> {
    let mut output = None;
    for bv in sets {
        match output {
            None => {
                output = Some(bv);
            }
            Some(ref mut v) => {
                v.intersect(&bv);
            }
        }
    }
    output
}


#[test]
fn mass_intersect_empty_test() {
    let test_vec = vec![];
    let output = mass_intersect(test_vec);
    assert_eq!(output, None);
}

#[test]
fn mass_intersect_single_test() {
    let test_vec = vec![BitVec::from_elem(10, true)];
    let output = mass_intersect(test_vec);
    let result = Some(BitVec::from_elem(10, true));
    assert_eq!(output, result);
}

#[test]
fn mass_intersect_multi_test() {
    let mut test_vec = vec![];
    let test1 = BitVec::from_elem(8, true);
    let test2 = BitVec::from_bytes(&[0b11110000]);
    let test3 = BitVec::from_bytes(&[0b11110011]);
    test_vec.push(test1);
    test_vec.push(test2);
    test_vec.push(test3);
    let output = mass_intersect(test_vec);
    let result = Some(BitVec::from_bytes(&[0b11110000]));
    assert_eq!(output, result);
}
