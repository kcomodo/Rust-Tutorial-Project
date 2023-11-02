use std::io;
use rand::Rng;

fn main() {
    // Generate two random numbers between 1 and 10.
    let mut rng = rand::thread_rng();
    let num1: u32 = rng.gen_range(1..11);
    let num2: u32 = rng.gen_range(1..11);

    // Calculate the correct answer.
    let correct_answer = num1 * num2;

    println!("Welcome to the Random Multiplication Game!");
    println!("I will give you two random numbers to multiply. Try to guess the result.");

    println!("First random number: {}", num1);
    println!("Second random number: {}", num2);

    // Ask the user for their guess.
    let mut user_guess = String::new();
    println!("Enter your guess: ");
    io::stdin()
        .read_line(&mut user_guess)
        .expect("Failed to read line");

    // Convert the user's input to an unsigned integer.
    let user_guess: Result<u32, _> = user_guess.trim().parse();

    // Check if the user's guess matches the correct answer.
    match user_guess {
        Ok(guess) => {
            if guess == correct_answer {
                println!("Congratulations! Your guess is correct.");
            } else {
                println!("Sorry, your guess is incorrect. The correct answer is {}.", correct_answer);
            }
        }
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
        }
    }
}
