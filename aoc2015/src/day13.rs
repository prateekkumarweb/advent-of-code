use std::collections::HashMap;

use itertools::Itertools;

#[aoc::aoc(day13, part1)]
fn part_1(input: &str) -> i32 {
    let input = input
        .lines()
        .map(|line| {
            let (person, rest) = line.split_once(' ').unwrap();
            let rest = rest.strip_prefix("would ").unwrap();
            let (lg, rest) = rest.split_once(' ').unwrap();
            let sign = if lg == "lose" { -1 } else { 1 };
            let (h, rest) = rest.split_once(' ').unwrap();
            let h = sign * h.parse::<i32>().unwrap();
            let rest = rest
                .strip_prefix("happiness units by sitting next to ")
                .unwrap();
            let other = rest.strip_suffix('.').unwrap();
            ((person, other), h)
        })
        .collect::<HashMap<_, _>>();
    let mut keys = input.keys().map(|&(p, _)| p).unique().collect::<Vec<_>>();
    let mut max_h = 0;
    let first = keys.pop().unwrap();
    let l = keys.len();
    for mut keys in keys.into_iter().permutations(l) {
        let mut current = first;
        let mut h = 0;
        while let Some(key) = keys.pop() {
            let h1 = *input.get(&(current, key)).unwrap();
            let h2 = *input.get(&(key, current)).unwrap();
            h += h1 + h2;
            current = key;
        }
        let h1 = *input.get(&(current, first)).unwrap();
        let h2 = *input.get(&(first, current)).unwrap();
        h += h1 + h2;
        max_h = max_h.max(h);
    }
    max_h
}

#[aoc::aoc(day13, part2)]
fn part_2(input: &str) -> i32 {
    let input = input
        .lines()
        .map(|line| {
            let (person, rest) = line.split_once(' ').unwrap();
            let rest = rest.strip_prefix("would ").unwrap();
            let (lg, rest) = rest.split_once(' ').unwrap();
            let sign = if lg == "lose" { -1 } else { 1 };
            let (h, rest) = rest.split_once(' ').unwrap();
            let h = sign * h.parse::<i32>().unwrap();
            let rest = rest
                .strip_prefix("happiness units by sitting next to ")
                .unwrap();
            let other = rest.strip_suffix('.').unwrap();
            ((person, other), h)
        })
        .collect::<HashMap<_, _>>();
    let keys = input.keys().map(|&(p, _)| p).unique().collect::<Vec<_>>();
    let mut max_h = 0;
    let l = keys.len();
    for keys in keys.into_iter().permutations(l) {
        let mut h = 0;
        for (k1, k2) in keys.into_iter().tuple_windows() {
            let h1 = *input.get(&(k1, k2)).unwrap();
            let h2 = *input.get(&(k2, k1)).unwrap();
            h += h1 + h2;
        }
        max_h = max_h.max(h);
    }
    max_h
}
