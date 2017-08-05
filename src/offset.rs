use std::convert::From;
use std::ops::{Neg, Sub};
use num_traits::cast::NumCast;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Offset2D(pub isize, pub isize);

impl Offset2D {
    pub fn from_difference<T, U>(from: T, to: U) -> Self
    where
        T: Into<Offset2D>,
        U: Into<Offset2D>,
    {
        to.into() - from.into()
    }
}

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

impl Sub<Self> for Offset2D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Offset2D(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[test]
fn from_difference_test() {
    let base = Offset2D(1, 1);
    let test = Offset2D::from_difference((44, 44), (45, 45));
    assert_eq!(base, test);
}
