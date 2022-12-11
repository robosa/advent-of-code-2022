use std::cmp::Reverse;

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

impl Monkey {
    pub fn new(str: &str) -> Self {
        let input = str.split('\n').collect::<Vec<_>>();
        let items = input[1]
            .strip_prefix("  Starting items: ")
            .expect("invalid input")
            .split(", ")
            .filter_map(|s| s.parse().ok())
            .collect();
        let op = match input[2]
            .strip_prefix("  Operation: new = old ")
            .and_then(|s| s.split_once(' '))
        {
            Some(("*", "old")) => Op::Square,
            Some(("*", n)) => Op::Mul(n.parse().expect("invalid input")),
            Some(("+", n)) => Op::Add(n.parse().expect("invalid input")),
            _ => panic!("invalid input"),
        };
        let divisor = input[3]
            .strip_prefix("  Test: divisible by ")
            .and_then(|s| s.parse().ok())
            .expect("invalid input");
        let idx_true = input[4]
            .strip_prefix("    If true: throw to monkey ")
            .and_then(|s| s.parse().ok())
            .expect("invalid input");
        let idx_false = input[5]
            .strip_prefix("    If false: throw to monkey ")
            .and_then(|s| s.parse().ok())
            .expect("invalid input");
        Self {
            items,
            op,
            divisor,
            idx_true,
            idx_false,
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
            };
            if step == 1 {
                item /= 3;
            }
            item %= max_worry;
            if item % self.divisor == 0 {
                Some((item, self.idx_true))
            } else {
                Some((item, self.idx_false))
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
