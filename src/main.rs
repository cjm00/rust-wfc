#![allow(dead_code)]

extern crate ndarray;
extern crate rand;
extern crate chrono;
extern crate num_traits;
extern crate image;
extern crate bit_vec;
extern crate itertools;

mod utils;
mod offset;
mod patch;
mod shape;
mod pixel;
mod texturesource;
mod model;
mod possibilitycell;

use chrono::prelude::*;

use std::path::Path;
use std::fs::create_dir;

static INPUT: &'static str = "./assets/Flowers.png";
static OUTPUT_DIR: &'static str = "./output";


fn main() {
    // Make sure output folder exists
    if !(Path::new(OUTPUT_DIR).is_dir()) {
        match create_dir(OUTPUT_DIR) {
            Err(_) => panic!("Don't have permission to make files here"),
            Ok(_) => (),
        };
    }

    let mut model: model::Model2D<_> = model::Model2D::new(INPUT, (100, 100), (3, 3));
    model.run();
    
    let now: i64 = Local::now().timestamp();
    model.to_file(&format!("{}/output{}.png", OUTPUT_DIR, now))
}
