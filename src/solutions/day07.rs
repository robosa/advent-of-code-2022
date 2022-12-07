use super::get_input;

fn get_sizes<'a, T>(lines: &mut T, sizes: &mut Vec<u64>) -> u64
where
    T: Iterator<Item = &'a str>,
{
    let mut size = 0;
    loop {
        match lines.next().unwrap_or("$ cd ..").rsplit_once(' ') {
            Some(("$ cd", "..")) => {
                sizes.push(size);
                return size;
            }
            Some(("$ cd", _)) => size += get_sizes(lines, sizes),
            Some((size_str, _)) => size += size_str.parse().unwrap_or(0),
            _ => (),
        }
    }
}

pub fn day07(step: u8) -> u64 {
    let data = get_input("input/day07.txt");
    let mut sizes = Vec::new();
    let needed_space = get_sizes(&mut data.lines().skip(1), &mut sizes) - 40000000;
    if step == 1 {
        sizes.iter().filter(|x| **x <= 100000).sum()
    } else {
        sizes
            .iter()
            .filter(|x| **x >= needed_space)
            .min()
            .copied()
            .unwrap_or(0)
    }
}
