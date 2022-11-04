// Private module, only this file should have access to the data.
mod char_codes;
// Import the const data.
use crate::password_tools::char_codes::CHAR_RANGES;

// Returns a vector of strings containing names of character code ranges.
pub fn get_char_code_range_names() -> Vec<String> {
    let mut range_name_vec: Vec<String> = Vec::new();

    for name in CHAR_RANGES {
        range_name_vec.push(String::from(name.0));
    }

    return range_name_vec;
}

// Receives a vector of bools indicating whether to use each type of character.
// Returns a vector of character codes based on the types desired in the input.
pub fn build_char_code_vec(index_ids_to_use: &Vec<bool>) -> Vec<u32> {
    let mut code_vec: Vec<u32> = Vec::new();

    // Check for mismatched lengths.
    if index_ids_to_use.len() != CHAR_RANGES.len() {
        println!("Invalid index_ids_to_use vector!");
        return code_vec;
    }

    // Iterate over the index ids and add to the code vector as needed.
    for i in 0..index_ids_to_use.len() {
        if index_ids_to_use[i] {
            for (start, end) in CHAR_RANGES[i].1 {
                // let mut t_vec: Vec<u32> = (start..=end).collect();
                code_vec.append(&mut (*start..=*end).collect());
            }
        }
    }

    return code_vec;
}