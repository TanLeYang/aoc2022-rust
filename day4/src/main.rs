use std::fs;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Interval(i32, i32);

fn main() {
    let input = fs::read_to_string("./src/input.txt")
        .expect("Should be able to read input");

    let interval_pairs = parse_input_to_interval_pairs(&input);

    let num_fully_contained_pairs = interval_pairs
        .iter()
        .filter(|p| is_fully_containing(p.0, p.1))
        .count();

    println!("{}", num_fully_contained_pairs);
    
    let num_overlapping_pairs = interval_pairs
        .iter()
        .filter(|p| is_overlapping(p.0, p.1))
        .count();

    println!("{}", num_overlapping_pairs);
}

fn parse_input_to_interval_pairs(input: &str) -> Vec<(Interval, Interval)> {
    input
        .lines()
        .map(|line| {
            let split_line = line.split(',');
            let intervals: (Interval, Interval) = split_line
                .map(|interval_str| {
                    let interval: (i32, i32) = interval_str 
                        .split('-')
                        .map(|s| s.parse().expect("Should be an i32"))
                        .collect_tuple().unwrap();

                    Interval(interval.0, interval.1)
                })
                .collect_tuple().unwrap();

            intervals
        })
        .collect()
}

fn is_fully_containing(interval_1: Interval, interval_2: Interval) -> bool {
    interval_1.0 >= interval_2.0 && interval_1.1 <= interval_2.1
        || interval_2.0 >= interval_1.0 && interval_2.1 <= interval_1.1
}

fn is_overlapping(interval_1 : Interval, interval_2: Interval) -> bool {
    interval_1.0 >= interval_2.0 && interval_1.0 <= interval_2.1
        || interval_2.0 >= interval_1.0 && interval_2.0 <= interval_1.1
}
