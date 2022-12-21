use std::{
    fs,
    ops::RangeInclusive,
    cmp::{min, max}
};

use nom::{
    IResult,
    bytes::complete::tag, multi::separated_list1, character::complete::newline,
};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct SensorBeaconPair {
    sensor_position: Point,
    beacon_position: Point,
    distance_between: i32,
}

impl SensorBeaconPair {
    fn new(sensor_position: Point, beacon_position: Point) -> Self {
        let distance_between = manhatten_distance(&sensor_position, &beacon_position);
        Self {
            sensor_position,
            beacon_position,
            distance_between,
        }
    }

    fn marked_column_range_for_row(
        &self,
        row: i32,
        min_col: i32,
        max_col: i32
    ) -> Option<RangeInclusive<i32>> {
        let distance_to_row = (row - self.sensor_position.y).abs();
        if distance_to_row > self.distance_between {
            None
        } else {
            let col_offset = self.distance_between - distance_to_row;
            let col_lower_bound = max(min_col, self.sensor_position.x - col_offset);
            let col_upper_bound = min(max_col, self.sensor_position.x + col_offset);
            Some(RangeInclusive::new(col_lower_bound, col_upper_bound))
        }
    }
}

fn sort_merge_ranges(mut ranges: Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    ranges.sort_by_key(|r| *r.start());
    let mut result = vec![];
    let mut current_range = ranges[0].clone();
    for i in 1..ranges.len() {
        let range = ranges[i].clone();
        if range.start() <= current_range.end() {
            let new_end = max(*range.end(), *current_range.end());
            current_range = RangeInclusive::new(*current_range.start(), new_end);
        } else {
            result.push(current_range);
            current_range = range;
        }
    }

    result.push(current_range);
    result
}

fn find_missing(
    ranges: Vec<RangeInclusive<i32>>,
    desired_range: &RangeInclusive<i32>
) -> Option<i32> {
    if ranges.len() == 1 && &ranges[0] == desired_range {
        return None
    }

    let mut next_expected = *desired_range.start();
    for range in ranges {
        if *range.start() != next_expected {
            return Some(next_expected)
        }

        next_expected = range.end() + 1;
    }

    if next_expected != *desired_range.end() + 1 {
        Some(next_expected)
    } else {
        None
    }
}

fn manhatten_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input file");
    let (_, sensor_beacon_pairs) = parse_sensor_beacon_pairs(&input).expect("Should be able to parse input");
    
    let min_coord = 0;
    let max_coord = 4000000;
    let tuning_frequency_multiplier: i64 = 4000000;
    let desired_range = RangeInclusive::new(min_coord, max_coord);
    for row in min_coord..=max_coord {
        let ranges = sensor_beacon_pairs.iter() 
            .filter_map(|p| p.marked_column_range_for_row(row, min_coord, max_coord))
            .collect::<Vec<RangeInclusive<i32>>>();

        let sorted_merged_ranges = sort_merge_ranges(ranges);
        if let Some(missing_col) = find_missing(sorted_merged_ranges, &desired_range) {
            let tuning_frequency = i64::from(missing_col) * tuning_frequency_multiplier + i64::from(row); 
            println!("{}", tuning_frequency);
            return
        }
    }

    println!("NO ANSWER FOUND!!!!");
}

fn parse_sensor_beacon_pairs(input: &str) -> IResult<&str, Vec<SensorBeaconPair>> {
    separated_list1(newline, parse_sensor_beacon_pair)(input)
}

fn parse_sensor_beacon_pair(input: &str) -> IResult<&str, SensorBeaconPair> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sensor_x) = nom::character::complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sensor_y) = nom::character::complete::i32(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x) = nom::character::complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, beacon_y) = nom::character::complete::i32(input)?;
    Ok((input, SensorBeaconPair::new(
        Point { x: sensor_x, y: sensor_y, },
        Point { x: beacon_x, y: beacon_y, }
    )))
}

