#![allow(dead_code)]
#![feature(try_from)]


extern crate bit_vec;
extern crate png;
extern crate ndarray;
extern crate rand;
extern crate ordered_float;

mod overlappingmodel;
mod sourceimage;
mod utils;

static INPUT: &'static str = "./assets/Knot.png";
static OUTPUT: &'static str = "./assets/first_output.png";


fn main() {
    let im = sourceimage::SeedImage::from_file(INPUT);
    let model = overlappingmodel::OverlappingModel::from_seed_image(im, (100, 100), 3);
    match model.collapse_and_propagate() {
        Ok(_) => model.to_image(OUTPUT),
        Err(u) => println!("{:?}", u),
    }
}
