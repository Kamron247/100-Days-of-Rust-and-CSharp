use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("!@!@! Guess the number !@!@!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret numer is: {secret_number}");

    // let apples = 5; // immutable
    // let mut bananas = 5; // mutable
    // println!("Var test_a = {apples} but test_b = {bananas}");

    loop{
        println!("Input a number punk!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; //.expect("Please type a number!");

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
