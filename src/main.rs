use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("{}", concat!(
        "This utility takes in a password and a list of indices ",
        "and selects those characters. No more counting through long ",
        "passwords to try to find the right characters!"
    ));

    // Read in the password
    let mut password = String::new();
    print!("Password: ");
    stdout.flush().unwrap();
    stdin.read_line(&mut password).unwrap();

    // Read in the letter indices
    let mut indices: Vec<usize> = Vec::new();
    loop {
        print!("Character index: ");
        stdout.flush().unwrap();
        let mut idx_str = String::new();
        stdin.read_line(&mut idx_str).unwrap();
        if let Ok(idx_num) = idx_str.trim().parse::<usize>() {
            if idx_num < password.len() {
                indices.push(idx_num);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    println!("");

    for idx in indices {
        println!(
            "Character {} is {}",
            idx,
            password.chars().collect::<Vec<_>>()[idx - 1]
        );
    }
}
