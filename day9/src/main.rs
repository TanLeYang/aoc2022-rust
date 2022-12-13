use std::{cmp::max, collections::HashSet, fs};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point(i32, i32);

#[derive(Clone, Copy)]
enum Move {
    Right,
    Left,
    Up,
    Down,
}

fn new_head(original_head: Point, motion: Move) -> Point {
    match motion {
        Move::Right => Point(original_head.0 + 1, original_head.1),
        Move::Left => Point(original_head.0 - 1, original_head.1),
        Move::Up => Point(original_head.0, original_head.1 + 1),
        Move::Down => Point(original_head.0, original_head.1 - 1),
    }
}

fn new_tail(new_head: Point, original_tail: Point) -> Point {
    let distance = max((new_head.0 - original_tail.0).abs(), (new_head.1 - original_tail.1).abs());
    if distance <= 1 {
        return original_tail
    }

    let x_direction = if new_head.0 < original_tail.0 {
        -1
    } else {
        1
    };

    let y_direction = if new_head.1 < original_tail.1 {
        -1
    } else {
        1
    };

    if new_head.0 == original_tail.0 {
        Point(original_tail.0, original_tail.1 + y_direction)
    } else if new_head.1 == original_tail.1 {
        Point(original_tail.0 + x_direction, original_tail.1)
    } else {
        Point(original_tail.0 + x_direction, original_tail.1 + y_direction)
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let c = parts[0];
        let m = match c {
            "R" => Move::Right,
            "L" => Move::Left,
            "U" => Move::Up,
            "D" => Move::Down,
            _ => Move::Down,
        };
        let n: i32 = parts[1].to_string().parse().unwrap();

        for _ in 0..n {
            moves.push(m);
        }
    }

    moves
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input file");
    let moves = parse_moves(&input);

    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(Point(0, 0));
    let num_knots = 10;
    let mut knots: Vec<Point> = vec![];
    for _ in 0..num_knots {
        knots.push(Point(0, 0));
    }

    for m in moves {
        let new_head = new_head(knots[0], m);
        knots[0] = new_head;
        for i in 1..num_knots {
            let new_knot_position = new_tail(knots[i - 1], knots[i]);
            knots[i] = new_knot_position;
        }

        visited.insert(knots[num_knots - 1]);
    }

    println!("{}", visited.len());
}
