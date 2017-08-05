use image::{Rgb};

pub trait Pixel {}

impl Pixel for Rgb<u8> {}

impl Pixel for usize {}

impl Pixel for [u8; 3] {}
