use shape::{Offset2D, ShapeRange2D};

pub trait Patch2D {
    fn compare_with_offset(&self, other: Self, offset: Offset2D) -> bool;

    fn shape(&self) -> (usize, usize);
}
