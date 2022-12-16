use std::{fs, cmp::Ordering};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::delimited,
    *,
};

#[derive(Debug)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

#[derive(PartialEq, Eq)]
enum CompareResult {
    Correct,
    Wrong,
    Continue,
}

fn compare_packets(left: &Packet, right: &Packet) -> CompareResult {
    match (left, right) {
        (Packet::Integer(i), Packet::Integer(j)) => {
            match i.cmp(j) {
                Ordering::Equal => CompareResult::Continue,
                Ordering::Greater => CompareResult::Wrong,
                Ordering::Less => CompareResult::Correct,
            }
        },
        (Packet::List(left_vals), Packet::List(right_vals)) => {
            for i in 0..left_vals.len() {
                if i >= right_vals.len() {
                    return CompareResult::Wrong
                }

                let r = compare_packets(&left_vals[i], &right_vals[i]);
                if r == CompareResult::Correct || r == CompareResult::Wrong {
                    return r;
                }
            }

            if left_vals.len() < right_vals.len() {
                CompareResult::Correct
            } else {
                CompareResult::Continue
            }
        },
        (Packet::Integer(i), Packet::List(_)) => {
            let list_val = Packet::List(vec![Packet::Integer(*i)]);
            compare_packets(&list_val, right)
        },
        (Packet::List(_), Packet::Integer(j)) => {
            let list_val = Packet::List(vec![Packet::Integer(*j)]);
            compare_packets(left, &list_val)
        }
    }
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input file");
    let (_, packet_pairs) = parse_packet_pairs(&input).expect("Should be able to parse input");

    let part_1_ans = (1..=packet_pairs.len())
        .filter(|idx| compare_packets(&packet_pairs[idx - 1].left, &packet_pairs[idx - 1].right) == CompareResult::Correct)
        .sum::<usize>();
    println!("{}", part_1_ans);

    let all_packets: Vec<&Packet> = packet_pairs.iter()
        .flat_map(|p| vec![&p.left, &p.right])
        .collect();
    
    let divider_1 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);

    let divider_1_idx = all_packets.iter()
        .filter(|p| compare_packets(p, &divider_1) == CompareResult::Correct)
        .count() + 1;
    let divider_2_idx = all_packets.iter()
        .filter(|p| compare_packets(p, &divider_2) == CompareResult::Correct)
        .count() + 2;

    let part_2_ans = divider_1_idx * divider_2_idx; 
    println!("{}", part_2_ans);
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]"))
            .map(Packet::List),
        nom::character::complete::u32
            .map(Packet::Integer)
    ))(input)
}

fn parse_packet_pair(input: &str) -> IResult<&str, PacketPair> {
    let (input, left) = parse_packet(input)?;
    let (input, _) = newline(input)?;
    let (input, right) = parse_packet(input)?;
    let (input, _) = newline(input)?;
    Ok((input, PacketPair { left, right }))
}

fn parse_packet_pairs(input: &str) -> IResult<&str, Vec<PacketPair>> {
    separated_list1(newline, parse_packet_pair)(input)
}

