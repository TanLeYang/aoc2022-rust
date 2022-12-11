use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input");

    let input_characters: Vec<char> = input.chars().collect(); 
    let mut characters_seen: HashSet<char> = HashSet::new();
    for start_idx in 0..input_characters.len() {
        characters_seen.clear();
        for end_idx in start_idx..start_idx+14 {
            let new_char = input_characters[end_idx];
            if characters_seen.contains(&new_char) {
                break;
            }

            characters_seen.insert(new_char);

            if end_idx == start_idx + 13 {
                println!("{}", end_idx + 1);
                return;
            }
        }
    }
}
