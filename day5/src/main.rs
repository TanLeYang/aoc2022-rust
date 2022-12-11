use std::fs;

use nom::{
    IResult,
    character::complete::{self, alpha1, newline, space1, digit1, multispace1},
    branch::alt,
    bytes::complete::tag,
    sequence::{delimited, preceded},
    multi::{separated_list1, many1}
};

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input");

    let (_, (mut crates, moves)) = parse_input(&input).expect("Should be able to parse input");

    for Move { quantity, from, to } in moves.iter() {
        let from_stack = &mut crates[*from as usize];
        let final_length = from_stack.len().saturating_sub(*quantity as usize);
        let drain: Vec<&str> = from_stack.drain(final_length..).collect();

        for cr in drain.iter() {
            crates[*to as usize].push(cr);
        }
    }

    let result: String = crates.iter()
        .map(|crate_stack| match crate_stack.iter().last() {
            Some(c) => c,
            None => ""
        })
        .collect();

    println!("{}", result);
}

#[derive(Clone, Copy, Debug)]
struct Move {
    quantity: u32,
    from: u32,
    to: u32
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    let (input, crates_horizontal) = separated_list1(newline, parse_line)(input)?;
    let (input, _) = newline(input)?;

    let (input, _numbers) =
        many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;

    let (input, moves) =
        separated_list1(newline, parse_move)(input)?;

    let mut crates_vertical: Vec<Vec<&str>> = vec![];
    let num_crates = crates_horizontal[0].len();
    for _ in 0..num_crates {
        crates_vertical.push(vec![]);
    }

    for vec in crates_horizontal.iter().rev() {
        for (idx, cr_opt) in vec.iter().enumerate() {
            if let Some(cr) = cr_opt {
                crates_vertical[idx].push(cr);
            }
        }
    }

    Ok((input, (crates_vertical, moves)))
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(
            complete::char('['),
            alpha1,
            complete::char(']'),
        )
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value)
    };
    Ok((input, result))
}

fn parse_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, line) = separated_list1(tag(" "), parse_crate)(input)?;
    Ok((input, line))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, quantity) = complete::u32(input)?;

    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;

    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    let m = Move {
       quantity,
       from: from - 1,
       to: to - 1
    };

    Ok((input, m))
}

