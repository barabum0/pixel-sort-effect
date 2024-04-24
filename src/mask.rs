use image::{ImageBuffer, Luma, Rgba};
use rayon::prelude::*;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum MaskFuncChoice {
    Luminance,
    Hue,
    BrokenHue,
    Red,
    Green,
    Blue,
    ColorSum
}

impl MaskFuncChoice {
    pub fn get_range(&self) -> (f64, f64) {
        match self {
            MaskFuncChoice::Luminance => { (0.0, 255.0) }
            MaskFuncChoice::Hue => { (0.0, 360.0) }
            MaskFuncChoice::BrokenHue => { (0.0, 360.0) }
            MaskFuncChoice::Red => { (0.0, 255.0 ) }
            MaskFuncChoice::Green => { (0.0, 255.0) }
            MaskFuncChoice::Blue => { (0.0, 255.0 ) }
            MaskFuncChoice::ColorSum => { (0.0, 765.0) }
        }
    }
}

fn mask_pixel(v: f64, low_threshold: f64, high_threshold: f64, invert_mask: bool) -> u8 {
    if (low_threshold < v && v < high_threshold) ^ invert_mask { 255 } else { 0 }
}

pub fn mask_image<F: Fn(&Rgba<u8>) -> f64 + Sync + Send>(
    image: &ImageBuffer::<Rgba<u8>, Vec<u8>>,
    low_threshold: f64,
    high_threshold: f64,
    invert_mask: bool,
    mask_function: F,
) -> ImageBuffer::<Luma<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();

    let pixels: Vec<u8> = (0..height).into_par_iter().flat_map(
        |y| {
            (0..width).into_iter().map(
                |x| {
                    mask_pixel(mask_function(image.get_pixel(x, y)), low_threshold, high_threshold, invert_mask)
                }
            ).collect::<Vec<u8>>()
        }
    ).collect();

    ImageBuffer::<Luma<u8>, Vec<u8>>::from_vec(width, height, pixels).unwrap()
}