use std::convert::From;
use std::ops::Neg;
use num_traits::cast::NumCast;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Offset2D(pub isize, pub isize);

impl Neg for Offset2D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Offset2D(-self.0, -self.1)
    }
}

impl<T, U> From<(T, U)> for Offset2D
where
    T: NumCast,
    U: NumCast,
{
    fn from(src: (T, U)) -> Self {
        Offset2D(src.0.to_isize().unwrap(), src.1.to_isize().unwrap())
    }
}
