#![feature(let_else)]

use std::path::{Path, PathBuf};

use capturing_glob::glob;
use clap::Parser;
use image::imageops::{self, colorops};
use rayon::prelude::*;

const WIDTH: u16 = 256;
const HEIGHT: u16 = 128;

/// Simple program to process images
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Path of directory containing images to process
    #[clap(short, long, default_value = "./captchas")]
    dir: String,
    /// Extension of images to process
    #[clap(short, long, default_value = "jpg")]
    ext: String,
    /// Path of directory to store processed images
    #[clap(short, long, default_value = "./processed")]
    output: String,
    // /// Is performing grayscale conversion
    // #[clap(short, long)]
    // grayscale: bool,
    /// Brightness adjustment between -255 and 255
    #[clap(short, long, default_value = "50")]
    brightness: i32,
}

fn main() {
    let args = Args::parse();

    let mut path = PathBuf::new();
    path.push(args.dir);
    path.push("(**/*)");
    path.set_extension(args.ext);

    let path = Path::new(&path).to_str().expect("Invalid path");
    let glob: Vec<_> = glob(path).expect("Failed to read glob pattern").collect();

    glob.into_par_iter().enumerate().for_each(|(index, entry)| {
        let Ok(entry) = entry else {
            return;
        };
        let mut image = image::open(entry).unwrap();
        colorops::brighten_in_place(&mut image, args.brightness);
        colorops::contrast_in_place(&mut image, 100.0);
        let mut output = colorops::grayscale(&image);
        let asd = ['A', 'B', 'C', 'D', 'E', 'F'];
        let h = HEIGHT / 2;
        let w = WIDTH / 3;
        for j in 0..2 {
            for i in 0..3 {
                let asd_index = (i + 3 * j) as usize;
                let y = h * j + j;
                let x = w * i + i;
                let output = imageops::crop(&mut output, x as u32, y as u32, 84, 63);
                let path =
                    Path::new(&args.output).join(format!("{}-{}.jpg", index, asd[asd_index]));
                output.to_image().save(path).unwrap();
            }
        }
    });
}
