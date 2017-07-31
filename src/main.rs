#![allow(dead_code)]


extern crate bit_vec;
extern crate png;
extern crate ndarray;
extern crate rand;
extern crate chrono;
extern crate num_traits;

mod overlappingmodel;
mod sourceimage;
mod utils;
mod offset;
mod patch;

use chrono::prelude::*;

use std::path::Path;
use std::fs::create_dir;

static INPUT: &'static str = "./assets/Knot.png";
static OUTPUT_DIR: &'static str = "./output";


fn main() {
    // Make sure output folder exists
    if !(Path::new(OUTPUT_DIR).is_dir()) {
        match create_dir(OUTPUT_DIR) {
            Err(_) => panic!("Don't have permission to make files here"),
            Ok(_)  => (),
        };
    }

    let im = sourceimage::SeedImage::from_file(INPUT);
    let model = overlappingmodel::OverlappingModel::from_seed_image(im, (50, 50), 3);

    match model.collapse_and_propagate() {
        Ok(_) => {
            let now: i64 = Local::now().timestamp();
            model.to_image(&format!("{}/output{}.png", OUTPUT_DIR, now))
        }
        Err(u) => {
            println!("{:?}", u);
            let now: i64 = Local::now().timestamp();
            model.to_image(&format!("{}/output{}.png", OUTPUT_DIR, now));
        }
    }
}
