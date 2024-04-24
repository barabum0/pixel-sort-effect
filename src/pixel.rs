use colors_transform::{Color, Rgb};
use image::Rgba;


#[derive(Debug)]
#[derive(PartialEq)]
pub enum PixelSortKeyChoice {
    Hue,
    BrokenHue,
    Luminance,
    Red,
    Green,
    Blue,
    ColorSum
}

pub fn luminance(pixel: &Rgba<u8>) -> f64 {
    0.2126 * (pixel.0[0] as f64) + 0.7152 * (pixel.0[1] as f64) + 0.0722 * (pixel.0[2] as f64)
}

pub fn some_color(pixel: &Rgba<u8>) -> i16 {
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

pub fn hue(pixel: &Rgba<u8>) -> i16 {
    let red = pixel.0[0] as f32;
    let green = pixel.0[1] as f32;
    let blue = pixel.0[2] as f32;

    Rgb::from(red, green, blue).to_hsl().get_hue().round() as i16
}