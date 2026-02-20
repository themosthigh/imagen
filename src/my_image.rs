use std::io::{self, Write};

use colored::*;
use image::{DynamicImage, GenericImageView, ImageReader};
use imageproc::gradients::horizontal_sobel;

const ASCII_RAMP: &str = ".:-=+*#%@";

pub fn load_image(path: &str) -> DynamicImage {
    ImageReader::open(path).unwrap().decode().unwrap()
}

// Gemini: map a lightness value to an ASCII character
fn map_to_ascii(lightness: u8) -> char {
    // Scale 0-255 to 0-(ramp_length - 1)
    let index = (lightness as f32 / 255.0 * (ASCII_RAMP.len() - 1) as f32).round() as usize;

    ASCII_RAMP.as_bytes()[index] as char
}

pub fn resize(img: DynamicImage, new_width: u32, new_height: u32) -> DynamicImage {
    img.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3)
}

pub fn draw(img: DynamicImage) {
    let width = img.width();

    // Gemini: calculate horizontal gradients
    let gray = img.to_luma8();
    let gx = horizontal_sobel(&gray);
    let gy = horizontal_sobel(&gray);

    let mut stdout = io::BufWriter::new(io::stdout());

    for (x, y, pixel) in img.pixels() {
        let [r, g, b, _a] = pixel.0;

        // Gemini: calculate gradient magnitude and angle
        let dx = gx.get_pixel(x, y)[0] as f32;
        let dy = gy.get_pixel(x, y)[0] as f32;
        let magnitude = (dx.powi(2) + dy.powi(2)).sqrt();
        let angle = dy.atan2(dx);

        // Gemini: luminance standard formula
        let brightess = (r as f32 * 0.2126 + g as f32 * 0.7152 + b as f32 * 0.0722) as u8;

        let character = if magnitude > 100.0 {
            let deg = angle.to_degrees();
            if (deg > -22.5 && deg <= 22.5) || (deg > 157.5 || deg <= -157.5) {
                '|'
            } else if (deg > 22.5 && deg <= 67.5) || (deg > -157.5 && deg <= -112.5) {
                '/'
            } else if (deg > 67.5 && deg <= 112.5) || (deg > -112.5 && deg <= -67.5) {
                '-'
            } else {
                '\\'
            }
        } else {
            map_to_ascii(brightess)
        };

        write!(stdout, "{}", character.to_string().truecolor(r, g, b)).unwrap();
        if x == width - 1 {
            writeln!(stdout).unwrap();
        }
    }

    stdout.flush().unwrap();
}
