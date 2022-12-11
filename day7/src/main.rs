use std::{cmp::min, collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
enum Operation<'a> {
    Cd(CdArg<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum CdArg<'a> {
    Root,
    Up,
    Down { dst: &'a str },
}

#[derive(Debug)]
enum Files<'a> {
    File { size: u32, name: &'a str },
    Dir { name: &'a str },
}

fn parse_file(input: &str) -> IResult<&str, Files> {
    let (input, (size, name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        is_a("qwertyuiopasdfghjklzxcvbnm."),
    )(input)?;

    Ok((input, Files::File { size, name }))
}

fn parse_dir(input: &str) -> IResult<&str, Files> {
    let (input, (_, name)) = separated_pair(tag("dir"), tag(" "), alpha1)(input)?;
    Ok((input, Files::Dir { name }))
}

fn parse_cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((alpha1, tag("/"), tag("..")))(input)?;
    let cd_arg = match dir {
        "/" => CdArg::Root,
        ".." => CdArg::Up,
        name => CdArg::Down { dst: name },
    };

    Ok((input, Operation::Cd(cd_arg)))
}

fn parse_ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((parse_file, parse_dir)))(input)?;
    Ok((input, Operation::Ls((files))))
}

fn parse_operations(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, ops) = separated_list1(newline, alt((parse_cd, parse_ls)))(input)?;
    Ok((input, ops))
}

fn solve(operations: &Vec<Operation>) -> (u32, u32) {
    let mut directory_sizes: HashMap<String, u32> = HashMap::new();
    let mut current_directory_stack: Vec<&str> = vec!["/"];

    for op in operations {
        match op {
            Operation::Cd(CdArg::Root) => {
                current_directory_stack.clear();
                current_directory_stack.push("/");
            }
            Operation::Cd(CdArg::Up) => {
                let _ = current_directory_stack.pop();
            }
            Operation::Cd(CdArg::Down { dst }) => {
                current_directory_stack.push(dst);
            }
            Operation::Ls(files) => {
                let total_file_size = sum_file_only_sizes(files);
                let mut context = "".to_owned();

                for dir in current_directory_stack.iter() {
                    context.push_str("/");
                    context.push_str(dir);

                    *directory_sizes.entry(context.clone()).or_insert(0) += total_file_size;
                }
            }
        }
    }

    let mut part1 = 0;

    let mut part2 = 70000001;
    let max_space_available = 70000000;
    let min_space_required = 30000000;
    let size_of_root = directory_sizes["//"];
    let min_directory_size_to_delete = min_space_required - (max_space_available - size_of_root);

    for (_, v) in directory_sizes {
        if v <= 100000 {
            part1 += v;
        }

        if v < min_directory_size_to_delete {
            continue;
        }

        part2 = min(part2, v);
    }

    return (part1, part2);
}

fn sum_file_only_sizes(files: &Vec<Files>) -> u32 {
    files
        .iter()
        .map(|f| match f {
            Files::File { size, name: _ } => *size,
            Files::Dir { name: _ } => 0,
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input file");
    let (_, operations) = parse_operations(&input).expect("Should be able to parse input");

    let (part1, part2) = solve(&operations);
    println!("{}, {}", part1, part2);
}
