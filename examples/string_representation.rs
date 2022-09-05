pub use png;

use std::fs::File;
use std::io::prelude::*;

use png::RGBA;

const SMALL: &[&str] = &[
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: &[&str] = &[
    "PANIC", "PANIC", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const MAGNITUDE: &[&str] = &[
    "PANIC",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

fn wordify(mut number: i64) -> String {
    if number == 0 {
        return String::from("zero");
    }

    let mut res = String::new();

    if number < 0 {
        res += "negative ";
        number = -number;
    }

    while number != 0 {
        if number < 20 {
            res += SMALL[number as usize];
            break;
        } else if number < 100 {
            res += TENS[number as usize / 10];

            number %= 10;
            if number != 0 {
                res += "-";
            }
        } else if number < 1_000 {
            res += &format!("{} hundred", SMALL[number as usize / 100]);
            number %= 100;
            if number != 0 {
                res += " and ";
            }
        } else {
            let mut top = number;
            let mut magnitude = 0i64;
            let mut magnitude_pow = 1i64;
            while top >= 1_000 {
                top /= 1_000;
                magnitude += 1;
                magnitude_pow *= 1_000;
            }
            res += &wordify(top);
            number %= magnitude_pow;

            if number == 0 {
                res += &format!(" {}", MAGNITUDE[magnitude as usize]);
            } else if number > 100 {
                res += &format!(" {}, ", MAGNITUDE[magnitude as usize]);
            } else {
                res += &format!(" {} and ", MAGNITUDE[magnitude as usize]);
            }
        }
    }

    res
}

fn get_str_length(num: i32) -> i32 {
    wordify(num.into()).len().try_into().unwrap()
}

fn main() {
    let mut data = Vec::new();
    let height = 500i32;
    let width = 500i32;

    let mut longest_representation_num = 0i32;
    let mut longest_representation_len = 0i32;

    for y in 0..height {
        for x in 0..width {
            let pixel = get_str_length(x + y);

            if pixel > longest_representation_len {
                longest_representation_len = pixel;
                longest_representation_num = x + y;
            }

            let magnitude = u8::try_from(pixel).unwrap_or(u8::MAX);

            data.push(RGBA {
                red: magnitude * 8,
                blue: magnitude * 8,
                green: magnitude * 8,
                alpha: 255,
            })
        }
    }

    println!(
        "longest value: {:?} = {:?} len={:?}",
        longest_representation_num,
        wordify(longest_representation_num.into()),
        longest_representation_len
    );

    let image_data = png::create_image(data, width, height);

    let file_path = "./images/string-representation.png";

    let mut png_file = File::create(file_path)
        .unwrap_or_else(|_| panic!("Could not create/write to `{}`", file_path));

    png_file
        .write_all(&image_data[..])
        .unwrap_or_else(|_| panic!("Could not write to `{}`", file_path));
}
