/*
 *
 * A utility to help with the "enter the 12th, 35th, and 63rd characters
 * from your password" situations
 * Copyright (C) 2019 David Young
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the
 * GNU General Public License as published by the Free Software Foundation, either version 3 of
 * the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this program. If
 * not, see <https://www.gnu.org/licenses/>.
 *
 */
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
    print!("{}", print_chars(&password, &indices));

    //TODO Zero the memory properly
    password.clear();
}

#[inline]
fn print_chars(password: &str, indices: &Vec<usize>) -> String {
    let mut output = String::new();
    for idx in indices {
        output.push_str(format!(
            "Character {} is {}\n",
            idx,
            password.chars().collect::<Vec<_>>()[idx - 1]
        ).as_str());
    }

    output
}

#[cfg(test)]
mod test {
    #[test]
    fn test_print_chars() {
        let password: String = "password1234".into();
        let indices = vec![2_usize, 5, 7];

        assert_eq!(
            super::print_chars(&password, &indices),
            String::from("Character 2 is a\nCharacter 5 is w\nCharacter 7 is r\n"),
            );
    }
}
