use image::{GenericImageView, ImageReader};

const IMAGE_SRC: &str = "./inputs/shawn.png";
const ASCII_RAMP: &str = ".:-=+*#%@";

fn main() {
    let img = ImageReader::open(IMAGE_SRC).unwrap().decode().unwrap();
    let _height = img.height();
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

// Gemini: map a lightness value to an ASCII character
fn map_to_ascii(lightness: u8) -> char {
    // Ordered from darkest (space) to lightest (density)

    // Scale 0-255 to 0-(ramp_length - 1)
    let index = (lightness as f32 / 255.0 * (ASCII_RAMP.len() - 1) as f32).round() as usize;

    ASCII_RAMP.as_bytes()[index] as char
}
