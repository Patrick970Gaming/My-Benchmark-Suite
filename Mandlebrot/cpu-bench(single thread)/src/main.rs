use rayon::prelude::*;
use image;
use std::format;
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;
use std::collections::HashMap;

const WIDTH: u32 = 14000;
const HEIGHT: u32 = 8000;
const MAX_ITER: u32 = 500;

fn main() {
    //let mut numbers = HashMap::new();

    let save_image = true;
    let mut buffer: Vec<u32> = vec![0; WIDTH as usize * HEIGHT as usize];
    let mut img = image::RgbImage::new(WIDTH, HEIGHT);

    for c in 0..WIDTH as u32 {
        let x0 = ((c as f32) / (WIDTH as f32)) * 3.5 - 2.5;
        for r in 0..HEIGHT as u32 {
            let y0 = ((r as f32) / (HEIGHT as f32)) * 2.0 - 1.0;
            let mut x = 0.0;
            let mut y = 0.0;
            let mut iteration: u32 = 0;
            while x * x + y * y <= 4.0 && iteration < MAX_ITER {
                let xtemp = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                iteration = iteration + 1;
            }
            let rgb: f32 = ((iteration as f32 + 1.0)/ (MAX_ITER as f32 + 1.0) as f32).powf(0.2);
            img.put_pixel(c, r, get_colorV2((rgb * 16777215.0) as u32));
            let a = get_1d_index(r, c);
            buffer[a as usize] = (rgb * 16777215.0) as u32;
        }
    }

    if save_image {
        let fname = format!("mandelbrot_{}.png", "naive");
        //img.save_with_format(Path::new("/tmp").join(fname), image::ImageFormat::Png).unwrap();
        img.save(fname);
    }

    //println!("{:?}", numbers)
}

fn get_1d_index(row: u32, col: u32) -> u32 {
    return col + row * WIDTH as u32
}

fn get_color(i: u32, max_iterations: u32) -> image::Rgb<u8> {
    if i > max_iterations {
        return image::Rgb([255, 255, 255]);
    }
    if max_iterations == 255 {
        let idx = i as u8;
        return image::Rgb([idx, idx, idx]);
    }
    let idx = (((i as f32) / (max_iterations as f32)) * 255.0).round() as u8;
    return image::Rgb([idx, idx, idx]);
}

fn get_colorV2(colour: u32) -> image::Rgb<u8> {
    let red: u8 = ((colour & 16711680) >> 16).try_into().unwrap();
    let green: u8 = ((colour & 65280) >> 8).try_into().unwrap();
    let blue: u8 = ((colour & 255)).try_into().unwrap();

    return image::Rgb([red, green, blue]);
}