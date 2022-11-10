mod user_input;

use rand::Rng;
use std::fs;
use rayon::prelude::*;

fn generate_password_par(length: i64) -> String {
    let characters: [char; 91] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '\'', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', '{', ']', '}', ';', ':', '"', '@', '#', '~', ',', '<', '.', '>', '/', '?'];

    let random_chars: Vec<char> = (0..length).map(|_| characters[rand::thread_rng().gen_range(0..characters.len() - 1)]).collect();
    random_chars.into_par_iter().collect()
}

fn generate_password(length: i64) -> String {
    let characters: [char; 91] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '\'', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', '{', ']', '}', ';', ':', '"', '@', '#', '~', ',', '<', '.', '>', '/', '?'];

    let mut counter: i64 = 0;
    let mut password: String = String::from("");

    while counter < length {
        let percentage_progress: f64 = ((counter as f64) / (length as f64) * 100.0) + 1.0;
        if percentage_progress % 1.0 == 0.0 {
            println!("{}% complete.", percentage_progress);
        }
        let random_index: usize = rand::thread_rng().gen_range(0..characters.len() - 1);
        password.push(characters[random_index]);
        counter += 1;
    }

    return password;
}

fn main() {
    let number_of_characters: i64 = user_input::get_user_input("Enter number of characters: ");
    println!("Generating random password...");
    // let password: String = generate_password(number_of_characters);
    let password: String = generate_password_par(number_of_characters);

    println!("Saving password to file...");
    fs::write("password.txt", password).expect("Unable to write file.");
}