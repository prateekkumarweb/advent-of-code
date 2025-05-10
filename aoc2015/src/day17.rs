use std::collections::HashMap;

use itertools::Itertools;

fn n_ways(containers: &[u32], cap: u32) -> u32 {
    if containers.is_empty() {
        return u32::from(cap == 0);
    }
    let (first, rest) = (containers[0], &containers[1..]);
    if first <= cap {
        n_ways(rest, cap - first) + n_ways(rest, cap)
    } else {
        n_ways(rest, cap)
    }
}

#[aoc::aoc(day17, part1)]
fn part_1(input: &str) -> u32 {
    const CAP: u32 = 150;
    let containers = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect_vec();
    n_ways(&containers, CAP)
}

fn n_ways2(containers: &[u32], cap: u32, n: u32, map: &mut HashMap<u32, u32>) -> u32 {
    if containers.is_empty() {
        return if cap == 0 {
            let entry = map.entry(n + 1).or_default();
            *entry += 1;
            1
        } else {
            0
        };
    }
    let (first, rest) = (containers[0], &containers[1..]);
    if first <= cap {
        n_ways2(rest, cap - first, n + 1, map) + n_ways2(rest, cap, n, map)
    } else {
        n_ways2(rest, cap, n, map)
    }
}

#[aoc::aoc(day17, part2)]
fn part_2(input: &str) -> u32 {
    const CAP: u32 = 150;
    let containers = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect_vec();
    let mut map = HashMap::new();
    n_ways2(&containers, CAP, 0, &mut map);
    *map.iter().min_by(|a, b| a.0.cmp(b.0)).unwrap().1
}
