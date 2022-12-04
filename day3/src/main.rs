use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("./src/input.txt")
        .expect("Should be able to read input");

    let rucksacks: Vec<Vec<char>> = input
        .split('\n')
        .map(|s| {
            s.chars().collect::<Vec<char>>()
        })
        .collect();

    let mut score = 0;
    for i in (0..rucksacks.len()).step_by(3) {
        if i + 2 >= rucksacks.len() {
            break;
        }

        let r1 = &rucksacks[i];
        let r2 = &rucksacks[i + 1];
        let r3 = &rucksacks[i + 2];
        let repeating_item = get_repeating_item(r1, r2, r3).expect("Should have repating item in 3 rucksacks");
        score += priority(repeating_item);
    }

    println!("{}", score);
}

fn get_repeating_item(r1: &[char], r2: &[char], r3: &[char]) -> Option<char> {
    let s1: HashSet<&char> = r1.iter().collect();
    let s2: HashSet<&char> = r2.iter().collect();
    for item in r3 {
        if s1.contains(item) && s2.contains(item) {
            return Some(*item);
        }
    }

    None
}

fn priority(item: char) -> u32 {
    let ascii = item as u32;
    if item.is_lowercase() {
        ascii - ('a' as u32) + 1 
    } else {
        ascii - ('A' as u32) + 27
    }
}

