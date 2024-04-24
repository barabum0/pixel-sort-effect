use std::time::{Duration, Instant};

use image::{ImageBuffer, Luma, Pixel, Rgba};
use rand::Rng;
use rayon::prelude::*;

fn pixel_matrix(image: &ImageBuffer::<Rgba<u8>, Vec<u8>>) -> Vec<Vec<&Rgba<u8>>> {
    image.rows().map(|r| r.collect()).collect()
}

pub fn process_sorting_effect<
    PA: Fn(usize, usize, Rgba<u8>) -> Rgba<u8> + Sync + Send,
    PF: Fn(&Rgba<u8>) -> i16 + Sync + Send
>(
    image: &ImageBuffer::<Rgba<u8>, Vec<u8>>,
    mask_image: &ImageBuffer::<Luma<u8>, Vec<u8>>,
    pixel_add_random_prob: f64,
    pixel_add_func: PA,
    pixel_sort_key_func: PF
) -> ImageBuffer::<Rgba<u8>, Vec<u8>>
{
    let (width, height) = image.dimensions();

    let mut rows: Vec<Vec<&Rgba<u8>>> = pixel_matrix(&image);

    let new_rows: Vec<Vec<Rgba<u8>>> = rows.into_par_iter().enumerate()
        .map(|(y, mut row)| {
            let mut rng = rand::thread_rng();
            let mut re: Vec<(usize, usize, Rgba<u8>)> = row.iter().enumerate()
                .filter(|(x, _)| mask_image.get_pixel(x.clone() as u32, y as u32).0[0] == 255)
                .enumerate()
                .map(|(x, e)| (x, e.0, **e.1))
                .collect();
            let mut r: Vec<Rgba<u8>> = re.into_iter().map(| (x, y, p) | if rng.gen_bool(pixel_add_random_prob) {  pixel_add_func(x, y, p) } else { p }).collect();
            r.par_sort_by_key(|p: &Rgba<u8>| pixel_sort_key_func(&p));
            r
        })
        .collect();

    let mut sorted_pixels: Vec<Rgba<u8>> = Vec::with_capacity((width * height) as usize);
    let mut new_rows_iter = new_rows.iter().flatten();
    for y in 0..height {
        for x in 0..width {
            if mask_image.get_pixel(x, y).0[0] == 255 {
                sorted_pixels.push(*new_rows_iter.next().unwrap())
            } else {
                sorted_pixels.push(*image.get_pixel(x, y))
            }
        }
    }

    ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(width, height, sorted_pixels.iter().flat_map(|p| p.channels().to_vec()).collect()).unwrap()
}