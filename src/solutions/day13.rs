use itertools::Itertools;
use nom::combinator::all_consuming;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::u8, combinator::map, error::Error,
    multi::separated_list0, sequence::delimited, Finish, IResult,
};
use std::cmp::Ordering;
use std::str::FromStr;

use super::get_input;

#[derive(PartialEq, Eq, Clone)]
enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), _) => a.cmp(&vec![other.clone()]),
            (_, Self::List(b)) => vec![self.clone()].cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Packet {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match all_consuming(parse_packet)(s).finish() {
            Ok((_, packet)) => Ok(packet),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(u8, Packet::Int),
        map(
            delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")),
            Packet::List,
        ),
    ))(input)
}

pub fn day13(step: u8) -> usize {
    let mut packets = get_input("input/day13.txt")
        .lines()
        .flat_map(|s| s.parse())
        .collect::<Vec<_>>();
    if step == 1 {
        packets
            .iter()
            .tuples()
            .enumerate()
            .filter(|(_, (a, b))| a <= b)
            .fold(0, |acc, (i, _)| acc + i + 1)
    } else {
        let div1 = "[[2]]".parse::<Packet>().unwrap();
        let div2 = "[[6]]".parse::<Packet>().unwrap();
        packets.push(div1.clone());
        packets.push(div2.clone());
        packets.sort_unstable();
        (packets.binary_search(&div1).unwrap() + 1) * (packets.binary_search(&div2).unwrap() + 1)
    }
}
