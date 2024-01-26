use image::{DynamicImage, GenericImage, GenericImageView};
use image_compare::Algorithm;
use show_image::{create_window, event, ImageInfo, ImageView};
use std::{
    fs::{self, File},
    io::{BufReader, Read, Seek, SeekFrom},
};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pieces_folder_path = "../examples/slika 1 - 1/";
    let mut image_pieces = load_images_from_folder(pieces_folder_path);
    image_pieces.retain(|image| image.width() > 5 && image.height() > 5);
    println!("image_pieces: {}", image_pieces.len());

    let org_image = load_image("../examples/picture1.jpg");
    let mut result_image = blank_image(&org_image);

    let mut current_x = 0;
    let mut current_y = 0;

    while image_pieces.len() > 0 {
        let mut match_piece = DynamicImage::new_rgb8(0, 0);
        let mut match_index = 0;
        let mut max_score = f64::MAX;

        for (index, image_piece) in image_pieces.iter().enumerate() {
            let sub_img = crop_image(
                &org_image,
                current_x,
                current_y,
                image_piece.width(),
                image_piece.height(),
            );
            // display_image(&sub_img);
            // display_image(&image_piece);
            let calc_score = compare_images_mse(&sub_img, &image_piece);

            if max_score > calc_score {
                match_piece = image_piece.clone();
                match_index = index;
                max_score = calc_score;
                println!("max_score: {}", max_score);
            }
        }

        place_image(&mut result_image, &match_piece, current_x, current_y);
        current_x += match_piece.width();
        image_pieces.remove(match_index);

        // display_image(&result_image);

        if current_x >= result_image.width() - 5 {
            current_x = 0;
            current_y += match_piece.height();
        }
    }

    display_image(&result_image);

    Ok(())
}

#[allow(dead_code)]
fn compare_images_mse(reference_image: &DynamicImage, test_image: &DynamicImage) -> f64 {
    if reference_image.dimensions() != test_image.dimensions() {
        return 0.0;
    }

    // Get the dimensions of the images
    let (width, height) = test_image.dimensions();

    // Calculate the sum of squared differences
    let mut sum_squared_diff = 0.0;
    for y in 0..height {
        for x in 0..width {
            let pixel1 = reference_image.to_rgb8().get_pixel(x, y).0;
            let pixel2 = test_image.to_rgb8().get_pixel(x, y).0;

            for c in 0..pixel1.len() {
                let diff = f64::from(pixel1[c]) - f64::from(pixel2[c]);
                sum_squared_diff += diff * diff;
            }
        }
    }

    // Calculate the mean squared error
    let mse = sum_squared_diff / (height * width) as f64;

    mse
}

#[allow(dead_code)]
fn compare_images_px(reference_image: &DynamicImage, test_image: &DynamicImage) -> bool {
    if reference_image.dimensions() != test_image.dimensions() {
        return false;
    }

    for y in 0..reference_image.height() {
        for x in 0..reference_image.width() {
            if test_image.to_rgb8().get_pixel(x, y) != reference_image.to_rgb8().get_pixel(x, y) {
                return false;
            }
        }
    }

    true
}

#[allow(dead_code)]
fn compare_images_luma(reference_image: &DynamicImage, test_image: &DynamicImage) -> f64 {
    let score = image_compare::gray_similarity_structure(
        &Algorithm::MSSIMSimple,
        &reference_image.to_luma8(),
        &test_image.to_luma8(),
    )
    .expect("Error: Images had different dimensions")
    .score;
    println!("s: {score}");
    score
}

#[allow(dead_code)]
fn compare_images_rgb(reference_image: &DynamicImage, test_image: &DynamicImage) -> f64 {
    let score = image_compare::rgb_similarity_structure(
        &Algorithm::MSSIMSimple,
        &reference_image.to_rgb8(),
        &test_image.to_rgb8(),
    )
    .expect("Error: Images had different dimensions")
    .score;
    println!("s: {score}");
    score
}

#[allow(dead_code)]
fn compare_images_his(reference_image: &DynamicImage, test_image: &DynamicImage) -> f64 {
    let score = image_compare::gray_similarity_histogram(
        image_compare::Metric::ChiSquare,
        &reference_image.to_luma8(),
        &test_image.to_luma8(),
    )
    .unwrap();
    println!("s: {score}");
    score
}

fn load_images_from_folder(folder_path: &str) -> Vec<image::DynamicImage> {
    let mut images = Vec::new();

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                images.push(load_image(&entry.path().to_str().unwrap()));
            }
        }
    }

    images
}

fn load_image(path: &str) -> image::DynamicImage {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    // Read the beginning of the file to guess the format
    let mut start = [0; 16]; // Adjust size as needed
    reader.read_exact(&mut start).unwrap();
    let format = image::guess_format(&start).unwrap();

    // Seek back to the start of the file
    reader.seek(SeekFrom::Start(0)).unwrap();

    // Load the image with the guessed format
    let image = image::load(reader, format).unwrap();
    image
}

fn place_image(dest: &mut DynamicImage, src: &DynamicImage, x: u32, y: u32) {
    for (src_x, src_y, pixel) in src.pixels() {
        let dest_x = x + src_x;
        let dest_y = y + src_y;
        if dest_x < dest.width() && dest_y < dest.height() {
            dest.put_pixel(dest_x, dest_y, pixel)
        }
    }
}

fn blank_image(image: &DynamicImage) -> DynamicImage {
    DynamicImage::new_rgb8(image.width(), image.height())
}

fn crop_image(
    original: &DynamicImage,
    crop_x: u32,
    crop_y: u32,
    crop_width: u32,
    crop_height: u32,
) -> DynamicImage {
    let cropped_image = original.crop_imm(crop_x, crop_y, crop_width, crop_height);
    cropped_image
}

fn display_image(image: &DynamicImage) {
    let image = ImageView::new(
        ImageInfo::rgb8(image.width(), image.height()),
        image.as_bytes(),
    );

    let window = create_window("image", Default::default()).unwrap();
    let _ = window.set_image("image", image);

    for event in window.event_channel().unwrap() {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
        }
    }
}
