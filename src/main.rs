extern crate rand;

use rand::Rng;

use std::io;
use std::cmp::Ordering;

fn main() {
	let min = 0;
	let max = 100;

	let secret_number = rand::thread_rng().gen_range(min + 1, max + 1);

	println!("Guess the number between {} and {}!", min, max);

	loop {
		// Variables are immutable by default
		let mut guess = String::new();

		// References are also immutable by default
		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(err) => {
				println!("{}", err);
				continue;
			}
		};

		println!("You guessed: {}", guess);

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}