use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Down,
    Right,
    Left,
}

#[derive(Clone, Copy)]
enum RockType {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Rock {
    Horizontal { left: Point },
    Cross { center: Point },
    Corner { center: Point },
    Vertical { bottom: Point },
    Square { bottom_left: Point },
}

impl Rock {
    fn new(rock_type: RockType, left_x: i64, bottom_y: i64) -> Self {
        match rock_type {
            RockType::Horizontal => Self::Horizontal { left: Point { x: left_x, y: bottom_y } },
            RockType::Cross => Self::Cross { center: Point { x: left_x + 1, y: bottom_y + 1 } },
            RockType::Corner => Self::Corner { center: Point { x: left_x + 1, y: bottom_y + 1 } },
            RockType::Vertical => Self::Vertical { bottom: Point { x: left_x, y: bottom_y } },
            RockType::Square => Self::Square { bottom_left: Point { x: left_x, y: bottom_y } },
        }
    }

    fn move_in_direction(&self, direction: Direction) -> Self {
        match &self {
            Self::Horizontal { left } => Self::Horizontal { left: move_point(*left, direction) },
            Self::Cross { center } => Self::Cross { center: move_point(*center, direction) },
            Self::Corner { center } => Self::Corner { center: move_point(*center, direction) },
            Self::Vertical { bottom } => Self::Vertical { bottom: move_point(*bottom, direction) },
            Self::Square { bottom_left } => Self::Square { bottom_left: move_point(*bottom_left, direction) },
        }
    }

    fn covering_points(&self) -> HashSet<Point> {
        match &self {
            Self::Horizontal { left } => (left.x..left.x+4).map(|x| Point { x, y: left.y }).collect(),
            Self::Cross { center } => {
                HashSet::from([
                    *center,
                    Point { x: center.x, y: center.y + 1 },
                    Point { x: center.x, y: center.y - 1 },
                    Point { x: center.x + 1, y: center.y },
                    Point { x: center.x - 1, y: center.y },
                ])
            },
            Self::Corner { center } => {
                HashSet::from([
                    Point { x: center.x + 1, y: center.y },
                    Point { x: center.x + 1, y: center.y + 1 },
                    Point { x: center.x + 1, y: center.y - 1 },
                    Point { x: center.x, y: center.y - 1 },
                    Point { x: center.x - 1, y: center.y - 1 },
                ])
            },
            Self::Vertical { bottom } => (bottom.y..bottom.y+4).map(|y| Point { x: bottom.x, y }).collect(),
            Self::Square { bottom_left } => {
                HashSet::from([
                    *bottom_left,
                    Point { x: bottom_left.x, y: bottom_left.y + 1 },
                    Point { x: bottom_left.x + 1, y: bottom_left. y },
                    Point { x: bottom_left.x + 1, y: bottom_left.y + 1}
                ])
            },
        }
    }
}

struct Chamber {
    rock_points: HashSet<Point>,
    highest_rock_y: i64,
}

impl Chamber {
    fn new() -> Self {
        Self {
            rock_points: HashSet::new(),
            highest_rock_y: -1,
        }
    }

    fn spawn_rock(&mut self, rock_type: RockType) -> Rock {
        let left_x = 2; // bc 2 units from left_wall
        let bottom_y = self.highest_rock_y + 4; // bc 3 units from highest rock
        Rock::new(rock_type, left_x, bottom_y)
    }

    fn move_rock(&mut self, old_rock: Rock, direction: Direction) -> (Rock, bool) {
        // 1. Move in given direction
        let mut rock_moved_in_direction = old_rock.move_in_direction(direction);
        let rock_moved_in_direction_covering_points = rock_moved_in_direction.covering_points();
        
        let is_within_bound = rock_moved_in_direction_covering_points
            .iter()
            .all(|p| p.x >= 0 && p.x <= 6 && p.y >= 0);
        let is_intersecting = rock_moved_in_direction_covering_points.intersection(&self.rock_points).count() != 0;
        if !is_within_bound || is_intersecting {
            rock_moved_in_direction = old_rock;
        }

        // 2. Move downwads
        let final_rock = rock_moved_in_direction.move_in_direction(Direction::Down);
        let final_rock_covering_points = final_rock.covering_points();
        let is_resting_on_floor = final_rock_covering_points
            .iter()
            .any(|p| p.y < 0);
        let is_resting_on_other_rocks = final_rock_covering_points.intersection(&self.rock_points).count() != 0;

        if is_resting_on_floor || is_resting_on_other_rocks {
            self.update_with_rock_at_rest(rock_moved_in_direction);
            (rock_moved_in_direction, true)
        } else {
            (final_rock, false)
        }
    }

    fn update_with_rock_at_rest(&mut self, rock: Rock) {
        let resting_rock_covering_points = rock.covering_points();
        for pt in resting_rock_covering_points {
            self.rock_points.insert(pt);
            if pt.y > self.highest_rock_y {
                self.highest_rock_y = pt.y;
            }
        }
    }
}

fn move_point(point: Point, direction: Direction) -> Point {
    match direction {
        Direction::Down => Point { x: point.x, y: point.y - 1 },
        Direction::Right => Point { x: point.x + 1, y: point.y },
        Direction::Left => Point { x: point.x - 1, y: point.y },
    }
}

fn print_grid(chamber: &Chamber) {
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..(chamber.highest_rock_y + 1) {
        grid.push(vec!['.'; 7])
    }

    for p in &chamber.rock_points {
        grid[p.y as usize][p.x as usize] = '#';
    }

    grid.reverse();
    for row in &grid {
        println!("{:?}", row)
    }
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input file");
    let movement_directions_vec = parse_movement_directions(&input);

    let mut movement_directions_pattern = movement_directions_vec
        .iter()
        .cycle();
    let rock_pattern = [RockType::Horizontal, RockType::Cross, RockType::Corner, RockType::Vertical, RockType::Square]
        .iter()
        .cycle();

    let mut chamber = Chamber::new();
    let num_rocks = 1000000000000;
    for rock_type in rock_pattern.take(num_rocks) {
        let mut curr_rock = chamber.spawn_rock(*rock_type);
        loop {
            let movement = movement_directions_pattern.next().unwrap();
            let (new_rock, is_at_rest) = chamber.move_rock(curr_rock, *movement);
            if is_at_rest {
                break;
            }
            curr_rock = new_rock;
        }
    }

    println!("{}", chamber.highest_rock_y + 1);
}

fn parse_movement_directions(input: &str) -> Vec<Direction> {
    input.chars()
        .filter_map(|c| {
            match c {
                '>' => Some(Direction::Right),
                '<' => Some(Direction::Left),
                _ => None,
            }
        })
        .collect::<Vec<Direction>>()
}
