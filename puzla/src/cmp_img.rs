use image::{DynamicImage, GenericImageView};
use image_compare::Algorithm;

#[allow(dead_code)]
pub(crate) fn compare_images_mse(reference_image: &DynamicImage, test_image: &DynamicImage) -> f64 {
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
pub(crate) fn compare_images_px(reference_image: &DynamicImage, test_image: &DynamicImage) -> bool {
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
pub(crate) fn compare_images_hybrid(
    reference_image: &DynamicImage,
    test_image: &DynamicImage,
) -> f64 {
    let score =
        image_compare::rgb_hybrid_compare(&reference_image.to_rgb8(), &test_image.to_rgb8())
            .expect("Error: Images had different dimensions")
            .score;
    println!("s: {score}");
    score
}

#[allow(dead_code)]
pub(crate) fn compare_images_luma(
    reference_image: &DynamicImage,
    test_image: &DynamicImage,
) -> f64 {
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
pub(crate) fn compare_images_rgb(reference_image: &DynamicImage, test_image: &DynamicImage) -> f64 {
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
pub(crate) fn compare_images_his(reference_image: &DynamicImage, test_image: &DynamicImage) -> f64 {
    let score = image_compare::gray_similarity_histogram(
        image_compare::Metric::ChiSquare,
        &reference_image.to_luma8(),
        &test_image.to_luma8(),
    )
    .unwrap();
    println!("s: {score}");
    score
}
