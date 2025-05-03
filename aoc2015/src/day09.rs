use std::collections::{HashMap, HashSet};

use aoc::aoc;
use itertools::Itertools;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u32 {
    let mut distances = HashMap::new();
    let mut cities = HashSet::new();
    for line in input.lines() {
        let (a_to_b, dist) = line.split_once(" = ").unwrap();
        let (a, b) = a_to_b.split_once(" to ").unwrap();
        let dist = dist.parse::<u32>().unwrap();
        cities.insert(a);
        cities.insert(b);
        distances.insert((a, b), dist);
        distances.insert((b, a), dist);
    }
    let mut min_dist = u32::MAX;
    for perm in cities.iter().permutations(cities.len()) {
        let mut d = 0;
        for (&a, &b) in perm.into_iter().tuple_windows() {
            d += distances.get(&(a, b)).unwrap();
        }
        min_dist = min_dist.min(d);
    }
    min_dist
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u32 {
    let mut distances = HashMap::new();
    let mut cities = HashSet::new();
    for line in input.lines() {
        let (a_to_b, dist) = line.split_once(" = ").unwrap();
        let (a, b) = a_to_b.split_once(" to ").unwrap();
        let dist = dist.parse::<u32>().unwrap();
        cities.insert(a);
        cities.insert(b);
        distances.insert((a, b), dist);
        distances.insert((b, a), dist);
    }
    let mut max_dist = u32::MIN;
    for perm in cities.iter().permutations(cities.len()) {
        let mut d = 0;
        for (&a, &b) in perm.into_iter().tuple_windows() {
            d += distances.get(&(a, b)).unwrap();
        }
        max_dist = max_dist.max(d);
    }
    max_dist
}
