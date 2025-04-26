use aoc::aoc;

#[aoc(day2, part1)]
pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut splits = line.splitn(3, 'x');
            let a: u32 = splits.next().unwrap().parse().unwrap();
            let b: u32 = splits.next().unwrap().parse().unwrap();
            let c: u32 = splits.next().unwrap().parse().unwrap();
            let (ab, bc, ca) = (a * b, b * c, c * a);
            let min = ab.min(bc).min(ca);
            2 * (ab + bc + ca) + min
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut splits = line.splitn(3, 'x');
            let a: u32 = splits.next().unwrap().parse().unwrap();
            let b: u32 = splits.next().unwrap().parse().unwrap();
            let c: u32 = splits.next().unwrap().parse().unwrap();
            let max = a.max(b).max(c);
            let p = 2 * (a + b + c - max);
            let v = a * b * c;
            p + v
        })
        .sum()
}
