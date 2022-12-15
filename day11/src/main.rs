use std::{collections::VecDeque, fs};

use nom::{
    IResult,
    bytes::complete::tag,
    multi::separated_list1,
    character::complete::{self, newline, anychar, alphanumeric1}, branch::alt,
};

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> bool>,
    test_divisor: u64,
    yes_monkey: u32,
    no_monkey: u32,
    num_inspections: u64,
}

impl Monkey {
    fn inspect_one_item(&mut self, modulo: u64) -> (u64, u32) {
        self.num_inspections += 1;
        let item = self.items.pop_front().unwrap();
        let new_item = (self.operation)(item);
        if (self.test)(new_item) {
            (new_item % modulo, self.yes_monkey)
        } else {
            (new_item % modulo, self.no_monkey)
        }
    }
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input file");
    let (_, mut monkeys) = parse_monkeys(&input).expect("Should be able to parse input");
    let num_rounds = 10000;

    let modulo = monkeys.iter()
        .map(|monkey| monkey.test_divisor)
        .product::<u64>();

    for _ in 0..num_rounds {
        for monkey_index in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey = monkeys.get_mut(monkey_index).unwrap();
                let (new_item, monkey_to_send_to) = monkey.inspect_one_item(modulo);
                monkeys
                    .get_mut(monkey_to_send_to as usize)
                    .unwrap().
                    items.
                    push_back(new_item);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.num_inspections);
    let result = monkeys
        .iter()
        .rev()
        .map(|monkey| monkey.num_inspections)
        .take(2)
        .product::<u64>();

    println!("{}", result);
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = separated_list1(newline, parse_monkey)(input)?;
    Ok((input, monkeys))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    // Parse out and ignore the first line of each monkey
    let (input, _) = tag("Monkey ")(input)?;
    let (input, _) = complete::u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = newline(input)?;

    let (input, starting_items) = parse_starting_items(input)?;
    let (input, _) = newline(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, _) = newline(input)?;
    let (input, (test, test_divisor)) = parse_test(input)?;
    let (input, _) = newline(input)?;
    let (input, yes_monkey) = parse_target_monkey(input)?;
    let (input, _) = newline(input)?;
    let (input, no_monkey) = parse_target_monkey(input)?;
    let (input, _) = newline(input)?;

    Ok((
        input,
        Monkey {
            items: VecDeque::from(starting_items),
            operation,
            test,
            test_divisor,
            yes_monkey,
            no_monkey,
            num_inspections: 0,
        }
    ))
}

fn parse_starting_items(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("  Starting items: ")(input)?;
    let (input, items) = separated_list1(tag(", "), complete::u64)(input)?;
    Ok((input, items))
}

fn parse_operation(input: &str) -> IResult<&str, Box<dyn Fn(u64) -> u64>> {
    let (input, _) = tag("  Operation: new = old ")(input)?;
    let (input, operator) = anychar(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, operand) = alphanumeric1(input)?;
    let numeric_operand = match operand.parse::<u64>() {
        Ok(n) => Some(n),
        Err(_) => None,
    };

    if operator == '*' {
        if let Some(n) = numeric_operand {
            Ok((input, Box::new(move |x| x * n)))
        } else {
            Ok((input, Box::new(|x| x * x)))
        }
    } else {
        if let Some(n) = numeric_operand {
            Ok((input, Box::new(move |x| x + n)))
        } else {
            Ok((input, Box::new(|x| x + x)))
        }
    }
}

fn parse_test(input: &str) -> IResult<&str, (Box<dyn Fn(u64) -> bool>, u64)> {
    let (input, _) = tag("  Test: divisible by ")(input)?;
    let (input, val) = complete::u64(input)?;
    Ok((input, (Box::new(move |x| x % val == 0), val)))
}

fn parse_target_monkey(input: &str) -> IResult<&str, u32> {
    let (input, _) = alt((tag("    If true: throw to monkey "), tag("    If false: throw to monkey ")))(input)?;
    let (input, val) = complete::u32(input)?;
    Ok((input, val))
}

