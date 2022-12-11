use std::{cmp::Reverse, str::FromStr};

use super::get_input;

enum Op {
    Square,
    Mul(u64),
    Add(u64),
}

struct Monkey {
    items: Vec<u64>,
    op: Op,
    divisor: u64,
    idx_true: usize,
    idx_false: usize,
    count: usize,
}

fn parse_items(str: &str) -> Option<Vec<u64>> {
    str.strip_prefix("  Starting items: ")
        .and_then(|s| s.split(", ").map(|item| item.parse().ok()).collect())
}

fn parse_op(str: &str) -> Option<Op> {
    str.strip_prefix("  Operation: new = old ").and_then(|s| {
        s.split_once(' ').and_then(|op| match op {
            ("*", "old") => Some(Op::Square),
            ("*", n) => n.parse().ok().map(Op::Mul),
            ("+", n) => n.parse().ok().map(Op::Add),
            _ => None,
        })
    })
}

fn parse_any<T: FromStr>(str: &str) -> Option<T> {
    str.rsplit_once(' ').and_then(|(_, val)| val.parse().ok())
}

impl Monkey {
    pub fn new(str: &str) -> Self {
        let mut iter = str.split('\n').skip(1);
        Self {
            items: iter.next().and_then(parse_items).expect("invalid items"),
            op: iter.next().and_then(parse_op).expect("invalid op"),
            divisor: iter.next().and_then(parse_any).expect("invalid divisor"),
            idx_true: iter.next().and_then(parse_any).expect("invalid true idx"),
            idx_false: iter.next().and_then(parse_any).expect("invalid false idx"),
            count: 0,
        }
    }

    fn throw(&mut self, max_worry: u64, step: u8) -> Option<(u64, usize)> {
        if let Some(mut item) = self.items.pop() {
            self.count += 1;
            item = match self.op {
                Op::Square => item * item,
                Op::Mul(n) => item * n,
                Op::Add(n) => item + n,
            } % max_worry;
            if step == 1 {
                item /= 3;
            }
            match item % self.divisor {
                0 => Some((item, self.idx_true)),
                _ => Some((item, self.idx_false)),
            }
        } else {
            None
        }
    }
}

pub fn day11(step: u8) -> usize {
    let mut monkeys = get_input("input/day11.txt")
        .split("\n\n")
        .map(Monkey::new)
        .collect::<Vec<_>>();
    let max_worry = monkeys.iter().map(|m| m.divisor).product();
    let rounds = if step == 1 { 20 } else { 10000 };
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some((item, idx)) = monkeys[i].throw(max_worry, step) {
                monkeys[idx].items.push(item);
            }
        }
    }
    monkeys.sort_unstable_by_key(|m| Reverse(m.count));
    monkeys[0].count * monkeys[1].count
}
