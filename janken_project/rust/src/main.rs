use rand::Rng;
use std::io;

fn main() {
    let choices = ["Rock", "Paper", "Scissors"];
    println!("Enter 0 (Rock), 1 (Paper), 2 (Scissors): ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let user_choice: usize = match input.trim().parse() {
        Ok(num) if num < 3 => num,
        _ => {
            println!("Invalid input. Please enter 0, 1, or 2.");
            return;
        }
    };

    let computer_choice = rand::thread_rng().gen_range(0..3);
    println!("You: {}", choices[user_choice]);
    println!("Computer: {}", choices[computer_choice]);

    if user_choice == computer_choice {
        println!("It's a tie!");
    } else if (user_choice == 0 && computer_choice == 2)
        || (user_choice == 1 && computer_choice == 0)
        || (user_choice == 2 && computer_choice == 1)
    {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}
