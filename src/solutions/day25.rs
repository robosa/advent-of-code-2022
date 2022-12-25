use super::get_input;

fn snafu_to_int(number: &str) -> u64 {
    number.chars().fold(0, |acc, c| match c {
        '=' => acc * 5 - 2,
        '-' => acc * 5 - 1,
        '0' => acc * 5,
        '1' => acc * 5 + 1,
        '2' => acc * 5 + 2,
        _ => panic!(),
    })
}

fn int_to_snafu(number: u64) -> String {
    let (c, n) = match number % 5 {
        0 => ('0', number / 5),
        1 => ('1', number / 5),
        2 => ('2', number / 5),
        3 => ('=', number / 5 + 1),
        4 => ('-', number / 5 + 1),
        _ => unreachable!(),
    };
    if n != 0 {
        let mut res = int_to_snafu(n);
        res.push(c);
        res
    } else {
        String::from(c)
    }
}

pub fn day25(_: u8) -> String {
    int_to_snafu(get_input("input/day25.txt").lines().map(snafu_to_int).sum())
}
