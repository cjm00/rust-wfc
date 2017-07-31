use std::ops::Neg;
use std::convert::From;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ShapeRange2D {
    y_range: (isize, isize),
    x_range: (isize, isize),
}

impl ShapeRange2D {
    pub fn from_shape(shape: (usize, usize)) -> Self {
        let (y, x) = shape;
        let y_range = (0, y as isize);
        let x_range = (0, x as isize);
        ShapeRange2D { y_range, x_range }
    }

    pub fn shift_by<T: Into<Offset2D>>(self, shift: T) -> Self {
        let shift: Offset2D = shift.into();
        let (shift_y, shift_x) = (shift.0, shift.1);
        let y_range = (self.y_range.0 + shift_y, self.y_range.1 + shift_y);
        let x_range = (self.x_range.0 + shift_x, self.x_range.1 + shift_x);
        ShapeRange2D { y_range, x_range }
    }

    pub fn clamp_to(self, shape: (usize, usize)) -> Self {
        let max_y = shape.0 as isize;
        let max_x = shape.1 as isize;
        let y_range_min = clamp(self.y_range.0, 0, max_y);
        let y_range_max = clamp(self.y_range.1, 0, max_y);
        let x_range_min = clamp(self.x_range.0, 0, max_x);
        let x_range_max = clamp(self.x_range.1, 0, max_x);

        let y_range = (y_range_min, y_range_max);
        let x_range = (x_range_min, x_range_max);

        ShapeRange2D { y_range, x_range }
    }
}

impl IntoIterator for ShapeRange2D {
    type Item = (usize, usize);
    type IntoIter = ShapeRange2DIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let y = self.y_range.0 as usize;
        let max_y = self.y_range.1 as usize;
        let x = self.x_range.0 as usize;
        let min_x = self.x_range.0 as usize;
        let max_x = self.x_range.1 as usize;
        ShapeRange2DIntoIter {
            y,
            max_y,
            x,
            max_x,
            min_x,
        }
    }
}

pub struct ShapeRange2DIntoIter {
    max_y: usize,
    max_x: usize,
    min_x: usize,
    y: usize,
    x: usize,
}

impl Iterator for ShapeRange2DIntoIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        let output: Option<(usize, usize)>;

        if self.y < self.max_y && self.x < self.max_x {
            output = Some((self.y, self.x));
            self.x += 1;
        } else {
            self.x = self.min_x;
            self.y += 1;
            if self.y < self.max_y && self.x < self.max_x {
                output = Some((self.y, self.x));
                self.x += 1;
            } else {
                output = None
            }
        }

        output
    }
}

#[test]
fn shaperange_iter_test_1() {
    let shape = ShapeRange2D::from_shape((2, 2));
    let items: Vec<_> = shape.into_iter().collect();
    assert_eq!(vec![(0, 0), (0, 1), (1, 0), (1, 1)], items);
}

#[test]
fn shaperange_iter_test_2() {
    let shape = ShapeRange2D::from_shape((3, 3)).shift_by((3, 7));
    let items: Vec<_> = shape.into_iter().collect();
    assert_eq!(9, items.len());
}

#[test]
fn shaperange_iter_test_3() {
    let shape = ShapeRange2D::from_shape((4, 4))
        .shift_by((-1, -2))
        .clamp_to((4, 4));
    let shape2 = ShapeRange2D::from_shape((3, 2));
    let items: Vec<_> = shape.into_iter().collect();
    let items2: Vec<_> = shape2.into_iter().collect();
    assert_eq!(items, items2);
}

#[test]
fn shaperange_iter_test_4() {
    let shape = ShapeRange2D::from_shape((5, 5))
        .shift_by((100, 7))
        .clamp_to((5, 5));
    let items: Vec<_> = shape.into_iter().collect();
    assert_eq!(0, items.len());
}



#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Offset2D(isize, isize);

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


fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    if input < min {
        return min;
    } else if input > max {
        return max;
    } else {
        return input;
    }
}

#[test]
fn clamp_test_1() {
    assert_eq!(0isize, clamp(0isize, -1isize, 1isize));
}

#[test]
fn clamp_test_2() {
    assert_eq!(5isize, clamp(0isize, 5isize, 20isize));
}

#[test]
fn clamp_test_3() {
    assert_eq!(100isize, clamp(1000isize, -100isize, 100isize));
}
