use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input file");

    let elevation_map = parse_grid(&input);
    let start_coordinates = find_possible_starts(&elevation_map);
    let end_coordinate = find_end(&elevation_map).expect("Input should contain an ending position");

    let result = start_coordinates
        .iter()
        .filter_map(|start_coordinate| bfs(elevation_map.clone(), *start_coordinate, end_coordinate))
        .min()
        .expect("Should have at least one valid starting position");

    println!("{}", result);
}

fn bfs(mut elevation_map: Vec<Vec<char>>, start: (i32, i32), end: (i32, i32)) -> Option<i32> {
    let visited_marker = '#';
    let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut frontier: Vec<(i32, i32)> = vec![start];
    let mut num_steps = 0;
    while !frontier.is_empty() {
        let mut arrived = false;
        let mut new_frontier: Vec<(i32, i32)> = vec![];
        for (i, j) in frontier {
            let curr_elevation = match elevation_map[i as usize][j as usize] {
                'S' => 'a',
                c => c
            };

            if curr_elevation == visited_marker {
                continue;
            } 

            elevation_map[i as usize][j as usize] = visited_marker;
            if (i, j) == end {
                arrived = true;
                break;
            }

            for (di, dj) in directions.iter() {
                let new_i = i + di;
                let new_j = j + dj;

                if new_i < 0 || new_i >= elevation_map.len() as i32 || new_j < 0 || new_j >= elevation_map[0].len() as i32 {
                    continue;
                }

                let neighbour_elevation = elevation_map[new_i as usize][new_j as usize];
                if (neighbour_elevation as i32) - (curr_elevation as i32) <= 1 {
                    new_frontier.push((new_i, new_j));
                }
            }
        }

        if arrived {
            return Some(num_steps);
        } else {
            num_steps += 1;
            frontier = new_frontier;
        }
    }

    None 
}

fn find_end(elevation_map: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    for i in 0..elevation_map.len() {
        for j in 0..elevation_map[0].len() {
            if elevation_map[i][j] == 'E' {
                return Some((i as i32, j as i32))
            }
        }
    }
    None
}

fn find_possible_starts(elevation_map: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut starting_coords = vec![];
    for i in 0..elevation_map.len() {
        for j in 0..elevation_map[0].len() {
            if elevation_map[i][j] == 'S' || elevation_map[i][j] == 'a' {
                starting_coords.push((i as i32, j as i32));
            }
        }
    }
    starting_coords
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
