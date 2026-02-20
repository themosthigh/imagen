use image::{GenericImageView, ImageReader};

const IMAGE_SRC: &str = "./inputs/shawn.png";

fn main() {
    let img = ImageReader::open(IMAGE_SRC).unwrap().decode().unwrap();

    println!("Image size: {:?}", img.dimensions());
}
