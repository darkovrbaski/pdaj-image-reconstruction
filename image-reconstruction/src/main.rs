use image::{DynamicImage, GenericImage, GenericImageView};
use image_compare::Algorithm;
use show_image::{create_window, event, ImageInfo, ImageView};
use std::{
    fs::{self, File},
    io::{BufReader, Read, Seek, SeekFrom},
    thread,
    thread::available_parallelism,
};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pieces_folder_path = "../examples/slika 2 -1/";
    let mut image_pieces = load_images_from_folder(pieces_folder_path);
    image_pieces.retain(|image| image.width() > 5 && image.height() > 5);
    println!("image_pieces: {}", image_pieces.len());

    let org_image = load_image("../examples/picture2.jpg");
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
            let calc_score = parallel_mse(&sub_img, &image_piece);

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

fn parallel_mse(reference_image: &DynamicImage, test_image: &DynamicImage) -> f64 {
    if reference_image.dimensions() != test_image.dimensions() {
        return 0.0;
    }

    let (width, height) = test_image.dimensions();
    let num_threads = available_parallelism().unwrap().get() as u32;

    let chunk_size = height / num_threads;

    let handles: Vec<_> = (0..num_threads)
        .into_iter()
        .map(|i| {
            let start_y = i * chunk_size;
            let end_y = if i == num_threads - 1 {
                height
            } else {
                (i + 1) * chunk_size
            };

            let image1 = reference_image.clone();
            let image2 = test_image.clone();

            thread::spawn(move || calculate_mse(&image1, &image2, start_y, end_y))
        })
        .collect();

    let mse_sum: f64 = handles
        .into_iter()
        .map(|handel| handel.join().unwrap())
        .sum();
    let mse = mse_sum / (width * height) as f64;
    mse
}

#[allow(dead_code)]
fn calculate_mse(
    reference_image: &DynamicImage,
    test_image: &DynamicImage,
    start_y: u32,
    end_y: u32,
) -> f64 {
    let (width, _) = test_image.dimensions();

    let mut sum_squared_diff = 0.0;
    for y in start_y..end_y {
        for x in 0..width {
            let pixel1 = reference_image.to_rgb8().get_pixel(x, y).0;
            let pixel2 = test_image.to_rgb8().get_pixel(x, y).0;

            sum_squared_diff += ((pixel1[0] as f64 - pixel2[0] as f64).powi(2)
                + (pixel1[1] as f64 - pixel2[1] as f64).powi(2)
                + (pixel1[2] as f64 - pixel2[2] as f64).powi(2))
                / 3.0;
        }
    }

    // Calculate the mean squared error
    sum_squared_diff
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

            sum_squared_diff += ((pixel1[0] as f64 - pixel2[0] as f64).powi(2)
                + (pixel1[1] as f64 - pixel2[1] as f64).powi(2)
                + (pixel1[2] as f64 - pixel2[2] as f64).powi(2))
                / 3.0;
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
fn compare_images_hybrid(reference_image: &DynamicImage, test_image: &DynamicImage) -> f64 {
    let score =
        image_compare::rgb_hybrid_compare(&reference_image.to_rgb8(), &test_image.to_rgb8())
            .expect("Error: Images had different dimensions")
            .score;
    println!("s: {score}");
    score
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
    display_image(&reference_image);
    display_image(&test_image);

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
