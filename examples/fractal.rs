pub use png;

use std::fs::File;
use std::io::prelude::*;

use png::RGBA;

#[derive(Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    fn multiply_by_real(&self, by: f64) -> Complex {
        Complex {
            real: &self.real * by,
            imaginary: &self.imaginary * by,
        }
    }

    fn multiply_by_complex(&self, by: Complex) -> Complex {
        Complex {
            real: (self.real * by.real - self.imaginary * by.imaginary),
            imaginary: (self.real * by.imaginary + self.imaginary * by.real),
        }
    }

    fn add_to_real(&self, to: f64) -> Complex {
        Complex {
            real: self.real + to,
            imaginary: self.imaginary,
        }
    }

    fn magnitude(&self) -> f64 {
        (self.real.powi(2) + self.imaginary.powi(2)).sqrt()
    }
}

fn check_convergence(c: Complex) -> bool {
    c.magnitude() < 2.
}

// computer z^3 + 0.5
fn f(x: Complex) -> Complex {
    x.multiply_by_complex(x)
        .multiply_by_complex(x)
        .add_to_real(0.5)
}

fn get_pixel_color(x: i32, y: i32, width: i32, height: i32) -> RGBA {
    let center_x = 0.0;
    let center_y = 0.0;
    let scale_factor: f64 = 1. / 300.;
    let mut value = Complex {
        real: f64::from(x - width / 2) * scale_factor - center_x,
        imaginary: f64::from(y - height / 2) * scale_factor - center_y,
    };

    let mut failed_convergence_i = 100;
    for i in 0..200 {
        value = f(value);

        if !check_convergence(value) {
            failed_convergence_i = i;
            break;
        }
    }
    let color = 255. / 100. * f64::from(failed_convergence_i);

    let color: u8 = color as u8;

    RGBA {
        red: color,
        blue: color,
        green: color,
        alpha: 255,
    }
}

fn main() {
    let mut data = Vec::new();
    let height = 800;
    let width = 800;

    for y in 0..height {
        for x in 0..width {
            let pixel = get_pixel_color(x, y, width, height);

            data.push(pixel)
        }
    }

    let image_data = png::create_image(data, width, height);

    let file_path = "./images/fractal.png";

    let mut png_file = File::create(file_path)
        .unwrap_or_else(|_| panic!("Could not create/write to `{}`", file_path));

    png_file
        .write_all(&image_data[..])
        .unwrap_or_else(|_| panic!("Could not write to `{}`", file_path));
}
