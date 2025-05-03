use aoc::aoc;
use itertools::Itertools;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let mut input = input.to_string();
    for _ in 0..40 {
        let mut new_input = String::new();
        for (k, chunk) in &input.trim().chars().chunk_by(|c| *c) {
            new_input.push_str(&chunk.count().to_string());
            new_input.push(k);
        }
        input = new_input;
    }
    input.len()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let mut input = input.to_string();
    for _ in 0..50 {
        let mut new_input = String::new();
        for (k, chunk) in &input.trim().chars().chunk_by(|c| *c) {
            new_input.push_str(&chunk.count().to_string());
            new_input.push(k);
        }
        input = new_input;
    }
    input.len()
}
