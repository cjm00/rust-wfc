pub trait Pixel {}

impl Pixel for usize {}

impl Pixel for [u8; 3] {}