pub use png;

use std::fs::File;
use std::io::prelude::*;

use png::RGBA;

fn get_whole_part(numerator: i32, denominator: i32) -> (i32, i32, i32) {
    let whole_part = numerator / denominator;

    let numerator = numerator - denominator * whole_part;

    (whole_part, numerator, denominator)
}

fn continued_fraction_length(numerator: i32, denominator: i32) -> Result<i32, &'static str> {
    if denominator == 0 {
        return Err("division by 0");
    }

    if numerator == 0 {
        return Ok(1);
    }

    let mut continued_representation_length: i32 = 0;

    let mut n = numerator;
    let mut d = denominator;

    if n == 1 {
        return Ok(1);
    }

    while n > 1 {
        let (whole_part, new_numerator, new_denominator) = get_whole_part(n, d);
        continued_representation_length += 1;

        if new_numerator == 0 {
            break;
        }

        n = new_denominator;
        d = new_numerator;
    }

    Ok(continued_representation_length)
}

fn get_pixel_color(x: i32, y: i32) -> RGBA {
    if x == 0 || y == 0 {
        return RGBA {
            red: 0,
            blue: 0,
            green: 0,
            alpha: 255,
        };
    }

    let fraction_representation = continued_fraction_length(x, y);

    let darkness = fraction_representation.unwrap_or(0);
    let darkness: u8 = u8::try_from(darkness * 19).unwrap_or(u8::MAX);

    RGBA {
        red: darkness,
        blue: darkness,
        green: darkness,
        alpha: 255,
    }
}

fn main() {
    let mut data = Vec::new();
    let height = 512;
    let width = 512;

    let mut longest_representation = 0u8;
    let mut longest_frac: (i32, i32) = (0, 0);

    for y in 0..height {
        for x in 0..width {
            let pixel = get_pixel_color(x, y);

            if pixel.red > longest_representation {
                longest_representation = pixel.red;
                longest_frac = (x, y)
            }

            data.push(pixel)
        }
    }

    println!(
        "longest value: {:?}, {:?}",
        longest_representation, longest_frac
    );

    let image_data = png::create_image(data, width, height);

    let file_path = "./images/continued-fraction.png";

    let mut png_file = File::create(file_path)
        .unwrap_or_else(|_| panic!("Could not create/write to `{}`", file_path));

    png_file
        .write_all(&image_data[..])
        .unwrap_or_else(|_| panic!("Could not write to `{}`", file_path));
}
