mod cli;
mod my_image;

use image::DynamicImage;
use my_image::{draw, load_image, resize};

fn main() {
    // load image from disk
    let args = cli::parse();
    let img = load_image(&args.file);

    // resize image to terminal width
    let (new_width, new_height) = scale_terminal(img.clone());
    let resized = resize(img, new_width, new_height);

    // draw image to terminal
    draw(resized);
}

fn scale_terminal(img: DynamicImage) -> (u32, u32) {
    // get terminal size
    let (term_w, term_h) = crossterm::terminal::size().unwrap();

    // calculate image aspect ratio
    let image_aspect = img.height() as f32 / img.width() as f32;
    let font_aspect = 0.5;

    // calculate new image size
    let mut new_width: f32 = term_w as f32;
    let mut new_height: f32 = img.height() as f32 / image_aspect * font_aspect;

    // adjust for retained aspect ratio
    if new_height > term_h as f32 {
        new_height = term_h as f32;
        new_width = new_height * image_aspect / font_aspect;
    }

    (new_width as u32, new_height as u32)
}
