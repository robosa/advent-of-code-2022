use super::get_input;

pub fn day06(step: u8) -> usize {
    let input = get_input("input/day06.txt").chars().collect::<Vec<_>>();
    let mut count = 1;
    let goal = if step == 1 { 4 } else { 14 };
    for i in 1..input.len() {
        count = 1 + (1..=count).take_while(|j| input[i - j] != input[i]).count();
        if count == goal {
            return i + 1;
        }
    }
    0
}
