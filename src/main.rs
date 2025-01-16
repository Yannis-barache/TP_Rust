use image::{io::Reader as ImageReader, DynamicImage, ImageOutputFormat, RgbImage};
use std::fs::File;
use std::io::{Cursor, Write};

fn open_image() -> DynamicImage {
    ImageReader::open("./img/PNG_canal_alpha.png")
        .expect("Impossible d'ouvrir l'image")
        .decode()
        .expect("Impossible de décoder l'image")
}

fn save_image_rgb8(img: DynamicImage) {
    let img = img.to_rgb8();
    img.save("./img/RGB8_canal_alpha.png").expect("Impossible de sauvegarder l'image");
}



fn main() {
    let img = open_image();
    println!("Image ouverte avec succès, dimensions : {}x{}", img.width(), img.height());

    save_image_rgb8(img);
    println!("Image sauvegardée avec succès");
}