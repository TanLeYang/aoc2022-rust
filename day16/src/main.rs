use std::{
    collections::{HashMap, HashSet},
    fs,
    cmp::min,
};

use itertools::Itertools;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    IResult, branch::alt,
};

type Valve = String;
type Node = usize;
type Edge = (Node, Node);
type DistMatrix = Vec<Vec<i32>>;

struct ValveInfo {
    valve: Valve,
    neighbors: Vec<Valve>,
    flow_rate: i32,
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input file");
    let (_, all_valve_info) = parse_all_valve_info(&input).expect("Should be able to parse input");

    let valve_to_node = all_valve_info.iter()
        .enumerate()
        .map(|(i, v)| (v.valve.clone(), i))
        .collect::<HashMap<String, Node>>();

    let flow_rates = all_valve_info.iter()
        .map(|v| v.flow_rate)
        .collect::<Vec<i32>>();

    let edge_list = all_valve_info.iter()
        .flat_map(|v| {
            v.neighbors.iter()
                .map(|n| (valve_to_node[&v.valve], valve_to_node[n]))
                .collect::<Vec<Edge>>()
        }).collect::<Vec<Edge>>();

    let num_nodes = all_valve_info.len();
    let dist_matrix = to_distance_matrix(num_nodes, &edge_list);
    let nodes_of_interest = (0..num_nodes)
        .filter(|node| flow_rates[*node] > 0)
        .collect::<Vec<Node>>();

    let subset_pairs = disjoint_subset_pairs(&nodes_of_interest);
    println!("NUMBER OF PAIRS TO RUN: {}", subset_pairs.len());
    let start_node = valve_to_node["AA"];

    let part_2_ans = subset_pairs.iter()
        .map(|(subset, compliment)| {
            let mut visited_subset = vec![false; num_nodes];
            let subset_ans = dfs(start_node, subset, &dist_matrix, &flow_rates, &mut visited_subset, 0, 26).unwrap();
            let mut visited_compliment = vec![false; num_nodes];
            let compliment_ans = dfs(start_node, compliment, &dist_matrix, &flow_rates, &mut visited_compliment, 0, 26).unwrap();
            subset_ans + compliment_ans
        })
        .max().unwrap();

    println!("{:?}", part_2_ans);
}

fn disjoint_subset_pairs(all_nodes: &Vec<Node>) -> Vec<(Vec<Node>, Vec<Node>)> {
    let all_combinations = (1..=(all_nodes.len() / 2))
        .flat_map(|i| all_nodes.iter().copied().combinations(i))
        .map(|vec| vec.into_iter().collect::<HashSet<Node>>())
        .collect::<Vec<HashSet<Node>>>();

    let all_nodes_set = all_nodes.iter().copied().collect::<HashSet<Node>>();
    let compliments = all_combinations.iter()
        .map(|combination| all_nodes_set.difference(combination).copied().collect())
        .collect::<Vec<HashSet<Node>>>();

    all_combinations.iter()
        .zip(compliments.iter())
        .map(|(subset, compliment)| {
            (subset.iter().copied().collect(), compliment.iter().copied().collect())    
        })
        .collect()
}

fn dfs(
    node: Node,
    nodes_of_interest: &Vec<Node>,
    matrix: &DistMatrix,
    flow_rates: &Vec<i32>,
    visited: &mut Vec<bool>,
    total_pressure: i32,
    minutes_left: i32
) -> Option<i32> {
    if minutes_left <= 0 {
        return None
    }

    let nodes_to_visit = nodes_of_interest.iter().copied()
        .filter(|next_node| matrix[node][*next_node] < i32::MAX)
        .filter(|next_node| !visited[*next_node])
        .collect::<Vec<Node>>();

    let best_ans_after_travelling = nodes_to_visit.iter()
        .filter_map(|next_node| {
            visited[*next_node] = true;
            let new_minutes_left = minutes_left - matrix[node][*next_node] - 1; // travel there + 1 min to open the valve
            let new_total_pressure = total_pressure + (flow_rates[*next_node] * new_minutes_left);
            let subproblem_ans = dfs(*next_node, nodes_of_interest, matrix, flow_rates, visited, new_total_pressure, new_minutes_left);
            visited[*next_node] = false;
            subproblem_ans 
        })
        .max();
    
    best_ans_after_travelling.or(Some(total_pressure))
}

fn to_distance_matrix(num_nodes: usize, edge_list: &Vec<Edge>) -> DistMatrix {
    let mut matrix = vec![];
    for _ in 0..num_nodes {
        matrix.push(vec![i32::MAX; num_nodes]); 
    }

    // Floyd-Warshall to compute all pairs shortest path
    for node in 0..num_nodes {
        matrix[node][node] = 0;
    }
    for (u, v) in edge_list {
        matrix[*u][*v] = 1; 
    }
    for k in 0..num_nodes {
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                let new_dist = match matrix[i][k].checked_add(matrix[k][j]) {
                    Some(i) => i,
                    None => i32::MAX
                };
                matrix[i][j] = min(matrix[i][j], new_dist);
            }
        }
    }

    matrix
}

fn parse_all_valve_info(input: &str) -> IResult<&str, Vec<ValveInfo>> {
    separated_list1(newline, parse_valve_info)(input)
}

fn parse_valve_info(input: &str) -> IResult<&str, ValveInfo> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, valve) = alpha1(input)?;

    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow_rate) = nom::character::complete::i32(input)?;
    let (input, _) = alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve ")
    ))(input)?;

    let (input, neighbors) = separated_list1(tag(", "), alpha1)(input)?;

    let valve_info = ValveInfo {
        valve: valve.to_string(),
        neighbors: neighbors.iter().map(|s| s.to_string()).collect(),
        flow_rate,
    };
    Ok((input, valve_info))
}
