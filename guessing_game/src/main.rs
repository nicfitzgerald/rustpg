extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the game!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .expect("Failed to read the line!");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("\nYou guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => {
        println!("Too small!\n");
      },
      Ordering::Greater => {
        println!("Too big!\n")
      },
      Ordering::Equal => {
        println!("You win!\n");
        break;
      },
    }
  }
}