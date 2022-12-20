use std::{
    collections::HashSet,
    fs,
    cmp::{max, min}
};

use nom::{
    IResult,
    sequence::separated_pair,
    bytes::complete::tag,
    multi::separated_list1, character::complete::newline
};


#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

type RockPath = Vec<Point>;

fn points_in_path(path: &RockPath) -> Vec<Point> {
    (0..(path.len() - 1))
        .flat_map(|i| points_between(path[i], path[i + 1]))
        .collect()
}

fn points_between(start: Point, end: Point) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    if start.x == end.x {
        let max_y = max(start.y, end.y);
        let min_y = min(start.y, end.y);
        for y in min_y..=max_y {
            points.push(Point { x: start.x, y }) 
        }
    } else {
        let max_x = max(start.x, end.x);
        let min_x = min(start.x, end.x);
        for x in min_x..=max_x {
            points.push(Point { x, y: start.y }) 
        }
    }

    points
}

fn compute_sand_location(current_point: Point, rock_points: &HashSet<Point>, sand_points: &HashSet<Point>, max_y: i32) -> Option<Point> {
    if rock_points.contains(&current_point) || sand_points.contains(&current_point) || current_point.y == max_y {
        return None
    }

    let locations_to_check_in_order = vec![
        Point { x: current_point.x, y: current_point.y + 1 },
        Point { x: current_point.x - 1, y: current_point.y + 1 },
        Point { x: current_point.x + 1, y: current_point.y + 1 },
    ];

    locations_to_check_in_order.iter()
        .find_map(|p| compute_sand_location(*p, rock_points, sand_points, max_y))
        .or(Some(current_point))
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input file");
    
    let (_, rock_paths) = parse_rock_paths(&input).expect("Should be able to parse input");
    let mut rock_points: HashSet<Point> = HashSet::new();
    let mut sand_points: HashSet<Point> = HashSet::new();
    for path in rock_paths.iter() {
        let points = points_in_path(path);
        rock_points.extend(points);
    }

    let starting_point = Point { x: 500, y: 0 };
    let max_y = rock_points.iter().max_by_key(|p| p.y).unwrap().y + 2;
    for i in 1.. {
        let resting_point = compute_sand_location(starting_point, &rock_points, &sand_points, max_y).unwrap();
        if resting_point == starting_point {
            println!("{}", i);
            break;
        } else {
            sand_points.insert(resting_point);
        }
    }
}

fn parse_rock_paths(input: &str) -> IResult<&str, Vec<RockPath>> {
    separated_list1(newline, parse_rock_path)(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(nom::character::complete::i32, tag(","), nom::character::complete::i32)(input)?;
    Ok((input, Point { x, y }))
}

fn parse_rock_path(input: &str) -> IResult<&str, RockPath> {
    separated_list1(tag(" -> "), parse_point)(input)
}

