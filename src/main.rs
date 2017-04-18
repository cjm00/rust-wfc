#![allow(dead_code)]
#![feature(try_from)]


extern crate bit_vec;
extern crate png;
extern crate ndarray;
extern crate rand;
extern crate chrono;

mod overlappingmodel;
mod sourceimage;
mod utils;

use chrono::prelude::*;

static INPUT: &'static str = "./assets/Knot.png";


fn main() {
    let im = sourceimage::SeedImage::from_file(INPUT);
    let model = overlappingmodel::OverlappingModel::from_seed_image(im, (50, 50), 3);
    match model.collapse_and_propagate() {
        Ok(_) => {
            let now: i64 = Local::now().timestamp();
            model.to_image(&format!("./output/output{}.png", now))
        }
        Err(u) => {
            println!("{:?}", u);
            let now: i64 = Local::now().timestamp();
            model.to_image(&format!("./output/output{}.png", now));
        }
    }
}
