use std::fs;

#[derive(Clone, Copy)]
enum Shape {
    Paper,
    Rock,
    Scissors
}

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Lose
}

impl Shape {
    fn new(symbol: char) -> Option<Shape> {
        match symbol {
            'A' | 'X' => Some(Shape::Rock),
            'B' | 'Y' => Some(Shape::Paper),
            'C' | 'Z' => Some(Shape::Scissors),
            _ => None
        }
    }

    fn defeats(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper
        }
    }

    fn loses_to(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock
        }
    }

    fn versus(&self, other: Shape) -> Outcome {
        let w = self.defeats();
        let l = self.loses_to();

        if std::mem::discriminant(&other) == std::mem::discriminant(&w) {
            Outcome::Win
        } else if std::mem::discriminant(&other) == std::mem::discriminant(&l) {
            Outcome::Lose
        } else {
            Outcome::Draw
        }
    }
}

impl Outcome {
    fn desired_outcome(symbol: char) -> Option<Outcome> {
        match symbol {
            'X' => Some(Outcome::Lose),
            'Y' => Some(Outcome::Draw),
            'Z' => Some(Outcome::Win),
            _ => None
        }
    }
}

fn compute_score(player: Shape, opponent: Shape) -> i32 {
    let shape_score = match player {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3
    }; 

    let outcome_score = match player.versus(opponent) {
        Outcome::Win => 6,
        Outcome::Lose => 0,
        Outcome::Draw => 3
    };

    shape_score + outcome_score
}

fn shape_for_outcome(opponent: Shape, outcome: Outcome) -> Shape {
    match outcome {
        Outcome::Win => opponent.loses_to(),
        Outcome::Lose => opponent.defeats(),
        Outcome::Draw => opponent
    }
}

fn main() {
    let symbols = fs::read_to_string("./src/input.txt")
        .expect("Should be able to read input")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();
    
    let mut total_score = 0;
    for i in (0..symbols.len()).step_by(2) {
        let opponent = Shape::new(symbols[i]).expect("Should be valid symbol for opponent");
        let desired_outcome = Outcome::desired_outcome(symbols[i + 1]).expect("Shoudl be valid symbol for desired outcome");
        let player = shape_for_outcome(opponent, desired_outcome);
        total_score += compute_score(player, opponent);
    }

    println!("{}", total_score);
}
