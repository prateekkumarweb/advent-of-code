use std::collections::HashMap;

use aoc::aoc;
use itertools::Itertools;

fn is_nice(s: &str) -> bool {
    let v_count = s.chars().filter(|&c| "aeiou".contains(c)).count();
    if v_count < 3 {
        return false;
    }
    if !s.chars().tuple_windows().any(|(a, b)| a == b) {
        return false;
    }
    if s.chars().tuple_windows().any(|(a, b)| {
        ["ab", "cd", "pq", "xy"]
            .iter()
            .contains(&format!("{a}{b}").as_str())
    }) {
        return false;
    }
    true
}

#[aoc(day5, part1)]
pub fn part_1(input: &str) -> usize {
    input.lines().filter(|l| is_nice(l)).count()
}

fn is_nice2(s: &str) -> bool {
    let mut map = HashMap::new();
    let tup = s.chars().tuple_windows::<(char, char)>().enumerate();
    for (i, (a, b)) in tup {
        let entry = map.entry((a, b)).or_insert_with(Vec::new);
        entry.push(i);
    }
    let mut repeated = false;
    for (_, v) in map {
        if v.len() == 2 && v[0] + 1 != v[1] {
            repeated = true;
            break;
        }
        if v.len() > 2 {
            repeated = true;
            break;
        }
    }
    if !repeated {
        return false;
    }
    for (a, _, c) in s.chars().tuple_windows() {
        if a == c {
            return true;
        }
    }
    false
}

#[aoc(day5, part2)]
pub fn part_2(input: &str) -> usize {
    input.lines().filter(|l| is_nice2(l)).count()
}
