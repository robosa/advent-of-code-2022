use std::collections::HashMap;

use super::get_input;

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

enum Monkey {
    Op((Op, String, String)),
    Num(i64),
}

fn parse_monkey(s: &str) -> (String, Monkey) {
    let (name, op) = s.split_once(": ").unwrap();
    let monkey = match &op.split(' ').collect::<Vec<_>>()[..] {
        [s1, "+", s2] => Monkey::Op((Op::Add, s1.to_string(), s2.to_string())),
        [s1, "-", s2] => Monkey::Op((Op::Sub, s1.to_string(), s2.to_string())),
        [s1, "*", s2] => Monkey::Op((Op::Mul, s1.to_string(), s2.to_string())),
        [s1, "/", s2] => Monkey::Op((Op::Div, s1.to_string(), s2.to_string())),
        [s] => Monkey::Num(s.parse().unwrap()),
        _ => panic!(),
    };
    (name.to_string(), monkey)
}

fn find_human_path(monkeys: &HashMap<String, Monkey>, name: &str) -> Option<Vec<String>> {
    if name == "humn" {
        return Some(vec![name.to_string()]);
    };
    match monkeys.get(name).unwrap() {
        Monkey::Num(_) => None,
        Monkey::Op((_, s1, s2)) => {
            if let Some(mut v) = find_human_path(monkeys, s1) {
                v.push(name.to_string());
                Some(v)
            } else if let Some(mut v) = find_human_path(monkeys, s2) {
                v.push(name.to_string());
                Some(v)
            } else {
                None
            }
        }
    }
}

fn calc(monkeys: &HashMap<String, Monkey>, name: &str) -> i64 {
    match &monkeys[name] {
        Monkey::Num(i) => *i,
        Monkey::Op((Op::Add, s1, s2)) => calc(monkeys, s1) + calc(monkeys, s2),
        Monkey::Op((Op::Sub, s1, s2)) => calc(monkeys, s1) - calc(monkeys, s2),
        Monkey::Op((Op::Mul, s1, s2)) => calc(monkeys, s1) * calc(monkeys, s2),
        Monkey::Op((Op::Div, s1, s2)) => calc(monkeys, s1) / calc(monkeys, s2),
    }
}

fn solve(
    monkeys: &HashMap<String, Monkey>,
    human_path: &mut Vec<String>,
    name: &str,
    equal_to: i64,
) -> i64 {
    if name == "humn" {
        return equal_to;
    };
    let (op, left, right) = match &monkeys[name] {
        Monkey::Op((op, s1, s2)) => (op, s1, s2),
        _ => unreachable!(),
    };
    let is_human_left = left == &human_path.pop().unwrap();
    let (to_solve, other) = if is_human_left {
        (left, calc(monkeys, right))
    } else {
        (right, calc(monkeys, left))
    };
    match (op, is_human_left) {
        (Op::Add, _) => solve(monkeys, human_path, to_solve, equal_to - other),
        (Op::Sub, true) => solve(monkeys, human_path, to_solve, equal_to + other),
        (Op::Sub, false) => solve(monkeys, human_path, to_solve, other - equal_to),
        (Op::Mul, _) => solve(monkeys, human_path, to_solve, equal_to / other),
        (Op::Div, true) => solve(monkeys, human_path, to_solve, equal_to * other),
        (Op::Div, false) => solve(monkeys, human_path, to_solve, other / equal_to),
    }
}

pub fn day21(step: u8) -> i64 {
    let mut monkeys = HashMap::new();
    let input = get_input("input/day21.txt");
    for (name, monkey) in input.lines().map(parse_monkey) {
        monkeys.insert(name, monkey);
    }
    if step == 1 {
        calc(&monkeys, "root")
    } else {
        let mut human_path = find_human_path(&monkeys, "root").unwrap();
        human_path.pop();
        let (left, right) = match &monkeys["root"] {
            Monkey::Op((_, s1, s2)) => (s1, s2),
            _ => panic!(),
        };
        if left == &human_path.pop().unwrap() {
            let right_res = calc(&monkeys, right);
            solve(&monkeys, &mut human_path, left, right_res)
        } else {
            let left_res = calc(&monkeys, left);
            solve(&monkeys, &mut human_path, right, left_res)
        }
    }
}
