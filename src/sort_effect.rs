use std::time::{Duration, Instant};

use image::{ImageBuffer, Luma, Pixel, Rgba};
use colors_transform::{Color, Rgb};
use rand::Rng;
use rayon::prelude::*;


fn mask_pixel(v: f64, low_threshold: f64, high_threshold: f64, invert_mask: bool) -> u8 {
    if (low_threshold < v && v < high_threshold) ^ invert_mask { 255 } else { 0 }
}

pub(crate) fn mask_image(image: &ImageBuffer::<Rgba<u8>, Vec<u8>>, low_threshold: f64, high_threshold: f64, invert_mask: bool) -> ImageBuffer::<Luma<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();

    ImageBuffer::<Luma<u8>, Vec<u8>>::from_fn(width, height, |x, y| {
        Luma::from([mask_pixel(luminance(image.get_pixel(x, y)), low_threshold, high_threshold, invert_mask)])
    })
}

fn pixel_matrix(image: &ImageBuffer::<Rgba<u8>, Vec<u8>>) -> Vec<Vec<&Rgba<u8>>> {
    image.rows().map(|r| r.collect()).collect()
}

fn luminance(pixel: &Rgba<u8>) -> f64 {
    0.2126 * (pixel.0[0] as f64) + 0.7152 * (pixel.0[1] as f64) + 0.0722 * (pixel.0[2] as f64)
}

fn old_hue(pixel: &Rgba<u8>) -> i16 {
    let red = pixel.0[0] as i16;
    let green = pixel.0[1] as i16;
    let blue = pixel.0[2] as i16;

    let min = blue.min(green.min(red));
    let max = blue.max(green.max(red));

    // println!("{:?}, {:?}, {:?}", min, max, (red,green,blue));

    if min == max {
        return 0;
    }

    let mut hue: i16 = 0;
    if max == red {
        hue = (green - blue) / (max - min);
    } else if max == green {
        hue = 2 + (blue - red) / (max - min);
    } else {
        hue = 4 + (red - green) / (max - min);
    }

    hue = hue * 60;
    if hue < 0 {
        hue = hue + 360;
    }

    // println!("{:?}", hue);

    hue
}

pub(crate) fn hue(pixel: &Rgba<u8>) -> i16 {
    let red = pixel.0[0] as f32;
    let green = pixel.0[1] as f32;
    let blue = pixel.0[2] as f32;

    Rgb::from(red, green, blue).to_hsl().get_hue().round() as i16
}

pub fn process_sorting_effect<F>(image: &ImageBuffer::<Rgba<u8>, Vec<u8>>, mask_image: &ImageBuffer::<Luma<u8>, Vec<u8>>, pixel_add_random_prob: f64, pixel_add_func: F)
                                        -> (ImageBuffer::<Rgba<u8>, Vec<u8>>, Duration)
    where F: Fn(usize, usize, Rgba<u8>) -> Rgba<u8> + Sync + Send
{
    let (width, height) = image.dimensions();

    let mut rows: Vec<Vec<&Rgba<u8>>> = pixel_matrix(&image);

    let start = Instant::now();

    let new_rows: Vec<Vec<Rgba<u8>>> = rows.into_par_iter().enumerate()
        .map(|(y, mut row)| {
            let mut rng = rand::thread_rng();
            let mut re: Vec<(usize, usize, Rgba<u8>)> = row.iter().enumerate()
                .filter(|(x, _)| mask_image.get_pixel(x.clone() as u32, y as u32).0[0] == 255)
                .enumerate()
                .map(|(x, e)| (x, e.0, **e.1))
                .collect();
            let mut r: Vec<Rgba<u8>> = re.into_iter().map(| (x, y, p) | if rng.gen_bool(pixel_add_random_prob) {  pixel_add_func(x, y, p) } else { p }).collect();
            r.par_sort_by_key(|p: &Rgba<u8>| hue(&p));
            r
        })
        .collect();

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    let mut sorted_pixels: Vec<Rgba<u8>> = Vec::with_capacity((width * height) as usize);
    let mut new_rows_iter = new_rows.iter().flatten();
    // let blank = Rgba::from_channels(0_u8, 0_u8, 0_u8, 255_u8);
    for y in 0..height {
        for x in 0..width {
            if mask_image.get_pixel(x, y).0[0] == 255 {
                // if x > (width as f64 * 0.8).round() as u32 {
                //     sorted_pixels.push(Rgba([
                //         rng.gen_range(0..=255) as u8,
                //         rng.gen_range(0..=255) as u8,
                //         rng.gen_range(0..=255) as u8,
                //         255])
                //     );
                //     new_rows_iter.next();
                // } else {
                //
                // }
                sorted_pixels.push(*new_rows_iter.next().unwrap())
            } else {
                sorted_pixels.push(*image.get_pixel(x, y))
                // sorted_pixels.push(&blank)
            }
        }
    }

    (ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(width, height, sorted_pixels.iter().flat_map(|p| p.channels().to_vec()).collect()).unwrap(), duration)
}