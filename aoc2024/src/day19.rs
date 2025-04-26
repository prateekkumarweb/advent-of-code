use std::collections::HashMap;

use aoc::aoc;
use itertools::Itertools;

#[aoc(day19, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut lines = input.lines();
    let line = lines.next().unwrap();
    let mut patterns = line.trim().split(", ").collect_vec();
    patterns.sort_unstable();
    lines.next().unwrap();
    let mut count = 0;
    let mut cache = HashMap::new();
    let lines = lines.collect_vec();
    for design in &lines {
        if is_possible_design(design, &patterns, &mut cache) {
            count += 1;
        }
    }
    count
}

fn is_possible_design<'a, 'p>(
    design: &'a str,
    patterns: &'p [&'p str],
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if design.is_empty() {
        return true;
    }
    if let Some(&found) = cache.get(design) {
        return found;
    }
    for pat in patterns {
        if let Some(rest) = design.strip_prefix(pat) {
            let found = is_possible_design(rest, patterns, cache);
            if found {
                cache.insert(design, true);
                return true;
            }
        }
    }
    cache.insert(design, false);
    false
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut lines = input.lines();
    let line = lines.next().unwrap();
    let mut patterns = line.trim().split(", ").collect_vec();
    patterns.sort_unstable();
    lines.next().unwrap();
    let mut count = 0;
    let mut cache = HashMap::new();
    let lines = lines.collect_vec();
    for design in &lines {
        count += is_possible_design2(design, &patterns, &mut cache);
    }
    count
}

fn is_possible_design2<'a, 'p>(
    design: &'a str,
    patterns: &'p [&'p str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&found) = cache.get(design) {
        return found;
    }
    let mut count = 0;
    for pat in patterns {
        if let Some(rest) = design.strip_prefix(pat) {
            let found = is_possible_design2(rest, patterns, cache);
            count += found;
        }
    }
    cache.insert(design, count);
    count
}
