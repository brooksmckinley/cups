extern crate rand;

use std::io;

mod cup;

use cup::{Cup, Mug, Glass, Tumbler};

enum CupType {
    Mug,
    Glass,
    Tumbler,
}

fn buy(cups: &mut Vec<Box<Cup>>) {
    let stdin = io::stdin();
    println!("What Type of Cup is it?");
    println!("  1. Mug");
    println!("  2. Glass");
    println!("  3. Tumbler");
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let cup_type = match input.trim() {
        "1" => CupType::Mug,
        "2" => CupType::Glass,
        "3" => CupType::Tumbler,
        _ => {
            println!("Invalid type of cup.");
            return;
        }
    };

    println!("Name?");
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let cup_name = String::from(input.trim());

    println!("Serial?");
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let cup_serial: i32 = match input.trim().parse() {
        Ok(i) => i,
        Err(_) => { println!("Invalid input."); return; }
    };

    println!("Color?");
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let color = String::from(input.trim());

    println!("Max fluid?");
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let cup_fluid: f64 = match input.trim().parse() {
        Ok(i) => i,
        Err(_) => { println!("Invalid input."); return }
    };

    let cup_handle: bool = match cup_type {
        CupType::Tumbler => false,
        _ => {
            println!("Handle? (y/n)");
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            match input.trim() {
                "y" => true,
                "n" => false,
                _ => { println!("Invalid input"); return; }
            }
        }
    };

    let cup = match cup_type {
        CupType::Mug => Mug::new(cup_name, cup_serial, color, cup_handle, cup_fluid),
        CupType::Glass => Glass::new(cup_name, cup_serial, color, cup_handle, cup_fluid),
        CupType::Tumbler => Tumbler::new(cup_name, cup_serial, color, cup_fluid),
    };

    cups.push(Box::new(cup));

    println!("Thank you.");
}

fn sell(cups: &mut Vec<Box<Cup>>) {
    let stdin = io::stdin();
    println!("Which cup?");
    let mut i = 0;
    for cup in cups.as_slice() {
        println!("{}: {}", i, cup.name());
        i += 1;
    }

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let choice: usize = match input.trim().parse() {
        Ok(i) => i,
        Err(_) => { println!("Invalid input."); return; }
    };

    if choice >= cups.len() {
        println!("Invalid input.");
        return;
    }

    cups.remove(choice);
    
    println!("You sold the cup.");
    
}

fn maintenance(cups: &mut Vec<Box<Cup>>) {
    let stdin = io::stdin();
    println!("Maintenance Menu");
    println!("  1. Fill Cup");
    println!("  2. Empty Cup");
    println!("  3. Drop Cup");
    println!("  4. Break Cup");
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    match input.trim() {
        "1" => {
            println!("Which cup?");
            let mut i = 0;
            for cup in cups.as_slice() {
                println!("{}: {}", i, cup.name());
            }
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let choice: usize = match input.trim().parse() {
                Ok(i) => i,
                Err(_) => { println!("Invalid input."); return; },
            };
            if choice >= cups.len() {
                println!("Invalid input.");
                return;
            }


            println!("How much would you to fill it up by?");
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let fill_by: f64 = match input.trim().parse() {
                Ok(i) => i,
                Err(_) => { println!("Invalid input."); return },
            };

            cups[choice].fill(fill_by);
            println!("You filled the cup.");
        },
        "2" => {
            println!("Which cup?");
            let mut i = 0;
            for cup in cups.as_slice() {
                println!("{}: {}", i, cup.name());
            }
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let choice: usize = match input.trim().parse() {
                Ok(i) => i,
                Err(_) => { println!("Invalid input."); return; },
            };
            if choice >= cups.len() {
                println!("Invalid input.");
                return;
            }


            println!("How much would you to pour out?");
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let empty_by: f64 = match input.trim().parse() {
                Ok(i) => i,
                Err(_) => { println!("Invalid input."); return },
            };

            cups[choice].empty(empty_by);
            println!("You filled the cup.");
        },
        "3" => {
            println!("Which cup?");
            let mut i = 0;
            for cup in cups.as_slice() {
                println!("{}: {}", i, cup.name());
            }
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let choice: usize = match input.trim().parse() {
                Ok(i) => i,
                Err(_) => { println!("Invalid input."); return; },
            };
            if choice >= cups.len() {
                println!("Invalid input.");
                return;
            }

            println!("You dropped the cup.");
            cups[choice].drop_cup();
            if cups[choice].is_broken() {
                println!("The cup broke!");
            }
            else {
                println!("The cup did not break,");
            }
        },
        "4" => {
            println!("Which cup?");
            let mut i = 0;
            for cup in cups.as_slice() {
                println!("{}: {}", i, cup.name());
            }
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let choice: usize = match input.trim().parse() {
                Ok(i) => i,
                Err(_) => { println!("Invalid input."); return; },
            };
            if choice >= cups.len() {
                println!("Invalid input.");
                return;
            }

            cups[choice].break_cup();
            println!("You broke the cup.");
        },
        _ => println!("Invalid option."),
    }
}

fn inventory_report(cups: &Vec<Box<Cup>>) {
    println!("Name\tSerial\tColor\tFluid\tHandle\tBroken");
    for cup in cups.as_slice() {
        println!("{}\t{}\t{}\t{}\t{}\t{}", cup.name(), cup.serial(), cup.color(), cup.current_fluid(), cup.handle(), cup.is_broken());
    }
}

fn main() {
    let stdin = io::stdin();
    let mut cups: Vec<Box<Cup>> = Vec::new();
    loop {
        println!("Welcome to Cups’R’Us!");
        println!("Menu Options:");
        println!("  1. Buy");
        println!("  2. Sell");
        println!("  3. Maintenance");
        println!("  4. Inventory Report");
        println!("  5. Logout");
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        match input.trim() {
            "1" => buy(&mut cups),
            "2" => sell(&mut cups),
            "3" => maintenance(&mut cups),
            "4" => inventory_report(&mut cups),
            "5" => break,
            _ => println!("Invalid input"),
        }
    }
}
