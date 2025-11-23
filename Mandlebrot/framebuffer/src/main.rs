use num::complex::Complex;

use minifb::{Key, Window, WindowOptions};
//use rand::Rng;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const MAX_ITER: u32 = 1000;
fn main() {
    //let mut rng = rand::thread_rng();
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let colours: Vec<u32> = vec![16777215, 16511749, 16737283, 14485767, 15861892, 4653221, 211, 175082, 2078484, 25618, 5647365, 9466170, 12632256, 8421504, 4210752, 0];
    let cpus = num_cpus::get();

    //mandlebrot shit
    let RE_START: i64 = -2;
    let RE_END: i64 = 1;
    let IM_START: i64 = -1;
    let IM_END: i64 = 1;
    
    let grid_size: [u32; 2] = match cpus {
        1 => [1, 1],
        2 => [1, 2],
        3 => [1, 3],
        4 => [2, 2],
        6 => [2, 3],
        8 => [2, 4],
        10 => [2, 5],
        12 => [3, 4],
        14 => [2, 7],
        16 => [4, 4],
        _ => {
            if cpus > 16 {
                [4, 4]
            }
            else {
                [1, 1]
            }
        }
    };

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

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
            int_colour((rgb * 16777215.0) as u32);
            let a = get_1d_index(r, c);
            buffer[a as usize] = (rgb * 16777215.0) as u32;
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        /*  
        let mut counterx: u32 = 0;
        let mut countery: u32 = 0;
        for i in buffer.iter_mut() {
            let colour_select = map(counterx, 0, WIDTH as u32, 0, grid_size[0]) + (map(countery, 0, HEIGHT as u32, 0, grid_size[1]) * grid_size[1]);
            *i = colours[colour_select as usize]; // write something more funny here!
            counterx += 1;
            if counterx == WIDTH as u32{
                counterx = 0;
                countery += 1
            }
        }
        */
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
    //let a = rgb_int(255, 64, 255);
    //println!("{a:b}");
}

fn map(x:u32, in_min:u32, in_max:u32, out_min:u32, out_max:u32) -> u32 {
    return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

fn rgb_int(red: u32, green: u32, blue: u32) -> u32{
    return (((red << 8) + green) << 8) + blue;
}

fn get_1d_index(row: u32, col: u32) -> u32 {
    return col + row * WIDTH as u32
}

fn power_colour(distance: f32, exp: f32) -> f32 {
    let colour = distance.powf(exp);
    return colour;
}

fn int_colour(colour: u32) {
    let red: u8 = ((colour & 16711680) >> 16).try_into().unwrap();
    let green: u8 = ((colour & 65280) >> 8).try_into().unwrap();
    let blue: u8 = ((colour & 255)).try_into().unwrap();
    println!("{}, {}, {}", red, green, blue);
}
