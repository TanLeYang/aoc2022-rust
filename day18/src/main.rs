use std::{fs, collections::{HashSet, VecDeque, HashMap}};

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input file");
    let cubes = parse_input(&input);
    let area = calculate_surface_area(&cubes);
    println!("{}", area);

    let holes_area = calculate_holes_area(&cubes);
    println!("{}", area - holes_area);
}

fn calculate_surface_area(all_cubes: &HashSet<(i32, i32, i32)>) -> usize {
    all_cubes.iter()
        .map(|(x, y, z)| {
            let neighbors = [
                (*x + 1, *y, *z), (*x - 1, *y, *z),
                (*x, *y + 1, *z), (*x, *y - 1, *z),
                (*x, *y, *z + 1), (*x, *y, *z - 1)
            ];

            neighbors.iter()
                .filter(|neighbor| !all_cubes.contains(neighbor))
                .count()
        })
        .sum()
}

fn calculate_holes_area(all_cubes: &HashSet<(i32, i32, i32)>) -> usize {
    let min_x = all_cubes.iter().map(|(x, _, _)| *x).min().unwrap();
    let max_x = all_cubes.iter().map(|(x, _, _)| *x).max().unwrap();
    let min_y = all_cubes.iter().map(|(_, y, _)| *y).min().unwrap();
    let max_y = all_cubes.iter().map(|(_, y, _)| *y).max().unwrap();
    let min_z = all_cubes.iter().map(|(_, _, z)| *z).min().unwrap();
    let max_z = all_cubes.iter().map(|(_, _, z)| *z).max().unwrap();

    let mut air: HashMap<(i32, i32, i32), bool> = HashMap::new();
    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            for z in (min_z - 1)..=(max_z + 1) {
                if !all_cubes.contains(&(x, y, z)) {
                    air.insert((x, y, z), false);
                }
            }
        }
    }

    let start = (min_x - 1, min_y - 1, min_z - 1);
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
    queue.push_back(start);
    while !queue.is_empty() {
        let (x, y, z) = queue.pop_front().unwrap();
        if visited.contains(&(x, y, z)) {
            continue;
        } else {
            visited.insert((x, y, z));
        }

        air.insert((x, y, z), true);
        let neighbors = [
            (x + 1, y, z), (x - 1, y, z),
            (x, y + 1, z), (x, y - 1, z),
            (x, y, z + 1), (x, y, z - 1)
        ];

        neighbors.iter()
            .for_each(|neighbor| {
                if air.contains_key(neighbor) && !air[neighbor] {
                    queue.push_back(*neighbor);
                }
            })
    }

    let inner_cubes = air.keys().cloned().filter(|c| !air[c]).collect::<HashSet<(i32, i32, i32)>>();
    calculate_surface_area(&inner_cubes)
}

fn parse_input(input: &str) -> HashSet<(i32, i32, i32)> {
    input.lines()
        .map(|line| line.split(','))
        .map(|chars| {
            let numbers = chars.map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            (numbers[0], numbers[1], numbers[2])
        })
        .collect()
}
