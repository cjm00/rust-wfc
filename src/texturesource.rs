use image::{self, ImageFormat};
use ndarray::prelude::*;

use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::hash::Hash;
use std::collections::HashMap;

use pixel::Pixel;

pub struct TextureSource<T: Pixel + Copy + Hash + Eq> {
    pub img: Array2<T>,
}

impl TextureSource<[u8; 3]> {
    pub fn from_file<T: AsRef<Path>>(p: T) -> Result<TextureSource<[u8; 3]>, ()> {
        let p: &Path = p.as_ref();
        let buf = BufReader::new(File::open(p).unwrap());
        let src_img = image::load(buf, ImageFormat::PNG).unwrap().to_rgb();

        let dims = src_img.dimensions();
        let dims = (dims.0 as usize, dims.1 as usize);

        let src_img_data = src_img.into_raw();
        let src_img_data = src_img_data.chunks(3).map(|s| [s[0], s[1], s[2]]).collect();
        let image_data = Array2::from_shape_vec(dims, src_img_data).unwrap();
        Ok(TextureSource { img: image_data })
    }
}

impl<T> TextureSource<T>
where
    T: Pixel + Copy + Hash + Eq,
{
    pub fn states_and_counts(&self, patch_size: (usize, usize)) -> (Vec<Array2<T>>, Vec<u32>) {
        let mut patch_counts = HashMap::<Array2<T>, u32>::new();

        for patch in self.img.windows(patch_size) {
            let count = patch_counts.entry(patch.to_owned()).or_insert(0);
            *count += 1;
        }

        let mut states = Vec::<Array2<T>>::with_capacity(patch_counts.len());
        let mut counts = Vec::<u32>::with_capacity(patch_counts.len());

        for (s, c) in patch_counts {
            states.push(s);
            counts.push(c);
        }

        (states, counts)
    }
}
