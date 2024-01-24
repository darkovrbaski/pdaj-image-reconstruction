use image::{DynamicImage, GenericImage, GenericImageView};
use image_compare::Algorithm;
use show_image::{create_window, ImageInfo, ImageView};
use std::{collections::VecDeque, fs, process::Command};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pieces_folder_path = "../examples/slika 3/";
    let image_pieces = load_images_from_folder(pieces_folder_path);

    let org_image = image::open("../examples/picture3.jpg").unwrap();
    let mut result_image = blank_image(&org_image);

    let mut current_x = 0;
    let mut current_y = 0;

    let mut available_pieces = VecDeque::new();
    available_pieces.extend(image_pieces);

    while let Some(image_piece) = available_pieces.pop_front() {
        let piece_height = image_piece.height();
        let sub_img = crop_image(
            &org_image,
            current_x,
            current_y,
            image_piece.width(),
            image_piece.height(),
        );
        // display_image(&sub_img);
        // display_image(&image_piece);

        if compare_images_px(&sub_img, &image_piece) {
            place_image(&mut result_image, &image_piece, current_x, current_y);
            display_image(&result_image);

            current_x += image_piece.width();
        } else {
            available_pieces.push_back(image_piece);
        }

        if current_x >= result_image.width() - 1 {
            current_x = 0;
            current_y += piece_height;
        }
    }

    display_image(&result_image);

    Ok(())
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

fn load_images_from_folder(folder_path: &str) -> Vec<image::DynamicImage> {
    let mut images = Vec::new();

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(image) = image::open(entry.path()) {
                    images.push(image)
                }
            }
        }
    }

    images
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

fn compare_images_px(reference_image: &DynamicImage, test_image: &DynamicImage) -> bool {
    if reference_image.dimensions() != test_image.dimensions() {
        return false;
    }

    for y in 0..reference_image.height() {
        for x in 0..reference_image.width() {
            if test_image.get_pixel(x, y) != reference_image.get_pixel(x, y) {
                return false;
            }
        }
    }

    true
}

fn compare_images_luma(reference_image: &DynamicImage, test_image: &DynamicImage) -> bool {
    let score = image_compare::gray_similarity_structure(
        &Algorithm::MSSIMSimple,
        &reference_image.to_luma8(),
        &test_image.to_luma8(),
    )
    .expect("Error: Images had different dimensions")
    .score;
    println!("s: {score}");
    score > 0.95
}

fn compare_images_rgb(reference_image: &DynamicImage, test_image: &DynamicImage) -> bool {
    let score = image_compare::rgb_similarity_structure(
        &Algorithm::MSSIMSimple,
        &reference_image.to_rgb8(),
        &test_image.to_rgb8(),
    )
    .expect("Error: Images had different dimensions")
    .score;
    println!("s: {score}");
    score > 0.95
}

fn display_image(image: &DynamicImage) {
    let image = ImageView::new(
        ImageInfo::rgb8(image.width(), image.height()),
        image.as_bytes(),
    );

    let window = create_window("image", Default::default()).unwrap();
    let _ = window.set_image("image", image);

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
