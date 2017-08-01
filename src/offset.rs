use std::convert::From;
use std::ops::Neg;


#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Offset2D(pub isize, pub isize);

impl Neg for Offset2D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Offset2D(-self.0, -self.1)
    }
}

impl From<(isize, isize)> for Offset2D {
    fn from(src: (isize, isize)) -> Self {
        Offset2D(src.0, src.1)
    }
}

impl From<(usize, usize)> for Offset2D {
    fn from(src: (usize, usize)) -> Self {
        Offset2D(src.0 as isize, src.1 as isize)
    }
}