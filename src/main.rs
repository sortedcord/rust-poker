use rand::Rng;
// use std::cmp::Ordering;
use std::io;

fn main() {
    // println!("Guess the number");
    // println!("Generating secret number");
    //
    // let secret_number = rand::thread_rng().gen_range(1..=100);
    //
    // loop {
    //     println!("Please input your guess");
    //     let mut guess = String::new();
    //
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //
    //     println!("You guessed: {guess}");
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }
    //
    let mut starting_balance = 50;

    loop {
        let user_card = rand::thread_rng().gen_range(1..=13);
        let cpu_card = rand::thread_rng().gen_range(1..=13);

        println!("Your card: {user_card}");
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
