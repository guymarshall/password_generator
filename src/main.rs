mod user_input;

use rand::Rng;
use std::fs;
use rayon::prelude::*;

fn generate_password(length: i64) -> String {
    let characters: [char; 91] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '\'', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', '{', ']', '}', ';', ':', '"', '@', '#', '~', ',', '<', '.', '>', '/', '?'];
    let characters_length: usize = characters.len();

    let random_chars: Vec<char> = (0..length).into_par_iter().map(|_| characters[rand::thread_rng().gen_range(0..characters_length - 1)]).collect();
    random_chars.into_par_iter().collect()
}

fn main() {
    let number_of_characters: i64 = user_input::get_user_input("Enter number of characters: ");
    println!("Generating random password...");
    let password: String = generate_password(number_of_characters);

    println!("Saving password to file...");
    // generate_password(number_of_characters);
    fs::write("password.txt", password).expect("Unable to write file.");
}