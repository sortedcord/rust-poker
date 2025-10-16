use rand::Rng;
// use std::cmp::Ordering;
use std::io;

fn main() {
    let mut starting_balance = 50;
    let cards = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
    ];

    loop {
        if starting_balance <= 0 {
            println!("You are out of money! Game over.");
            break;
        } else if starting_balance >= 100 {
            println!("You reached 100! You win!");
            break;
        }
        let user_card = rand::thread_rng().gen_range(0..=12);
        let cpu_card = rand::thread_rng().gen_range(0..=12);

        println!("Your card: {}", cards[user_card]);
        println!("1: I have better cards\n2: I have worse cards");

        let mut user_action = String::new();
        io::stdin()
            .read_line(&mut user_action)
            .expect("Failed to read line");

        let user_action = match user_action.trim() {
            "1" => "better",
            "2" => "worse",
            _ => {
                println!("Invalid input, please enter 1 or 2.");
                continue;
            }
        };

        if user_action == "better" {
            if user_card >= cpu_card {
                starting_balance += 10;
                println!("You win! New balance: {starting_balance}");
            } else {
                starting_balance -= 10;
                println!("You lose! New balance: {starting_balance}");
            }
        } else {
            if user_card <= cpu_card {
                starting_balance += 10;
                println!("You win! New balance: {starting_balance}");
            } else {
                starting_balance -= 10;
                println!("You lose! New balance: {starting_balance}");
            }
        }
    }
}
