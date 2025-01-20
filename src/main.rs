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

fn get_pixel(img: &RgbImage, x: u32, y: u32) -> [u8; 3] {
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

fn rgb_distance(c1: Rgb<u8>, c2: Rgb<u8>) -> f32 {
    let r_diff = c2[0] as f32 - c1[0] as f32;
    let g_diff = c2[1] as f32 - c1[1] as f32;
    let b_diff = c2[2] as f32 - c1[2] as f32;
    (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff).sqrt()
}

fn index_image_on_palette(img: &DynamicImage, palette: &[Rgb<u8>]) -> RgbImage {
    let mut img = img.to_rgb8();

    // Si la palette est vide
    if palette.is_empty() {
        println!("La palette est vide. L'image sera la même.");
        return img;
    }
    
    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = get_pixel(&img, x, y);
            let pixel_rgb = Rgb([pixel[0], pixel[1], pixel[2]]);
            
            // On recherche la couleur la plus proche de la palette 
            let mut distance_min = f32::MAX;
            let mut couleur_proche = pixel_rgb;
            
            for &color in palette {
                let distance = rgb_distance(pixel_rgb, color);
                if distance < distance_min {
                    distance_min = distance;
                    couleur_proche = color;
                }
            }
            
            // On remplace le pixel par la couleur la plus proche du pixel auparavant
            img.put_pixel(x, y, couleur_proche);
        }
    }
    
    img
}

/// Modifie l'image en fonction de la luminosité
fn luminosity_based_change(img: &DynamicImage, colors: [Rgb<u8>; 2]) -> RgbImage {
    let mut img = img.to_rgb8();
    for x in 0..img.width() {
        for y in 0..img.height() {
            // Récupération du pixel via la fonction `get_pixel`
            let pixel = get_pixel(&img, x, y);
            // Calcul de la luminosité via `get_luminosite`
            let luminosity = get_luminosite(pixel);
            // Applique les changements selon la luminosité
            if luminosity > 128.0 {
                img.put_pixel(x, y, colors[0]); // Couleur pour luminosité élevée
            } else {
                img.put_pixel(x, y, colors[1]); // Couleur pour luminosité basse
            }
        }
    }
    img
}

fn get_luminosite(pixel : [u8; 3]) -> f32 {
    let r = pixel[0] as f32;
    let g = pixel[1] as f32;
    let b = pixel[2] as f32;
    0.212671 * r + 0.715160 * g + 0.072169 * b
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
    let pixel = get_pixel(&img_iut.to_rgb8(), 32, 52);
    println!("Pixel en (32, 52) : {:?}", pixel);

    println!("----------Question 5------------");
    // On passe un pixel sur 2 en blanc (255, 255, 255)
    let white = alternate_normal_white(&img_iut);
    save_image_rgb8(&DynamicImage::ImageRgb8(white), "Question5.png");

    println!("----------Question 7------------");
    // Si la valeur de luminosité est supérieure à 128, on met le pixel en blanc sinon en noir
    let processed_img = luminosity_based_change(&img_iut, [Rgb([255, 255, 255]), Rgb([0, 0, 0])]);
    save_image_rgb8(&DynamicImage::ImageRgb8(processed_img), "Question7.png");


    println!("----------Question 8------------");
    let processed_img = luminosity_based_change(&img_iut, [Rgb([255, 0, 0]), Rgb([0, 255, 0])]);
    save_image_rgb8(&DynamicImage::ImageRgb8(processed_img), "Question8.png");
  
    println!("----------Question 10------------");
    // Définition de la palette de couleurs 
    let palette = vec![
        Rgb([0, 0, 0]), // Noir
        Rgb([255, 0, 0]), // Rouge
        Rgb([0, 255, 0]), // Vert
        Rgb([0, 0, 255]), // Bleu
        Rgb([255, 255, 0]), // Jaune
        Rgb([255, 255, 255]), // Blanc
        Rgb([255, 0, 255]), // Magenta
        Rgb([0, 255, 255]), // Cyan
    ];

    // Indexation de l'image
    let image_indexe = index_image_on_palette(&img_iut, &palette);
    save_image_rgb8(&DynamicImage::ImageRgb8(image_indexe), "Question10.png");
    println!("Image sauvegardée avec succès");

}