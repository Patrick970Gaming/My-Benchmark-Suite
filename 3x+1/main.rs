use std::time::Instant;
use std::thread;
use std::sync::{Arc, Mutex};

fn solve(mut num: u64) {
	while num != 1 {
		if (num % 2) == 0 {
			num = num / 2;
		}
		
		else {
			num = (num * 3) + 1;
		}
	}
}

fn main() {
    let mut counter: u64 = 1;
    let mut end_num_string = String::new();
    let mut multi_string = String::new();
    let mut cores_string = String::new();
    
    println!("Enter End number: ");
    std::io::stdin().read_line(&mut end_num_string).unwrap();

    println!("Multithread, (y or n): ");
    std::io::stdin().read_line(&mut multi_string).unwrap();

    let multi_string = multi_string.trim_end();

    let end_num: u64 = end_num_string
    .trim_end()
    .parse()
    .expect("Wrong number format!");

    if multi_string == "n" {
        println!("Single threaded, up to {}", end_num);
        let start = Instant::now();
    
        while counter < end_num + 1 {
            solve(counter);
            counter += 1;
        }
    
        let elapsed = start.elapsed();
        println!("Time elapsed: {:.2?} seconds", elapsed);
    }
    else if multi_string == "y" {
        println!("Enter number of threads: ");
        std::io::stdin().read_line(&mut cores_string).unwrap();

        let cores: u64 = cores_string
        .trim_end()
        .parse()
        .expect("Wrong number format!");

        println!("Multithreaded for {}, up to {}", cores, end_num);

        let mut handles = vec![];

        for i in 0..cores {
            let thread_num = Arc::new(Mutex::new(i));
            let thread_num = Arc::clone(&thread_num);

            let cores_thread = Arc::new(Mutex::new(cores));
            let cores_thread = Arc::clone(&cores_thread);
            
            let end_num_thread = Arc::new(Mutex::new(end_num));
            let end_num_thread = Arc::clone(&end_num_thread);

            let handle = thread::spawn( move || {
                let thread_num = *thread_num.lock().unwrap();
                let cores = *cores_thread.lock().unwrap();
                let total_nums = *end_num_thread.lock().unwrap();

                let nums_to_solve = total_nums / cores;

                let start_num = nums_to_solve * thread_num;
                let end_num = (nums_to_solve * (thread_num + 1)) + 1;

                for i in start_num + 1..end_num {
                    solve(i)
                }
            });
            handles.push(handle);
        }

        let start = Instant::now();

        for handle in handles {
            handle.join().unwrap();
        }

        let elapsed = start.elapsed();
        println!("Time elapsed: {:.2?} seconds", elapsed);
        return
    } else {
        println!("Input correct option dummy")
    }
}
