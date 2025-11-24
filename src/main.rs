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

#[derive(PartialEq, Eq)]
enum PlayerAction {
    Bet,
    Fold,
}

#[derive(PartialEq, Eq, Clone)]
struct Player {
    name: String,
    player_type: PlayerType,
    chips: i32,
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

fn ask_action(player: &Player) -> PlayerAction {
    let mut player_action: PlayerAction;
    if player.player_type == PlayerType::Human {
        let mut action_string = String::new();
        player_action = loop {
            println!("{} please specify your action: ", player.name);
            io::stdin()
                .read_line(&mut action_string)
                .expect("Invalid Input");
            player_action = match action_string.trim() {
                "bet" => PlayerAction::Bet,
                "fold" => PlayerAction::Fold,
                _ => {
                    println!("Invalid action specififed, try again");
                    continue;
                }
            };
            break player_action;
        }
    } else {
        player_action = PlayerAction::Bet;
    }
    player_action
}

fn action_fold(player: &mut Player) {
    player.cards.clear();
    println!("{name} Folds their turn", name = player.name);
}

fn action_bet(player: &mut Player, pool: &mut i32, current_bet: &mut i32) {
    let mut rng = rand::rng();
    let amount: i32 = match player.player_type {
        PlayerType::Human => {
            println!("You have {} chips", player.chips);
            println!("Enter your bet amount: ");
            read_int()
        }
        PlayerType::Cpu => rng.random_range(1..=player.chips),
    };

    *pool += amount;
    player.chips -= amount;
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
        chips: 50,
        cards: vec![],
    };

    let mut cpu: Player = Player {
        name: String::from("computer"),
        player_type: PlayerType::Cpu,
        chips: 50,
        cards: vec![],
    };

    // Round starts
    loop {
        let mut seen_cards: Vec<Card> = Vec::new();

        // Display current chips
        for player in [&user, &cpu] {
            println!("{} has {} chips", player.name, player.chips)
        }

        //Blind selection
        let big_blind: &Player = [&user, &cpu].choose(&mut rng).unwrap();
        println!("Selected bigblind as {name} ", name = big_blind.name);

        // Put money in the pot
        if *big_blind == user {
            let bet = rng.random_range(25..=45);
            pot += bet;
            cpu.chips -= pot;

            println!("CPU bet {pot} ");

            pot = loop {
                println!("Enter your big blind: ");
                let bet: i32 = read_int();

                if bet > user.chips {
                    println!("Needs to be less or equal to your current holdings, try again!");
                    continue;
                }

                if bet <= pot {
                    println!("Needs to be greater than the small blind");
                    continue;
                }

                user.chips -= bet;
                pot += bet;
                break pot;
            };
        } else {
            println!("Enter your small blind: ");
            let bet: i32 = read_int();
            user.chips -= bet;
            pot += bet;
            let bet = rng.random_range(pot..=50);
            println!("CPU sets big blind as: {bet}");
            cpu.chips -= bet;
            pot += bet;
        }

        println!("Pool size is now: {pot}");

        // Deal hole cards
        for player in [&mut user, &mut cpu] {
            player.cards.push(generate_card(&mut seen_cards));
            player.cards.push(generate_card(&mut seen_cards));
        }

        println!("Your Hand: ");

        for card in &user.cards {
            display_card(card);
        }

        // Start preflop betting round
        let mut current_bet = 0;

        //User plays
        let user_action = ask_action(&user);

        match user_action {
            PlayerAction::Bet => action_bet(&mut user, &mut pot, &mut current_bet),
            PlayerAction::Fold => {
                action_fold(&mut user);
                break;
            }
        }

        // CPU plays
        let cpu_action = ask_action(&cpu);

        match cpu_action {
            PlayerAction::Bet => action_bet(&mut cpu, &mut pot, &mut current_bet),
            PlayerAction::Fold => {
                action_fold(&mut cpu);
                break;
            }
        };

        //Dealer reveal the cards
        //
        println!("Dealer dealt: ");

        for slot in community_cards[..3].iter_mut() {
            let n_card = generate_card(&mut seen_cards);
            display_card(&n_card);
            *slot = Some(n_card);
        }
        println!("\n\n");
    }
}
