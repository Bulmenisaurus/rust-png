pub use png;

use png::RGBA;

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
    png::create_image(data, width, height);
}
