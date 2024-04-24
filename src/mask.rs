use image::{ImageBuffer, Luma, Rgba};

fn mask_pixel(v: f64, low_threshold: f64, high_threshold: f64, invert_mask: bool) -> u8 {
    if (low_threshold < v && v < high_threshold) ^ invert_mask { 255 } else { 0 }
}

pub fn mask_image<F: Fn(&Rgba<u8>) -> f64>(
    image: &ImageBuffer::<Rgba<u8>, Vec<u8>>,
    low_threshold: f64,
    high_threshold: f64,
    invert_mask: bool,
    mask_function: F
) -> ImageBuffer::<Luma<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();

    ImageBuffer::<Luma<u8>, Vec<u8>>::from_fn(width, height, |x, y| {
        Luma::from([mask_pixel(mask_function(image.get_pixel(x, y)), low_threshold, high_threshold, invert_mask)])
    })
}