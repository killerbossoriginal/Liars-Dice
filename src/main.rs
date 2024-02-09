use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn roll () -> u8 {
    rand::thread_rng().gen_range(1..=6)
}

fn input () -> (u8, u8) {
    let mut number1: u8 = 0;
    let mut number2: u8 = 0;

    while number1 == 0 || number2 == 0 || number1 > 6 || number2 > 6 {
        let mut n: String = String::new();

        if (number2 == 0 && number1 != 0) || number2 > 6 {
            println!("Select the dice number (1 to 6).");
            println!("Input 7 to return to dice selection.");
            io::stdin()
                .read_line(&mut n)
                .expect("Failed to read line");
            number2 = match n.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        }
        if number1 == 0 || number2 == 7 || number1 > 6 {
            println!("Select your dice (1 to 6).");
            io::stdin()
                .read_line(&mut n)
                .expect("Failed to read line");
            number1 = match n.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        }
    };
    (number1, number2)
}

fn main() {
    println!("Welcome to Liar's Dice!");

    let mut player_number: u8 = 0;
    while player_number == 0 {
        println!("Please input player number.");
        let mut pn: String = String::new();
        io::stdin()
            .read_line(&mut pn)
            .expect("Failed to read line");
    
            player_number = match pn.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }

    let mut players: Vec<[u8; 5]> = vec![];
    println!("Rolling...");
    for _ in 0..player_number {
        players.push([roll(), roll(), roll(), roll(), roll()]);
    }
    println!("Rolled\nYour dices are:");
    for dice in players[0] {
        println!("{dice}");
    }

    let a = input();
    println!("{} {}", a.0, a.1);
    // let won = false;
    // while !won {

    // }
}