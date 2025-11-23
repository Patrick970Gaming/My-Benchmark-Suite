use rayon::prelude::*;
use image;
use std::time::Instant;
use std::env;

const WIDTH: u32 = 30_720;
const HEIGHT: u32 = 17_280;
const MAX_ITER: u32 = 500;


fn main() {
    println!("Width: {}, Height: {}", WIDTH, HEIGHT);
    let args: Vec<String> = env::args().collect();
    let save_image = true;
    let mut buffer: Vec<[u32; 3]> = vec![[0, 0, 0]; (HEIGHT * WIDTH) as usize];
    let mut img = image::RgbImage::new(WIDTH, HEIGHT);
    
    let mut counter: u32 = 0;
    for col in 0..WIDTH {
        for row in 0..HEIGHT {
            buffer[counter as usize][1] = row;
            buffer[counter as usize][0] = col;
            counter += 1;
        }
    }

    let start = Instant::now();
    if args.len() > 1 {
        if args[1] == "-p" {
            buffer.par_iter_mut().for_each(|p| {
                let c = p[0];
                let r = p[1];
                let x0 = ((c as f32) / (WIDTH as f32)) * 3.5 - 2.5;
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
                p[2] = (rgb * 16777215.0) as u32;
                
            });
        }   
    } else {
        buffer.iter_mut().for_each(|p| {
            let c = p[0];
            let r = p[1];
            let x0 = ((c as f32) / (WIDTH as f32)) * 3.5 - 2.5;
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
            p[2] = (rgb * 16777215.0) as u32;
            
        });
    }
    

    let mut recon: u64 = 0;
    for col in 0..WIDTH {
        for row in 0..HEIGHT {
            img.put_pixel(col, row, get_color_v2(buffer[recon as usize][2]));
            recon += 1;
        }
    }

    let elapsed = start.elapsed();
        println!("Time elapsed: {:.4?} seconds", elapsed);

    if save_image {
        let fname = format!("mandelbrot_{}.png", "naive");
        //img.save_with_format(Path::new("/tmp").join(fname), image::ImageFormat::Png).unwrap();
        let _result = img.save(fname);
    }

    //println!("{:?}", numbers)
}

fn get_color_v2(colour: u32) -> image::Rgb<u8> {
    let red: u8 = ((colour & 16711680) >> 16).try_into().unwrap();
    let green: u8 = ((colour & 65280) >> 8).try_into().unwrap();
    let blue: u8 = ((colour & 255)).try_into().unwrap();

    return image::Rgb([red, green, blue]);
}