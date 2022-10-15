use rand::Rng;
use std::io;

fn main() {
    let mut count: i32 = 0;

    loop {
        println!("Guess the sum!");
        let first_number: i32 = rand::thread_rng().gen_range(0..5000);
        let second_number: i32 = rand::thread_rng().gen_range(0..5000);

        println!("What is {} + {}?", first_number, second_number);

        let mut user_input = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.

        stdin.read_line(&mut user_input).ok();

        if (first_number + second_number).to_string() == user_input.trim() {
            println!("Correct! ✅");
        } else {
            println!("Incorrect! ❌");
        }

        count += 1;

        println!("You have guessed {} times", count);
        println!("=============================");
    }
}
