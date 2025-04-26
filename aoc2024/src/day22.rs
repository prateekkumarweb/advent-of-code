use aoc::aoc;
use itertools::Itertools;

#[aoc(day22, part1)]
pub fn solve_part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();

    input.into_iter().map(gen_secret_number).sum::<usize>()
}

fn gen_secret_number(initial: usize) -> usize {
    let mut secret = initial;
    for _ in 0..2000 {
        secret = ((secret << 6) ^ secret) % 16777216;
        secret = ((secret >> 5) ^ secret) % 16777216;
        secret = ((secret << 11) ^ secret) % 16777216;
    }
    secret
}

use std::collections::{HashMap, HashSet};

const NSTEPS: usize = 2000;

#[aoc(day22, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let input = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();

    let result = input.into_iter().map(gen_secret_number2).collect_vec();

    let mut sum_prices = HashMap::new();

    for (prices, changes) in &result {
        let mut seen_changes = HashSet::new();
        for i in 3..NSTEPS {
            let change = (changes[i - 3], changes[i - 2], changes[i - 1], changes[i]);
            if !seen_changes.contains(&change) {
                seen_changes.insert(change);
                *sum_prices.entry(change).or_insert(0) += prices[i];
            }
        }
    }

    *sum_prices.values().max().unwrap()
}

fn gen_secret_number2(initial: usize) -> (Vec<i64>, Vec<i64>) {
    let mut secret = initial;
    let mut prices = vec![];
    let mut changes = vec![];
    for _ in 0..NSTEPS {
        let prev = secret;
        secret = ((secret << 6) ^ secret) % 16777216;
        secret = ((secret >> 5) ^ secret) % 16777216;
        secret = ((secret << 11) ^ secret) % 16777216;
        prices.push((secret % 10) as i64);
        changes.push((secret % 10) as i64 - (prev % 10) as i64);
    }
    (prices, changes)
}
