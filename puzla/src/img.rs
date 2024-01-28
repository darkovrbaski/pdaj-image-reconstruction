use image::{DynamicImage, GenericImage, GenericImageView};
use show_image::{event, ImageInfo, ImageView, WindowProxy};
use std::{
    fs::{self, File},
    io::{BufReader, Read, Seek, SeekFrom},
};

pub(crate) fn load_images_from_folder(folder_path: &str) -> Vec<image::DynamicImage> {
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

pub(crate) fn load_image(path: &str) -> image::DynamicImage {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    // Read the beginning of the file to guess the format
    let mut start = [0; 16];
    reader.read_exact(&mut start).unwrap();
    let format = image::guess_format(&start).unwrap();

    // Seek back to the start of the file
    reader.seek(SeekFrom::Start(0)).unwrap();

    // Load the image with the guessed format
    let image = image::load(reader, format).unwrap();
    image
}

pub(crate) fn place_image(dest: &mut DynamicImage, src: &DynamicImage, x: u32, y: u32) {
    for (src_x, src_y, pixel) in src.pixels() {
        let dest_x = x + src_x;
        let dest_y = y + src_y;
        if dest_x < dest.width() && dest_y < dest.height() {
            dest.put_pixel(dest_x, dest_y, pixel)
        }
    }
}

pub(crate) fn blank_image(image: &DynamicImage) -> DynamicImage {
    DynamicImage::new_rgb8(image.width(), image.height())
}

pub(crate) fn crop_image(
    original: &DynamicImage,
    crop_x: u32,
    crop_y: u32,
    crop_width: u32,
    crop_height: u32,
) -> DynamicImage {
    let cropped_image = original.crop_imm(crop_x, crop_y, crop_width, crop_height);
    cropped_image
}

pub(crate) fn display_image(image: &DynamicImage, window: &WindowProxy, wait_for_esc: bool) {
    let image = ImageView::new(
        ImageInfo::rgb8(image.width(), image.height()),
        image.as_bytes(),
    );

    let _ = window.set_image("image", image);

    if wait_for_esc == true {
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
}
