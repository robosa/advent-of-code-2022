use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::{branch::alt, combinator::map, multi::many1, IResult};

use super::get_input;

#[derive(Debug)]
enum Command {
    Move(u32),
    TurnLeft,
    TurnRight,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn turn(dir: Direction, command: Command) -> Direction {
    match (dir, command) {
        (Direction::Up, Command::TurnLeft) => Direction::Left,
        (Direction::Up, Command::TurnRight) => Direction::Right,
        (Direction::Down, Command::TurnLeft) => Direction::Right,
        (Direction::Down, Command::TurnRight) => Direction::Left,
        (Direction::Left, Command::TurnLeft) => Direction::Down,
        (Direction::Left, Command::TurnRight) => Direction::Up,
        (Direction::Right, Command::TurnLeft) => Direction::Up,
        (Direction::Right, Command::TurnRight) => Direction::Down,
        _ => dir,
    }
}

fn parse_path(input: &str) -> IResult<&str, Vec<Command>> {
    many1(alt((
        map(u32, Command::Move),
        map(tag("L"), |_| Command::TurnLeft),
        map(tag("R"), |_| Command::TurnRight),
    )))(input)
}

fn move_right<F>(grid: &[Vec<char>], pos: (usize, usize), wrap: F) -> ((usize, usize), Direction)
where
    F: Fn((usize, usize), Direction) -> ((usize, usize), Direction),
{
    let (x, y) = pos;
    let line = &grid[y];
    let ((nx, ny), dir) = if x + 1 != line.len() && line[x + 1] != ' ' {
        ((x + 1, y), Direction::Right)
    } else {
        wrap(pos, Direction::Right)
    };
    if grid[ny][nx] == '.' {
        ((nx, ny), dir)
    } else {
        (pos, Direction::Right)
    }
}

fn move_left<F>(grid: &[Vec<char>], pos: (usize, usize), wrap: F) -> ((usize, usize), Direction)
where
    F: Fn((usize, usize), Direction) -> ((usize, usize), Direction),
{
    let (x, y) = pos;
    let line = &grid[y];
    let ((nx, ny), dir) = if x != 0 && line[x - 1] != ' ' {
        ((x - 1, y), Direction::Left)
    } else {
        wrap(pos, Direction::Left)
    };
    if grid[ny][nx] == '.' {
        ((nx, ny), dir)
    } else {
        (pos, Direction::Left)
    }
}

fn move_down<F>(grid: &[Vec<char>], pos: (usize, usize), wrap: F) -> ((usize, usize), Direction)
where
    F: Fn((usize, usize), Direction) -> ((usize, usize), Direction),
{
    let (x, y) = pos;
    let ((nx, ny), dir) = if y + 1 < grid.len() && x < grid[y + 1].len() && grid[y + 1][x] != ' ' {
        ((x, y + 1), Direction::Down)
    } else {
        wrap(pos, Direction::Down)
    };
    if grid[ny][nx] == '.' {
        ((nx, ny), dir)
    } else {
        (pos, Direction::Down)
    }
}

fn move_up<F>(grid: &[Vec<char>], pos: (usize, usize), wrap: F) -> ((usize, usize), Direction)
where
    F: Fn((usize, usize), Direction) -> ((usize, usize), Direction),
{
    let (x, y) = pos;
    let ((nx, ny), dir) = if y != 0 && x < grid[y - 1].len() && grid[y - 1][x] != ' ' {
        ((x, y - 1), Direction::Up)
    } else {
        wrap(pos, Direction::Up)
    };
    if grid[ny][nx] == '.' {
        ((nx, ny), dir)
    } else {
        (pos, Direction::Up)
    }
}

fn wrap_1(pos: (usize, usize), dir: Direction) -> ((usize, usize), Direction) {
    let (i, j) = (pos.0 / 50, pos.1 / 50);
    match (i, j, dir) {
        (1, 0, Direction::Up) => ((pos.0, 149), dir),
        (1, 0, Direction::Left) => ((149, pos.1), dir),
        (2, 0, Direction::Up) => ((pos.0, 49), dir),
        (2, 0, Direction::Right) => ((50, pos.1), dir),
        (2, 0, Direction::Down) => ((pos.0, 0), dir),
        (1, 1, Direction::Left) => ((99, pos.1), dir),
        (1, 1, Direction::Right) => ((50, pos.1), dir),
        (0, 2, Direction::Up) => ((pos.0, 199), dir),
        (0, 2, Direction::Left) => ((99, pos.1), dir),
        (1, 2, Direction::Right) => ((0, pos.1), dir),
        (1, 2, Direction::Down) => ((pos.0, 0), dir),
        (0, 3, Direction::Left) => ((49, pos.1), dir),
        (0, 3, Direction::Down) => ((pos.0, 100), dir),
        (0, 3, Direction::Right) => ((0, pos.1), dir),
        _ => panic!(),
    }
}

fn wrap_2(pos: (usize, usize), dir: Direction) -> ((usize, usize), Direction) {
    let (i, j) = (pos.0 / 50, pos.1 / 50);
    match (i, j, dir) {
        (1, 0, Direction::Up) => ((0, 100 + pos.0), Direction::Right),
        (1, 0, Direction::Left) => ((0, 149 - pos.1), Direction::Right),
        (2, 0, Direction::Up) => ((pos.0 - 100, 199), Direction::Up),
        (2, 0, Direction::Right) => ((99, 149 - pos.1), Direction::Left),
        (2, 0, Direction::Down) => ((99, pos.0 - 50), Direction::Left),
        (1, 1, Direction::Left) => ((pos.1 - 50, 100), Direction::Down),
        (1, 1, Direction::Right) => ((50 + pos.1, 49), Direction::Up),
        (0, 2, Direction::Up) => ((50, 50 + pos.0), Direction::Right),
        (0, 2, Direction::Left) => ((50, 149 - pos.1), Direction::Right),
        (1, 2, Direction::Right) => ((149, 149 - pos.1), Direction::Left),
        (1, 2, Direction::Down) => ((49, 100 + pos.0), Direction::Left),
        (0, 3, Direction::Left) => ((pos.1 - 100, 0), Direction::Down),
        (0, 3, Direction::Down) => ((pos.0 + 100, 0), Direction::Down),
        (0, 3, Direction::Right) => ((pos.1 - 100, 149), Direction::Up),
        _ => panic!(),
    }
}

fn move_dir<F>(
    grid: &[Vec<char>],
    pos: (usize, usize),
    dir: Direction,
    wrap: F,
) -> ((usize, usize), Direction)
where
    F: Fn((usize, usize), Direction) -> ((usize, usize), Direction),
{
    match dir {
        Direction::Up => move_up(grid, pos, wrap),
        Direction::Down => move_down(grid, pos, wrap),
        Direction::Left => move_left(grid, pos, wrap),
        Direction::Right => move_right(grid, pos, wrap),
    }
}

pub fn day22(step: u8) -> usize {
    let mut grid = Vec::new();
    let input = get_input("input/day22.txt");
    let (grid_str, path_str) = input.split_once("\n\n").unwrap();
    for line in grid_str.lines() {
        grid.push(line.chars().collect::<Vec<_>>())
    }
    let path = parse_path(path_str).unwrap().1;
    let mut dir = Direction::Right;
    let mut pos = (grid[0].iter().position(|&c| c != ' ').unwrap(), 0);
    let wrap = if step == 1 { wrap_1 } else { wrap_2 };
    for command in path {
        match command {
            Command::Move(i) => {
                for _ in 0..i {
                    (pos, dir) = move_dir(&grid, pos, dir, wrap);
                }
            }
            command => {
                dir = turn(dir, command);
            }
        }
    }
    1000 * (pos.1 + 1)
        + 4 * (pos.0 + 1)
        + match dir {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
}
