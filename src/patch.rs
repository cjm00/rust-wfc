use shape::{Offset2D, ShapeRange2D};
use ndarray::prelude::*;


pub trait Patch2D {
    fn compare_with_offset<T: Into<Offset2D>>(&self, other: &Self, offset: T) -> bool;
    fn shape(&self) -> (usize, usize);
}

impl<T> Patch2D for Array2<T>
where
    T: Copy + Eq,
{
    fn shape(&self) -> (usize, usize) {
        self.dim()
    }

    fn compare_with_offset<U: Into<Offset2D>>(&self, other: &Self, offset: U) -> bool {
        let offset: Offset2D = offset.into();
        let self_coords = ShapeRange2D::from_shape(Patch2D::shape(other))
            .shift_by(offset)
            .clamp_to(Patch2D::shape(self))
            .into_iter();
        let other_coords = ShapeRange2D::from_shape(Patch2D::shape(self))
            .shift_by(-offset)
            .clamp_to(Patch2D::shape(other))
            .into_iter();

        self_coords.zip(other_coords).all(
            |(s, o)| self[s] == other[o],
        )
    }
}

#[test]
fn array2_shape_test() {
    let foo: Array2<usize> = Array2::eye(5);
    assert_eq!((5usize, 5usize), Patch2D::shape(&foo));
}

#[test]
fn array2_patch2d_compare_test_1() {
    let a = arr2(&[[0usize, 1, 2], [3, 4, 5], [6, 7, 8]]);
    let b = arr2(&[[4usize, 5], [7, 8]]);
    assert!(a.compare_with_offset(&b, (1, 1)));
}

#[test]
fn array2_patch2d_compare_test_2() {
    let a = arr2(&[[0usize, 1, 2], [3, 4, 5], [6, 7, 8]]);
    let b = arr2(&[[4usize, 5], [7, 0]]);
    assert!(a.compare_with_offset(&b, (-1, -1)));
}

#[test]
fn array2_patch2d_compare_test_3() {
    let a = arr2(&[[0usize, 1, 2], [3, 4, 5], [6, 7, 8]]);
    let b = arr2(&[[100usize, 500], [700, 10000]]);
    assert!(a.compare_with_offset(&b, (-4, -4)));
}

#[test]
fn array2_patch2d_compare_test_4() {
    let a = arr2(&[[0usize, 1, 2], [3, 4, 5], [6, 7, 8]]);
    let b = arr2(&[[4usize, 4, 4], [4, 4, 4], [4, 4, 4]]);
    assert!(!a.compare_with_offset(&b, (1, 1)));
}

#[test]
fn array2_patch2d_compare_test_5() {
    let a = arr2(&[[0usize, 1, 2], [3, 4, 5], [6, 7, 8]]);
    let b = arr2(&[[5usize, 4, 4], [8, 4, 4], [4, 4, 4]]);
    assert!(a.compare_with_offset(&b, (1, 2)));
}