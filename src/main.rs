mod my_image;

use my_image::{draw, load_image, resize};

const IMAGE_SRC: &str = "./inputs/shawn.png";

fn main() {
    // load image from disk
    let img = load_image(IMAGE_SRC);

    // resize image to terminal width
    let (term_w, _term_h) = crossterm::terminal::size().unwrap();
    let resized = resize(img, term_w as u32);

    // draw image to terminal
    draw(resized);
}
