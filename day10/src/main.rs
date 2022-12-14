use std::fs;

use nom::{
    branch::alt,

    IResult,
    bytes::complete::tag,
    multi::separated_list1,
    character::complete::newline,
};

#[derive(Clone, Copy, Debug)]
enum Instruction {
    AddX(i32),
    NoOp,
}

#[derive(Clone, Copy)]
struct MachineState {
    register_value: i32,
    cycle: i32,
}

struct Screen {
    num_rows: i32,
    num_cols: i32,
    pixels: Vec<Vec<char>>,
}

impl Screen {
    fn new(num_rows: i32, num_cols: i32) -> Self {
        let mut pixels: Vec<Vec<char>> = vec![];
        for _ in 0..num_rows {
            pixels.push(vec![]);
        }

        Screen {
            num_rows,
            num_cols,
            pixels,
        }
    }

    fn draw(&mut self, state: MachineState) {
        let pixel_position = state.cycle - 1;
        let pixel_row = pixel_position / self.num_cols;
        let sprite_position = state.register_value;

        let c = match self.sprite_is_visible(sprite_position, pixel_position) {
            true => '#',
            false => '.',
        };
        self.pixels[pixel_row as usize].push(c);
    }

    fn sprite_is_visible(&self, sprite_position: i32, pixel_position: i32) -> bool {
        let pixel_row = pixel_position / self.num_cols;
        let normalized_pixel_position = pixel_position - (pixel_row * self.num_cols);
        (sprite_position - normalized_pixel_position).abs() <= 1
    }
}

fn process_instruction(state: &MachineState, instruction: Instruction) -> MachineState {
    match instruction {
        Instruction::AddX(x) => MachineState { register_value: state.register_value + x, cycle: state.cycle + 2 },
        Instruction::NoOp => MachineState { register_value: state.register_value, cycle: state.cycle + 1 },
    }
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input file");
    let (_, instructions) = parse_instructions(&input).expect("Should be able to parse instructions from input");

    let mut curr_state = MachineState { register_value: 1, cycle: 1 };
    let mut cycle_of_interest: Vec<i32> = (1..=240).rev().collect();
    let mut screen = Screen::new(6, 40);
    for instruction in instructions {
        let new_state = process_instruction(&curr_state, instruction);

        while !cycle_of_interest.is_empty() && cycle_of_interest.last().unwrap() < &new_state.cycle {
            let cycle = cycle_of_interest.pop().unwrap();
            let state_for_cycle = MachineState { cycle, register_value: curr_state.register_value };

            screen.draw(state_for_cycle);
        }

        curr_state = new_state;
    }

    for cycle in cycle_of_interest {
        let state_for_cycle = MachineState { cycle, register_value: curr_state.register_value };
        screen.draw(state_for_cycle);
    }

    for row in screen.pixels {
        println!("{:?}", row);
    }
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(newline, alt((parse_addx, parse_noop)))(input)?;
    Ok((input, instructions))
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("addx ")(input)?;
    let (input, x) = nom::character::complete::i32(input)?;
    Ok((input, Instruction::AddX(x)))
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Instruction::NoOp))
}
