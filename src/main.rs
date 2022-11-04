use std::io;
use rand::Rng;
pub mod password_tools;

// Min/max password lengths.
const MIN_PW_LENGTH: u32 = 8;
const MAX_PW_LENGTH: u32 = 30;

fn main() {
    pw_loop();
}

fn pw_loop() {
    // Get the desired character length of the password from the user.
    let num_pw_chars: u32 = get_num_pw_chars();

    // Setup character ranges for the password generator.
    let mut char_ranges_to_use: Vec<bool> = Vec::new();
    let range_name_vec: Vec<String> = password_tools::get_char_code_range_names();

    // Get input from user.
    // Yes/no to using specific character sets.
    for range_name in range_name_vec {
        loop {
            println!("Would you like to use {range_name} characters? [y/n]");
    
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line!");

            // the trim function returns a &str slice.
            let user_input = user_input.trim();
    
            match user_input {
                "y" | "Y" => {
                    println!("Added {range_name} characters to the shuffle!");

                    // Flag character set to be added to a vector.
                    char_ranges_to_use.push(true);

                    break;
                },
                "n" | "N" => {
                    println!("Password will not contain {range_name} characters.");
                    char_ranges_to_use.push(false);

                    break;
                },
                _ => {
                    continue;
                }
            }
        }
    }
    // Get an array of ints mapping to utf-8 (or ascii) character codes based on desired character types.
    let char_code_vec: Vec<u32> = password_tools::build_char_code_vec(&char_ranges_to_use);

    println!("New password: {}", generate_password(num_pw_chars, char_code_vec));
}

// Prompts user for input.
// Returns the number of characters desired in the password.
fn get_num_pw_chars() -> u32 {
    println!("Please enter the desired length of the password ({MIN_PW_LENGTH}-{MAX_PW_LENGTH}):");
    loop {
        let mut pw_length_input: String = String::new();
        io::stdin()
            .read_line(&mut pw_length_input)
            .expect("Failed to read line!");

        match pw_length_input.trim().parse() {
            Ok(pw_length) => {
                if pw_length >= MIN_PW_LENGTH && pw_length <= MAX_PW_LENGTH {
                    return pw_length;
                } else {
                    continue;
                }
            },
            Err(_) => {
                println!("Please enter a proper password length! ({MIN_PW_LENGTH}-{MAX_PW_LENGTH})");
                continue;
            }
        };

    }
}

// Generates a new password using the given length and char code vector.
fn generate_password(password_desired_length: u32, char_code_vec: Vec<u32>) -> String {
    // Use a cryptographically strong prng.
    let mut pw_prng = rand::thread_rng();
    // Final password string.
    let mut new_pw: String = String::new();

    while new_pw.len() < password_desired_length as usize {
        match char::from_u32(char_code_vec[pw_prng.gen_range(0..char_code_vec.len())]) {
            Some(new_char) => {
                // println!("Random char #{}: '{}'", i, new_char);
                new_pw.push(new_char);
            },
            None => {
                new_pw.push(' ');
            }
        }
    }

    return new_pw;
}