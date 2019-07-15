use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

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

    for idx in indices {
        println!(
            "Character {} is {}",
            idx,
            password.chars().collect::<Vec<_>>()[idx - 1]
        );
    }
}
