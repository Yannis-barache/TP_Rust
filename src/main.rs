use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, Rgb, RgbImage, RgbaImage};

fn open_image(path : &str) -> DynamicImage {
    let path = format!("./img/{}", path);
    ImageReader::open(path)
        .expect("Impossible d'ouvrir l'image")
        .decode()
        .expect("Impossible de décoder l'image")
}

fn save_image_rgb8(img: &DynamicImage, name: &str) {
    // On récupère le nom de l'image sans l'extension
    let img = img.to_rgb8();
    let path = format!("./img/{}", name);
    img.save(path).expect("Impossible de sauvegarder l'image");

}

fn get_pixel(img: &DynamicImage, x: u32, y: u32) -> [u8; 4] {
    let pixel = img.get_pixel(x, y);
    pixel.0
}



fn alternate_normal_white(img: &DynamicImage)-> RgbImage {
    let mut img = img.to_rgb8();
    for x in 0..img.width() {
        for y in 0..img.height() {
            if x % 2 == 0 && y % 2 == 0 {
                img.put_pixel(x, y, Rgb([255, 255, 255]));
            }
        }
    }
    img
}

fn main() {

    // IUT
    let img_iut = open_image("iut.jpg");
    println!("Image ouverte avec succès, dimensions : {}x{}", img_iut.width(), img_iut.height());

    save_image_rgb8(&img_iut,"iut2.jpg");
    println!("Image sauvegardée avec succès");


    // Image canal alpha
    let img_alpha = open_image("PNG_canal_alpha.png");
    println!("Image ouverte avec succès, dimensions : {}x{}", img_alpha.width(), img_alpha.height());

    save_image_rgb8(&img_alpha,"Question3.png");
    println!("Image sauvegardée avec succès");

    println!("----------Question 4------------");
    // On affiche le pixel en (32, 52)
    let pixel = get_pixel(&img_iut, 32, 52);
    println!("Pixel en (32, 52) : {:?}", pixel);

    println!("----------Question 5------------");
    // On passe un pixel sur 2 en blanc (255, 255, 255)
    let white = alternate_normal_white(&img_iut);
    save_image_rgb8(&DynamicImage::ImageRgb8(white), "Question5.png");
}