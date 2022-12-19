use nom::bytes::complete::tag;
use nom::character::complete::u8;
use nom::combinator::{all_consuming, map};
use nom::sequence::tuple;
use nom::IResult;

use super::get_input;

fn parse_blueprint(input: &str) -> IResult<&str, BluePrint> {
    map(
        all_consuming(tuple((
            tag("Blueprint "),
            u8,
            tag(": Each ore robot costs "),
            u8,
            tag(" ore. Each clay robot costs "),
            u8,
            tag(" ore. Each obsidian robot costs "),
            u8,
            tag(" ore and "),
            u8,
            tag(" clay. Each geode robot costs "),
            u8,
            tag(" ore and "),
            u8,
            tag(" obsidian."),
        ))),
        |(_, _, _, a, _, b, _, c, _, d, _, e, _, f, _)| BluePrint {
            ore_robot: a,
            clay_robot: b,
            obs_robot: (c, d),
            geode_robot: (e, f),
            max_ore: [a, b, c, e].into_iter().max().unwrap(),
        },
    )(input)
}

struct BluePrint {
    ore_robot: u8,
    clay_robot: u8,
    obs_robot: (u8, u8),
    geode_robot: (u8, u8),
    max_ore: u8,
}

#[derive(Default, Clone, Copy)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

fn maximize(
    bp: &BluePrint,
    mat: Resources,
    robots: Resources,
    ore_skipped: bool,
    clay_skipped: bool,
    obs_skipped: bool,
    time: u8,
) -> u8 {
    let can_build_ore = !ore_skipped && mat.ore >= bp.ore_robot && robots.ore < bp.max_ore;
    let can_build_clay = !clay_skipped && mat.ore >= bp.clay_robot && robots.clay < bp.obs_robot.1;
    let can_build_obs = !obs_skipped
        && mat.ore >= bp.obs_robot.0
        && mat.clay >= bp.obs_robot.1
        && robots.obsidian < bp.geode_robot.1;
    let can_build_geode = mat.ore >= bp.geode_robot.0 && mat.obsidian >= bp.geode_robot.1;

    let mut mat = mat;
    mat.ore += robots.ore;
    mat.clay += robots.clay;
    mat.obsidian += robots.obsidian;
    mat.geode += robots.geode;
    if time == 1 {
        return mat.geode;
    }
    if can_build_geode {
        let mut mat = mat;
        let mut robots = robots;
        mat.ore -= bp.geode_robot.0;
        mat.obsidian -= bp.geode_robot.1;
        robots.geode += 1;
        return maximize(bp, mat, robots, false, false, false, time - 1);
    }

    let mut best = 0;
    if can_build_obs {
        let mut mat = mat;
        let mut robots = robots;
        mat.ore -= bp.obs_robot.0;
        mat.clay -= bp.obs_robot.1;
        robots.obsidian += 1;
        best = maximize(bp, mat, robots, false, false, false, time - 1);
    }
    if can_build_clay {
        let mut mat = mat;
        let mut robots = robots;
        mat.ore -= bp.clay_robot;
        robots.clay += 1;
        best = best.max(maximize(bp, mat, robots, false, false, false, time - 1));
    }
    if can_build_ore {
        let mut mat = mat;
        let mut robots = robots;
        mat.ore -= bp.ore_robot;
        robots.ore += 1;
        best = best.max(maximize(bp, mat, robots, false, false, false, time - 1));
    }
    if !can_build_ore || !can_build_clay || !can_build_obs {
        best = best.max(maximize(
            bp,
            mat,
            robots,
            can_build_ore,
            can_build_clay,
            can_build_obs,
            time - 1,
        ));
    }
    best
}

pub fn day19(step: u8) -> usize {
    let input = get_input("input/day19.txt");
    let blueprints = input
        .lines()
        .map(|s| parse_blueprint(s).unwrap().1)
        .collect::<Vec<_>>();
    let mat = Resources::default();
    let robots = Resources {
        ore: 1,
        ..Default::default()
    };
    if step == 1 {
        let mut result = 0;
        for (i, bp) in blueprints.iter().enumerate() {
            result += maximize(bp, mat, robots, false, false, false, 24) as usize * (i + 1);
        }
        result
    } else {
        let mut result = 1;
        for bp in blueprints.iter().take(3) {
            result *= maximize(bp, mat, robots, false, false, false, 32) as usize;
        }
        result
    }
}
