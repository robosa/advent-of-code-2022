use super::get_input;

struct File {
    prev_next: Vec<(usize, usize)>,
    offset: Vec<usize>,
}

impl File {
    fn init(data: &[i64]) -> Self {
        let len = data.len();
        let prev_next = (0..len)
            .into_iter()
            .map(|i| match i {
                0 => (len - 1, 1),
                i => (i - 1, (i + 1) % len),
            })
            .collect();
        let offset = data
            .iter()
            .map(|x| {
                let x_mod = x % (len - 1) as i64;
                if x_mod < 0 {
                    len - 1 - x_mod.unsigned_abs() as usize
                } else {
                    x_mod as usize
                }
            })
            .collect();
        Self { prev_next, offset }
    }

    fn mix(&mut self) {
        for (i, &x) in self.offset.iter().enumerate() {
            let (mut prev, mut next) = self.prev_next[i];
            self.prev_next[prev].1 = next;
            self.prev_next[next].0 = prev;
            for _ in 0..x {
                next = self.prev_next[next].1;
            }
            prev = self.prev_next[next].0;
            self.prev_next[i] = (prev, next);
            self.prev_next[prev].1 = i;
            self.prev_next[next].0 = i;
        }
    }
}

pub fn day20(step: u8) -> i64 {
    let data = get_input("input/day20.txt")
        .lines()
        .flat_map(|s| s.parse::<i64>())
        .map(|x| if step == 2 { x * 811589153 } else { x })
        .collect::<Vec<_>>();
    let mut file = File::init(&data);
    let times = if step == 2 { 10 } else { 1 };
    for _ in 0..times {
        file.mix();
    }
    let (mut next, _) = data.iter().enumerate().find(|(_, &x)| x == 0).unwrap();
    let mut res = 0;
    for i in 1..=3000 {
        next = file.prev_next[next].1;
        if i % 1000 == 0 {
            res += data[next];
        }
    }
    res
}
