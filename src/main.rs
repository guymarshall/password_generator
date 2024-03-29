#![forbid(unsafe_code)]

mod user_input;

use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use rayon::prelude::*;
use rand::Rng;

fn generate_password(length: i64) {
    let characters: [char; 91] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '\'', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', '{', ']', '}', ';', ':', '"', '@', '#', '~', ',', '<', '.', '>', '/', '?'];
    let characters_length: usize = characters.len();

    let mut random_chars: Vec<char> = Vec::new();
    let mut save_interval: i64 = 100_000_000;

    if length < save_interval {
        save_interval = length;
    }
    // this is needed to create an empty file each time
    let _: File = File::create("password.txt").expect("Could not create file");
    let mut file: File = OpenOptions::new().write(true).append(true).open("password.txt").expect("Could not create file");
    for i in 0..length {
        let character: char = characters[rand::thread_rng().gen_range(0..characters_length - 1)];
        random_chars.push(character);
        if (i + 1) % save_interval == 0 || (i + 1) == length {
            let password: String = random_chars.into_par_iter().collect();

            if let Err(e) = writeln!(file, "{}", password) {
                eprintln!("Couldn't write to file: {}", e);
            }

            random_chars = Vec::new();
        }

        let percentage_progress: f64 = (i as f64 / length as f64) * 100.0;
        if percentage_progress % 1.0 == 0.0 {
            println!("{}% complete.", percentage_progress + 1.0);
        }
    }
}

fn main() {
    let number_of_characters: i64 = user_input::input("Enter number of characters: ");
    println!("Generating random password...");
    generate_password(number_of_characters);
}