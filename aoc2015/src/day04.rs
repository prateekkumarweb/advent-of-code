use aoc::aoc;

#[aoc(day4, part1)]
pub fn part_1(input: &str) -> usize {
    for i in 1.. {
        let s = format!("{}{}", input.trim(), i);
        let d = md5::compute(s.as_bytes());
        let d = format!("{:x}", d);
        if d.starts_with("00000") {
            return i;
        }
    }
    unreachable!()
}

#[aoc(day4, part2)]
pub fn part_2(input: &str) -> usize {
    for i in 1.. {
        let s = format!("{}{}", input.trim(), i);
        let d = md5::compute(s.as_bytes());
        let d = format!("{:x}", d);
        if d.starts_with("000000") {
            return i;
        }
    }
    unreachable!()
}
