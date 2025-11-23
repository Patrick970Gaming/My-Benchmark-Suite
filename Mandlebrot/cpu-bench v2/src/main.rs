use image;
use rayon::prelude::*;
use std::time::Instant;

const WIDTH: u32 = 30_720;
const HEIGHT: u32 = 17_280;
const MAX_ITER: u64 = 500;
const ARR_LEN: usize = (WIDTH * HEIGHT) as usize;

fn main() {
    println!("Width: {}, Height: {}", WIDTH, HEIGHT);
    let save_image = true;
    let mut multi_string = String::new();

    println!("Multithread, (y or n): ");
    std::io::stdin().read_line(&mut multi_string).unwrap();
    let multi_string = multi_string.trim_end();

    if multi_string == "n" {
        let mut buffer_single: Vec<u64> = vec![0; ARR_LEN];
        let mut img = image::RgbImage::new(WIDTH, HEIGHT);

        for i in 0..ARR_LEN {
            buffer_single[i] = i as u64;
        }

        let start2 = Instant::now();

        fn compute_brot(pixel: &u64) -> u64 {
            let i = pixel.clone();
            let c = find_x(i as u64);
            let r = find_y(i as u64);
            let x0 = ((c as f64) / (WIDTH as f64)) * 3.5 - 2.5;
            let y0 = ((r as f64) / (HEIGHT as f64)) * 2.0 - 1.0;
            let mut x1 = 0.0;
            let mut y1 = 0.0;
            let mut iteration: u64 = 0;
            
            while x1 * x1 + y1 * y1 <= 4.0 && iteration < MAX_ITER {
                let xtemp = x1 * x1 - y1 * y1 + x0;
                y1 = 2.0 * x1 * y1 + y0;
                x1 = xtemp;
                iteration = iteration + 1;
            }

            let rgb: f64 = ((iteration as f64 + 1.0) / (MAX_ITER as f64 + 1.0) as f64).powf(0.2);
            return (rgb * 16777215.0) as u64;
        }

        // run computation
        let pixels: Vec<u64> = buffer_single.iter().map(|x| {
            compute_brot(x)
        }).collect();
        
        let elapsed = start2.elapsed();
        println!("Time elapsed: {:.4?} seconds", elapsed);

        println!("Outputting Image.");
        for i in 0..ARR_LEN {
            let x = find_x(i as u64);
            let y = find_y(i as u64);

            img.put_pixel(x.try_into().unwrap(), y.try_into().unwrap(), get_color_v2(pixels[i]));
        }
        
        if save_image {
            let fname = "mandelbrot.png";
            //img.save_with_format(Path::new("/tmp").join(fname), image::ImageFormat::Png).unwrap();
            let _result = img.save(fname);
        }
    }
    if multi_string == "y" {
        let mut buffer2: Vec<u64> = vec![0; ARR_LEN];
        let mut img = image::RgbImage::new(WIDTH, HEIGHT);

        for i in 0..ARR_LEN {
            buffer2[i] = i as u64;
        }

        let start2 = Instant::now();

        fn compute_brot(pixel: &u64) -> u64 {
            let i = pixel.clone();
            let c = find_x(i as u64);
            let r = find_y(i as u64);
            let x0 = ((c as f64) / (WIDTH as f64)) * 3.5 - 2.5;
            let y0 = ((r as f64) / (HEIGHT as f64)) * 2.0 - 1.0;
            let mut x1 = 0.0;
            let mut y1 = 0.0;
            let mut iteration: u64 = 0;
            
            while x1 * x1 + y1 * y1 <= 4.0 && iteration < MAX_ITER {
                let xtemp = x1 * x1 - y1 * y1 + x0;
                y1 = 2.0 * x1 * y1 + y0;
                x1 = xtemp;
                iteration = iteration + 1;
            }

            let rgb: f64 = ((iteration as f64 + 1.0) / (MAX_ITER as f64 + 1.0) as f64).powf(0.2);
            return (rgb * 16777215.0) as u64;
        }

        // run computation
        let pixels: Vec<u64> = buffer2.par_iter().map(|x| {
            compute_brot(x)
        }).collect();
        
        let elapsed = start2.elapsed();
        println!("Time elapsed: {:.4?} seconds", elapsed);

        println!("Outputting Image.");
        for i in 0..ARR_LEN {
            let x = find_x(i as u64);
            let y = find_y(i as u64);

            img.put_pixel(x.try_into().unwrap(), y.try_into().unwrap(), get_color_v2(pixels[i]));
        }
        
        if save_image {
            let fname = "mandelbrot.png";
            let _result = img.save(fname);
        }
    }
 
}

fn get_color_v2(colour: u64) -> image::Rgb<u8> {
    let red: u8 = ((colour & 16711680) >> 16).try_into().unwrap();
    let green: u8 = ((colour & 65280) >> 8).try_into().unwrap();
    let blue: u8 = (colour & 255).try_into().unwrap();

    return image::Rgb([red, green, blue]);
}
fn find_x(index: u64) -> usize {
    return (index as f64 % WIDTH as f64).floor() as usize
}

fn find_y(index: u64) -> usize {
    return (index as f64 / WIDTH as f64).floor() as usize
}
