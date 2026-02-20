use image::{DynamicImage, GenericImageView, ImageReader};

const ASCII_RAMP: &str = ".:-=+*#%@";

pub fn load_image(path: &str) -> DynamicImage {
    ImageReader::open(path).unwrap().decode().unwrap()
}

// Gemini: map a lightness value to an ASCII character
fn map_to_ascii(lightness: u8) -> char {
    // Ordered from darkest (space) to lightest (density)

    // Scale 0-255 to 0-(ramp_length - 1)
    let index = (lightness as f32 / 255.0 * (ASCII_RAMP.len() - 1) as f32).round() as usize;

    ASCII_RAMP.as_bytes()[index] as char
}

pub fn resize(img: DynamicImage, new_width: u32, new_height: u32) -> DynamicImage {
    img.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3)
}

pub fn draw(img: DynamicImage) {
    let width = img.width();
    for (x, _y, pixel) in img.pixels() {
        let [r, g, b, _a] = pixel.0;

        // Gemini: luminance standard formula
        let brightess = (r as f32 * 0.2126 + g as f32 * 0.7152 + b as f32 * 0.0722) as u8;
        let ascii = map_to_ascii(brightess);
        print!("{}", ascii);
        if x == width - 1 {
            println!();
        }
    }
}
