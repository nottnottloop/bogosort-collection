extern crate rand;

use rand::seq::SliceRandom;
use std::time::Instant;

fn main() {
    const LENGTH: u32 = 7;
		const SORT_TIMES: u32 = 1000;

    let mut arr: Vec<u32> = (0..LENGTH).collect();
    let mut rng = rand::thread_rng();

    let now = Instant::now();

    let mut iterations: u32 = 0;

		for i in 0..SORT_TIMES {
			let mut sorted = false;
			arr.shuffle(&mut rng);
			while !sorted {
					iterations += 1;
					let mut count = 0;
					for e in &mut arr {
							// are the numbers sequential?
							if *e == count {
									count += 1;
									if count == LENGTH {
											sorted = true;
											break;
									}
							} else {
									// if not, just shuffle the entire array and try again!
									arr.shuffle(&mut rng);
									break;
							}
					}
			}
		}

    let elapsed_time = now.elapsed();

    println!("\nRust Bogosort");
    println!("Bogosort completed for array length {}", LENGTH);
    println!("Bogosort performed {} times", SORT_TIMES);
    println!("{} iterations performed", iterations);
    println!("It took {} seconds", elapsed_time.as_secs_f64());
    println!("An average of {} seconds per sort", elapsed_time.as_secs_f64() / SORT_TIMES as f64);
}
