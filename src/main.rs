extern crate rand;

use rand::Rng;

use std::io;
use std::cmp::Ordering;

const GUESS_RANGE_BOTTOM: u32	= 0;
const GUESS_RANGE_TOP: u32		= 100;

fn main() {
	// gen_range generates within [min, max) half-interval, so we add one to make it more user-friendly between min and max
	let secret_number = rand::thread_rng().gen_range(GUESS_RANGE_BOTTOM, GUESS_RANGE_TOP) + 1;

	println!("Guess the number between {} and {}!", GUESS_RANGE_BOTTOM, GUESS_RANGE_TOP);

	loop {
		// Variables are immutable by default
		let mut guess = String::new();

		// References are also immutable by default
		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");

		// Shadow the guess variable
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