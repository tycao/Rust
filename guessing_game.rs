extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	// notify user the Game starts...
    println!("Guess the number please!");
	
	// generate a random number
	let secret_number = rand::thread_rng().gen_range(1, 101);
	
	// test code: test what the true secret_number is...
	println!("the secret_number is : {}", secret_number);
	
	// shart a loop untill user guess right
	loop {
		println!("please input your guess..");
		
		// generate a string type mutable value
		let mut guess = String::new();
		
		// assign user's input from terminal to guess(a mutable value), if fail, throw out a panic!
		std::io::stdin().read_line(&mut guess).expect("failed to read line..");
		
		// convert guess' string type to i32 type(32 bit integer)
		//let guess: i32 = guess.trim().parse().expect("please type a number!");
		let guess: i32 = match guess.trim().parse(){
			Ok(num) => num,
			Err(_) => continue,
		};
		
		println!("you guessed: {}", guess);
		
		// compare user's input with the true secret_number
		match guess.cmp(&secret_number)	
		{
			Ordering::Less => println!("Too small!"),
			Ordering::Greater	=> println!("Too big!"),
			Ordering::Equal	=> {
				println!("You win!");
				break;
			}
		};
	}
}
