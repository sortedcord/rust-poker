use rand::prelude::*;
use std::{io, vec};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum CardSuit {
    Diamond,
    Heart,
    Club,
    Spade,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Card {
    number: i32,
    suit: CardSuit,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum PlayerType {
    Human,
    Cpu,
}

#[derive(PartialEq, Eq, Clone)]
struct Player {
    name: String,
    player_type: PlayerType,
    score: i32,
    cards: Vec<Card>,
}

fn convert_number(card_number: &i32) -> String {
    match card_number {
        9 => String::from("Jack"),
        10 => String::from("Queen"),
        11 => String::from("King"),
        12 => String::from("Ace"),
        _ => (card_number + 2).to_string(),
    }
}

fn read_int() -> i32 {
    let mut _input_string = String::new();
    io::stdin()
        .read_line(&mut _input_string)
        .expect("Could not get input");
    _input_string.trim().parse().unwrap()
}

fn generate_card(seen_cards: &mut Vec<Card>) -> Card {
    let mut rng = rand::rng();

    loop {
        let selected_suit = [
            CardSuit::Diamond,
            CardSuit::Heart,
            CardSuit::Club,
            CardSuit::Spade,
        ]
        .choose(&mut rng)
        .unwrap();

        let new_card = Card {
            number: rng.random_range(0..=12),
            suit: *selected_suit,
        };

        if seen_cards.contains(&new_card) {
            continue;
        } else {
            seen_cards.push(new_card);
            break new_card;
        }
    }
}

fn display_card(card: &Card) {
    println!("{} of {:#?}", convert_number(&card.number), card.suit);
}

fn action_fold(player: &mut Player) {
    player.cards.clear();
    println!("{name} Folds their turn", name = player.name);
}

fn action_bet(player: &mut Player, pool: &mut i32, current_bet: &mut i32) {
    let mut rng = rand::rng();
    let amount: i32 = match player.player_type {
        PlayerType::Human => {
            println!("You have {} chips", player.score);
            println!("Enter your bet amount: ");
            read_int()
        }
        PlayerType::Cpu => rng.random_range(1..=player.score),
    };

    *pool += amount;
    player.score -= amount;
    *current_bet = amount;

    println!("{name} bets {amount}", name = player.name);
}

fn main() {
    let mut rng = rand::rng();
    let mut community_cards: [Option<Card>; 6] = [None; 6];
    let mut pot = 0;

    let mut user = Player {
        name: String::from("sortedcord"),
        player_type: PlayerType::Human,
        score: 50,
        cards: vec![],
    };

    let mut cpu: Player = Player {
        name: String::from("computer"),
        player_type: PlayerType::Cpu,
        score: 50,
        cards: vec![],
    };

    // Round starts
    loop {
        let mut seen_cards: Vec<Card> = Vec::new();

        //Blind selection
        let big_blind: &Player = [&user, &cpu].choose(&mut rng).unwrap();
        println!("Selected bigblind as {name} ", name = big_blind.name);

        // Put money in the pot
        if *big_blind == user {
            let bet = rng.random_range(25..=45);
            pot += bet;
            cpu.score -= pot;

            println!("CPU bet {pot} ");

            pot = loop {
                println!("Enter your big blind: ");
                let bet: i32 = read_int();

                if bet > user.score {
                    println!("Needs to be less or equal to your current holdings, try again!");
                    continue;
                }

                if bet <= pot {
                    println!("Needs to be greater than the small blind");
                    continue;
                }

                user.score -= bet;
                pot += bet;
                break pot;
            };
        } else {
            println!("Enter your small blind: ");
            let bet: i32 = read_int();
            user.score -= bet;
            pot += bet;
            let bet = rng.random_range(pot..=50);
            println!("CPU sets big blind as: {bet}");
            cpu.score -= bet;
            pot += bet;
        }

        println!("Pool size is now: {pot}");

        // Deal hole cards
        for player in [&mut user, &mut cpu] {
            player.cards.push(generate_card(&mut seen_cards));
        }

        println!("Your Hand: ");

        for card in &user.cards {
            display_card(card);
        }

        // Start preflop betting round
        let mut current_bet = 0;
        loop {
            println!("Enter the action (bet, fold): ");
            let mut user_action = String::new();
            io::stdin()
                .read_line(&mut user_action)
                .expect("Wrong input");

            match user_action.trim() {
                "bet" => action_bet(&mut user, &mut pot, &mut current_bet),
                "fold" => action_fold(&mut user),
                _ => {
                    println!("Not a valid action, try again");
                    continue;
                }
            };

            break;
        }

        // CPU plays
        let cpu_action = ["bet", "fold"].choose(&mut rng).unwrap();

        match *cpu_action {
            "bet" => action_bet(&mut cpu, &mut pot, &mut current_bet),
            "fold" => action_fold(&mut cpu),
            _ => panic!("CPU performed invalid action"),
        };

        //Dealer reveal the cards
        //
        println!("Dealer dealt: ");

        for slot in community_cards[..3].iter_mut() {
            let n_card = generate_card(&mut seen_cards);
            display_card(&n_card);
            *slot = Some(n_card);
        }
    }
}
