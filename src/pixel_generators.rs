use image::Rgba;
use rand::Rng;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum PixelAddChoice {
    RandomPixel,
    RandomRedShade,
    RandomBlueShade,
    RandomGreenShade,
    Black
}

pub fn get_random_pixel() -> Rgba<u8> {
    let mut rng = rand::thread_rng();
    Rgba([
        rng.gen_range(0..=255) as u8,
        rng.gen_range(0..=255) as u8,
        rng.gen_range(0..=255) as u8,
        255]
    )
}

pub fn get_random_red_shade() -> Rgba<u8> {
    let mut rng = rand::thread_rng();
    Rgba([
        rng.gen_range(0..=255) as u8,
        0_u8,
        0_u8,
        255]
    )
}

pub fn get_random_blue_shade() -> Rgba<u8> {
    let mut rng = rand::thread_rng();
    Rgba([
        0_u8,
        0_u8,
        rng.gen_range(0..=255) as u8,
        255]
    )
}

pub fn get_random_green_shade() -> Rgba<u8> {
    let mut rng = rand::thread_rng();
    Rgba([
        0_u8,
        rng.gen_range(0..=255) as u8,
        0_u8,
        255]
    )
}

pub fn get_black() -> Rgba<u8> {
    Rgba([
        0_u8,
        0_u8,
        0_u8,
        255]
    )
}