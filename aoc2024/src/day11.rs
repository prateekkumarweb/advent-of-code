use aoc::aoc;
use itertools::Itertools;

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut stones = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    for _ in 0..25 {
        stones = stones
            .into_iter()
            .flat_map(|s| {
                if s == 0 {
                    vec![1]
                } else if (s.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
                    let n_digits = s.checked_ilog10().unwrap_or(0) + 1;
                    vec![s / 10usize.pow(n_digits / 2), s % 10usize.pow(n_digits / 2)]
                } else {
                    vec![s * 2024]
                }
                .into_iter()
            })
            .collect_vec();
    }
    stones.len()
}

use std::collections::HashMap;

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> usize {
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    let mut memo = HashMap::new();
    let blinks = 75;
    let result: usize = stones
        .into_iter()
        .map(|s| count(s, blinks, &mut memo))
        .sum();
    result
}

fn count(stone: usize, blinks: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    let result = memo.get(&(stone, blinks));
    if let Some(result) = result {
        return *result;
    }
    let result = if blinks == 0 {
        1
    } else if stone == 0 {
        count(1, blinks - 1, memo)
    } else if (stone.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
        let n_digits = stone.checked_ilog10().unwrap_or(0) + 1;
        count(stone / 10usize.pow(n_digits / 2), blinks - 1, memo)
            + count(stone % 10usize.pow(n_digits / 2), blinks - 1, memo)
    } else {
        count(stone * 2024, blinks - 1, memo)
    };
    memo.insert((stone, blinks), result);
    result
}
