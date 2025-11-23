use std::time::Instant;

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
    
    println!("Enter End number: ");
    std::io::stdin().read_line(&mut end_num_string).unwrap();

    let end_num: u64 = end_num_string
    .trim_end()
    .parse()
    .expect("Wrong number format!");

    println!("Single threaded, up to {}", end_num);
	let start = Instant::now();

    while counter < end_num + 1 {
        solve(counter);
        counter += 1;
    }

    let elapsed = start.elapsed();
	println!("Time elapsed: {:.2?} seconds", elapsed);
}
