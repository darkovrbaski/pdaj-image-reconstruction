// Darko Vrbaski E2 145-2023
pub mod cmp_img;
pub mod img;
pub mod menu;
pub mod mse_alg;

use image::DynamicImage;
use rayon::prelude::*;
use show_image::{create_window, WindowProxy};

#[allow(unused_imports)]
use crate::cmp_img::*;
use crate::img::*;
use crate::menu::*;
use crate::mse_alg::*;

#[show_image::main]
fn main() {
    let selected_option = select_image();
    let selected_image = match selected_option {
        Some(index) => {
            let selected_image = [
                "image1.jpg",
                "image1-1.jpg",
                "image2.jpg",
                "image2-1.jpg",
                "image3.jpg",
                "image4.jpg",
                "image5.jpg",
            ][index];
            println!("You selected: {}", selected_image);
            idx_to_image_path(index)
        }
        None => {
            println!("No image selected.");
            return;
        }
    };

    let org_image = load_image(selected_image.0);

    let mut image_pieces = load_images_from_folder(selected_image.1);
    image_pieces.retain(|image| image.width() > 5 && image.height() > 5);
    println!("image_pieces: {}", image_pieces.len());

    let window = create_window("image", Default::default()).unwrap();

    let time = std::time::Instant::now();
    let result_image = reconstruct_image(&org_image, &mut image_pieces, &window);
    println!("Time: {} seconds", time.elapsed().as_secs_f32());

    println!("Close the window or press ESC to exit.");
    display_image(&result_image, &window, true);
}

fn reconstruct_image(
    org_image: &DynamicImage,
    image_pieces: &mut Vec<DynamicImage>,
    window: &WindowProxy,
) -> DynamicImage {
    let mut result_image = blank_image(&org_image);

    let mut current_x = 0;
    let mut current_y = 0;

    while image_pieces.len() > 0 {
        let (match_index, match_piece, _) = image_pieces
            .par_iter()
            .enumerate()
            .map(|(index, image_piece)| {
                let sub_img = crop_image(
                    &org_image,
                    current_x,
                    current_y,
                    image_piece.width(),
                    image_piece.height(),
                );
                let calc_score = parallel_mse(&sub_img, &image_piece);
                (index, image_piece.clone(), calc_score)
            })
            .reduce(
                || (0, DynamicImage::new_rgb8(0, 0), f64::MAX),
                |(index1, piece1, score1), (index2, piece2, score2)| {
                    if score1 < score2 {
                        (index1, piece1, score1)
                    } else {
                        (index2, piece2, score2)
                    }
                },
            );

        place_image(&mut result_image, &match_piece, current_x, current_y);
        display_image(&result_image, &window, false);
        current_x += match_piece.width();
        image_pieces.remove(match_index);

        if current_x >= result_image.width() - 5 {
            current_x = 0;
            current_y += match_piece.height();
        }
    }

    result_image
}
