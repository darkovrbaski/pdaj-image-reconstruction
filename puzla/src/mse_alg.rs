use image::{DynamicImage, GenericImageView};
use std::{thread, thread::available_parallelism};

#[allow(dead_code)]
pub(crate) fn parallel_mse(reference_image: &DynamicImage, test_image: &DynamicImage) -> f64 {
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
pub(crate) fn calculate_mse(
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

    sum_squared_diff
}
