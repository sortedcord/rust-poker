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

#[derive(PartialEq, Eq)]
enum PlayerType {
    Human,
    Cpu,
}

#[derive(PartialEq, Eq)]
struct Player {
    name: String,
    player_type: PlayerType,
    score: i32,
    cards: Vec<Card>,
}

fn convert_number(card_number: i32) -> String {
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

fn main() {
    let mut rng = rand::rng();
    let mut seen_cards: Vec<Card> = Vec::new();
    let mut pool = 0;

    let mut user = Player {
        name: String::from("sortedcord"),
        player_type: PlayerType::Human,
        score: 50,
        cards: vec![
            generate_card(&mut seen_cards),
            generate_card(&mut seen_cards),
        ],
    };

    let mut cpu: Player = Player {
        name: String::from("computer"),
        player_type: PlayerType::Cpu,
        score: 50,
        cards: vec![
            generate_card(&mut seen_cards),
            generate_card(&mut seen_cards),
        ],
    };

    let big_blind: &Player = [&user, &cpu].choose(&mut rng).unwrap();
    println!("Selected bigblind as {name} ", name = big_blind.name);

    // Put money in the pot
    if *big_blind == user {
        let bet = rng.random_range(25..=45);
        pool += bet;
        cpu.score -= pool;

        println!("CPU bet {pool} ");

        pool = loop {
            println!("Enter your big blind: ");
            let bet: i32 = read_int();

            if bet > user.score {
                println!("Needs to be less or equal to your current holdings, try again!");
                continue;
            }

            if bet <= pool {
                println!("Needs to be greater than the small blind");
                continue;
            }

            user.score -= bet;
            pool += bet;
            break pool;
        };
    } else {
        println!("Enter your small blind: ");
        let bet: i32 = read_int();
        user.score -= bet;
        pool += bet;
        let bet = rng.random_range(pool..=50);
        println!("CPU sets big blind as: {bet}");
        cpu.score -= bet;
        pool += bet;
    }

    println!("Pool size is now: {pool}");

    println!("Your Hand: ");

    for card in user.cards {
        println!("{} of {:#?}", convert_number(card.number), card.suit);
    }
}
