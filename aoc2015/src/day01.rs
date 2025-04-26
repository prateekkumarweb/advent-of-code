use aoc::aoc;

#[aoc(day1, part1)]
pub fn part_1(input: &str) -> i64 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part_2(input: &str) -> usize {
    let mut sum = 0;
    for (i, c) in input.chars().enumerate() {
        sum += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if sum == -1 {
            return i + 1;
        }
    }
    0
}
