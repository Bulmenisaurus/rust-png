pub use png;
use png::RGBA;

use std::fs::File;
use std::io::prelude::*;

use rand::Rng;

fn main() {
    fn random_pixel() -> RGBA {
        let mut rng = rand::thread_rng();
        RGBA {
            red: rng.gen(),
            blue: 128,
            green: rng.gen(),
            alpha: rng.gen(),
        }
    }
    let mut data = Vec::new();
    let height = 250;
    let width = 250;
    for _ in 0..(height * width) {
        data.push(random_pixel());
    }
    let image_data = png::create_image(data, width, height);

    let file_path = "./images/random-image.png";

    let mut png_file = File::create(file_path)
        .unwrap_or_else(|_| panic!("Could not create/write to `{}`", file_path));

    png_file
        .write_all(&image_data[..])
        .unwrap_or_else(|_| panic!("Could not write to `{}`", file_path));
}
