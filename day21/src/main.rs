use std::{
    collections::{
        HashMap,
        HashSet,
    },
    fs,
};

use nom::{
    IResult,
    sequence::delimited,
    bytes::complete::tag,
    character::complete::{
        alpha1,
        one_of,
        newline,
    },
    Parser,
    branch::alt,
    multi::separated_list1,
};

enum MonkeyJob<'a> {
    Number(i64),
    Math(&'a str, &'a str, Operator),
}

enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

type MonkeyDependencyGraph<'a> = HashMap<&'a str, Vec<&'a str>>;

const MONKEY_OF_INTEREST: &str = "root";
const HUMAN: &str = "humn";

struct Monkey<'a> {
    name: &'a str,
    job: MonkeyJob<'a>,
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input file");
    let (_, monkeys) = parse_monkeys(&input).expect("Should be able to parse input");
    
    let monkey_dependency_graph = build_graph(&monkeys);
    let name_to_monkey_map: HashMap<&str, &Monkey> = monkeys.iter()
        .map(|m| (m.name, m))
        .collect();
    let topological_order = topological_sort(&monkey_dependency_graph);

    let part_1_ans = solve_part_1(&topological_order, &name_to_monkey_map);
    println!("{:?}", part_1_ans);

    let part_2_ans = solve_part_2(&monkey_dependency_graph, &name_to_monkey_map);
}

fn solve_part_1<'a>(topological_order: &[&'a str], name_to_monkey_map: &HashMap<&'a str, &'a Monkey>) -> i64 {
    let mut final_monkey_values: HashMap<&str, i64> = name_to_monkey_map.keys()
        .map(|k| (*k, 0))
        .collect();
    
    for monkey_name in &*topological_order {
        let monkey = name_to_monkey_map[monkey_name];
        match &monkey.job {
            MonkeyJob::Number(i) => final_monkey_values.insert(monkey_name, *i),
            MonkeyJob::Math(dependency_1, dependency_2, operator) => {
                let dep_1_val = final_monkey_values[dependency_1];
                let dep_2_val = final_monkey_values[dependency_2];
                let final_val = match operator {
                    Operator::Plus => dep_1_val + dep_2_val,
                    Operator::Minus => dep_1_val - dep_2_val,
                    Operator::Multiply => dep_1_val * dep_2_val,
                    Operator::Divide => dep_1_val / dep_2_val,
                };
                final_monkey_values.insert(monkey_name, final_val)
            },
        };
    }

    final_monkey_values[MONKEY_OF_INTEREST]
}

fn solve_part_2<'a>(graph: &'a MonkeyDependencyGraph, name_to_monkey_map: &HashMap<&'a str, &'a Monkey>) -> i64 {
    let root = name_to_monkey_map[MONKEY_OF_INTEREST];
    let root_dependencies = match root.job {
        MonkeyJob::Math(dep_1, dep_2, _) => [dep_1, dep_2],
        MonkeyJob::Number(_) => panic!("expect root monkey to have a math job"),
    };
    println!("{:?}", root_dependencies);

    let dependent_on_human = monkeys_dependent_on(HUMAN, graph);
    for m in &root_dependencies {
        if dependent_on_human.contains(m) {
            println!("{} is dependent on human", m);
        } else {
            println!("{} is not dependent on human", m)
        }
    }

    0
}

fn monkeys_dependent_on<'a>(monkey: &'a str, graph: &'a MonkeyDependencyGraph) -> Vec<&'a str> {
    let mut processed: HashSet<&str> = HashSet::new();
    let mut dependent_monkeys: Vec<&str> = Vec::new();
    let mut frontier: Vec<&str> = vec![monkey];
    
    while !frontier.is_empty() {
        let mut new_frontier = Vec::new();
        for m in &frontier {
            for neighbor in &graph[*m] {
                if processed.contains(*neighbor) {
                    continue;
                }

                processed.insert(*neighbor);
                new_frontier.push(*neighbor);
                dependent_monkeys.push(*neighbor);
            } 
        }
        frontier = new_frontier;
    }

    dependent_monkeys
}

fn build_graph<'a>(monkeys: &'a [Monkey]) -> MonkeyDependencyGraph<'a> {
    let mut graph = MonkeyDependencyGraph::new();
    // insert nodes first
    monkeys.iter()
        .for_each(|m| {
            graph.insert(m.name, vec![]);
        });

    // build edges
    monkeys.iter()
        .for_each(|m| match m.job {
            MonkeyJob::Number(_) => (),
            MonkeyJob::Math(dependency_1, dependency_2, _) => {
                graph.get_mut(dependency_1).unwrap().push(m.name);
                graph.get_mut(dependency_2).unwrap().push(m.name);
            }
        });

    graph
}

fn topological_sort<'a>(graph: &'a MonkeyDependencyGraph) -> Vec<&'a str> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut result: Vec<&str> = vec![]; 
    for monkey in (*graph).keys() {
        if !visited.contains(monkey) {
            visit(monkey, graph, &mut visited, &mut result);
        }
    }
    result.reverse();
    result
}

fn visit<'a>(monkey: &'a str, graph: &'a MonkeyDependencyGraph, visited: &mut HashSet<&'a str>, result: &mut Vec<&'a str>) {
    visited.insert(monkey);
    for dependent_monkey in graph.get(&monkey).unwrap() {
        if visited.contains(dependent_monkey) {
            continue;
        }
        visit(*dependent_monkey, graph, visited, result)
    }
    result.push(monkey);
}

fn parse_math(input: &str) -> IResult<&str, MonkeyJob> {
    let (input, first_name) = alpha1(input)?;
    let (input, operator) = delimited(
        tag(" "),
        one_of("*+-/").map(|c| match c {
            '+' => Operator::Plus,
            '-' => Operator::Minus,
            '*' => Operator::Multiply,
            '/' => Operator::Divide,
            _ => panic!("Unknown operator"),
        }),
        tag(" ")
    )(input)?;
    let (input, second_name) = alpha1(input)?;

    Ok((
        input,
        MonkeyJob::Math(first_name, second_name, operator),
    ))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, job) = alt((
        nom::character::complete::i64.map(MonkeyJob::Number),
        parse_math,
    ))(input)?;
    Ok((input, Monkey { name, job }))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(newline, parse_monkey)(input)
}

