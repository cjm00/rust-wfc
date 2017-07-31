use png::{Encoder, Decoder, ColorType, BitDepth, HasParameters, OutputInfo};
use ndarray::prelude::*;


use std::fs::File;
use std::path::Path;
use std::io::BufWriter;


/// A tuple struct of red, green, blue.
#[derive(Copy, Clone, Hash, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Color(pub u8, pub u8, pub u8);

pub struct SeedImage {
    pub image_data: Array2<Color>,
    pub image_info: OutputInfo,
}

impl SeedImage {
    pub fn from_file(file_path: &str) -> SeedImage {
        let (image, info) = match SeedImage::load_file(file_path) {
            Ok(u) => u,
            Err(t) => {
                println!("{}", t);
                panic!();
            }
        };

        SeedImage {
            image_data: image,
            image_info: info,
        }
    }

    pub fn to_file(&self, file_path: &str) -> () {
        let (y, x) = self.image_data.dim();
        let file_path = Path::new(file_path);
        let file = File::create(file_path).unwrap();
        let w = &mut BufWriter::new(file);
        let mut encoder = Encoder::new(w, x as u32, y as u32);
        encoder.set(ColorType::RGB).set(BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        let mut raw_data = Vec::<u8>::with_capacity(self.image_data.len() * 3);
        for rgb in self.image_data.iter().cloned() {
            raw_data.push(rgb.0);
            raw_data.push(rgb.1);
            raw_data.push(rgb.2);

        }

        writer.write_image_data(&raw_data).unwrap();

    }



    fn load_file(file_path: &str) -> Result<(Array2<Color>, OutputInfo), String> {
        let dec = Decoder::new(File::open(file_path).unwrap());
        let (info, mut reader) = dec.read_info().unwrap();
        match (info.color_type, info.bit_depth) {
            (ColorType::RGB, BitDepth::Eight) => {}
            (j, k) => {
                return Err(format!(
                    "Wrong color type or bit depth. Found color type: {:?} and \
                                    bit depth: {:?}",
                    j,
                    k
                ))
            }
        }

        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();
        let image_data: Vec<_> = buf.chunks(3).map(|s| Color(s[0], s[1], s[2])).collect();
        let image_data =
            Array::from_shape_vec((info.height as usize, info.width as usize), image_data).unwrap();
        Ok((image_data, info))
    }
}
