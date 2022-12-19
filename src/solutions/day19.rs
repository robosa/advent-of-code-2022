use nom::bytes::complete::tag;
use nom::character::complete::u16;
use nom::combinator::{all_consuming, map};
use nom::sequence::tuple;
use nom::IResult;

use super::get_input;

fn parse_blueprint(input: &str) -> IResult<&str, BluePrint> {
    map(
        all_consuming(tuple((
            tag("Blueprint "),
            u16,
            tag(": Each ore robot costs "),
            u16,
            tag(" ore. Each clay robot costs "),
            u16,
            tag(" ore. Each obsidian robot costs "),
            u16,
            tag(" ore and "),
            u16,
            tag(" clay. Each geode robot costs "),
            u16,
            tag(" ore and "),
            u16,
            tag(" obsidian."),
        ))),
        |(_, _, _, a, _, b, _, c, _, d, _, e, _, f, _)| BluePrint {
            ore_robot: a,
            clay_robot: b,
            obsidian_robot: (c, d),
            geode_robot: (e, f),
            max_ore: [a, b, c, e].into_iter().max().unwrap(),
        },
    )(input)
}

struct BluePrint {
    ore_robot: u16,
    clay_robot: u16,
    obsidian_robot: (u16, u16),
    geode_robot: (u16, u16),
    max_ore: u16,
}

#[derive(Default, Clone, Copy)]
struct State {
    ore: u16,
    ore_robot: u16,
    clay: u16,
    clay_robot: u16,
    obsidian: u16,
    obsidian_robot: u16,
    geode: u16,
    geode_robot: u16,
}

impl State {
    fn harvest(&mut self) {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
    }

    fn build_ore_robot(&mut self, bp: &BluePrint) -> u16 {
        let mut t = 1;
        while self.ore < bp.ore_robot {
            self.harvest();
            t += 1;
        }
        self.harvest();
        self.ore -= bp.ore_robot;
        self.ore_robot += 1;
        t
    }

    fn build_clay_robot(&mut self, bp: &BluePrint) -> u16 {
        let mut t = 1;
        while self.ore < bp.clay_robot {
            self.harvest();
            t += 1;
        }
        self.harvest();
        self.ore -= bp.clay_robot;
        self.clay_robot += 1;
        t
    }

    fn build_obsidian_robot(&mut self, bp: &BluePrint) -> u16 {
        let mut t = 1;
        while self.ore < bp.obsidian_robot.0 || self.clay < bp.obsidian_robot.1 {
            self.harvest();
            t += 1;
        }
        self.harvest();
        self.ore -= bp.obsidian_robot.0;
        self.clay -= bp.obsidian_robot.1;
        self.obsidian_robot += 1;
        t
    }

    fn build_geode_robot(&mut self, bp: &BluePrint) -> u16 {
        let mut t = 1;
        while self.ore < bp.geode_robot.0 || self.obsidian < bp.geode_robot.1 {
            self.harvest();
            t += 1;
        }
        self.harvest();
        self.ore -= bp.geode_robot.0;
        self.obsidian -= bp.geode_robot.1;
        self.geode_robot += 1;
        t
    }
}

fn maximize(bp: &BluePrint, time: u16) -> u16 {
    let initial_state = State {
        ore_robot: 1,
        ..Default::default()
    };
    let mut queue = Vec::from([(initial_state, time)]);
    let mut best = 0;
    while let Some((state, time)) = queue.pop() {
        let this_best = state.geode + time * state.geode_robot;
        best = best.max(this_best);
        if time == 1 || (state.geode + this_best + (time - 1) * time / 2) < best {
            continue;
        }
        if state.obsidian_robot > 0 {
            let mut new_state = state;
            let t = new_state.build_geode_robot(bp);
            if t < time {
                queue.push((new_state, time - t))
            }
        }
        if state.clay_robot > 0
            && state.obsidian_robot < bp.geode_robot.1
            && time > 3
            && state.obsidian < (time - 3) * (bp.geode_robot.1 - state.obsidian_robot)
        {
            let mut new_state = state;
            let t = new_state.build_obsidian_robot(bp);
            if t < time {
                queue.push((new_state, time - t))
            }
        }
        if state.clay_robot < bp.obsidian_robot.1
            && time > 5
            && state.clay < (time - 5) * (bp.obsidian_robot.1 - state.clay_robot)
        {
            let mut new_state = state;
            let t = new_state.build_clay_robot(bp);
            if t < time {
                queue.push((new_state, time - t))
            }
        }
        if state.ore_robot < bp.max_ore
            && time > 3
            && state.ore < (time - 3) * (bp.max_ore - state.ore_robot)
        {
            let mut state = state;
            let t = state.build_ore_robot(bp);
            if t < time {
                queue.push((state, time - t))
            }
        }
    }
    best
}

pub fn day19(step: u8) -> u16 {
    let input = get_input("input/day19.txt");
    let blueprints = input
        .lines()
        .map(|s| parse_blueprint(s).unwrap().1)
        .collect::<Vec<_>>();
    if step == 1 {
        let mut result = 0;
        for (i, bp) in blueprints.iter().enumerate() {
            result += maximize(bp, 24) * (i as u16 + 1);
        }
        result
    } else {
        let mut result = 1;
        for bp in blueprints.iter().take(3) {
            result *= maximize(bp, 32);
        }
        result
    }
}
