use super::get_input;

fn fill_init(line: &str, stacks: &mut [Vec<char>]) {
    for (i, c) in line.chars().enumerate() {
        if i % 4 == 1 && c != ' ' {
            stacks[i / 4].push(c)
        }
    }
}

fn parse_command(command: &str) -> Option<(usize, (usize, usize))> {
    command.split_once(" from ").and_then(|(s1, s2)| {
        s1.strip_prefix("move ").and_then(|s| s.parse().ok()).zip(
            s2.split_once(" to ")
                .and_then(|(from, to)| (from.parse().ok().zip(to.parse().ok()))),
        )
    })
}

fn execute_command(command: &str, stacks: &mut [Vec<char>], step: u8) {
    if let Some((count, (from, to))) = parse_command(command) {
        if step == 1 {
            for _ in 0..count {
                if let Some(item) = stacks[from - 1].pop() {
                    stacks[to - 1].push(item);
                }
            }
        } else {
            let split_idx = stacks[from - 1].len() - count;
            let mut items = stacks[from - 1].split_off(split_idx);
            stacks[to - 1].append(&mut items);
        }
    }
}

pub fn day05(step: u8) -> String {
    let data = get_input("input/day05.txt");
    let (init, commands) = data.split_once("\n\n").expect("invalid input");
    let mut init_iter = init.lines().rev();
    let mut stacks = vec![Vec::new(); (init_iter.next().expect("invalid input").len() + 1) / 4];
    for line in init_iter {
        fill_init(line, &mut stacks);
    }
    for command in commands.lines() {
        execute_command(command, &mut stacks, step);
    }
    stacks.iter().filter_map(|stack| stack.last()).collect()
}
