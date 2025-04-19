#![allow(clippy::cast_possible_truncation)]

use std::collections::HashMap;

use aoc_runner_derive::aoc;
use itertools::Itertools;

fn count_arrangement(input: &str, groups: &[usize]) -> usize {
    input.find('?').map_or_else(
        || {
            let new_groups = input
                .split('.')
                .map(str::len)
                .filter(|&x| x > 0)
                .collect_vec();
            usize::from(new_groups == groups)
        },
        |pos| {
            let mut new_input_1 = input.to_owned();
            new_input_1.replace_range(pos..=pos, ".");
            let c1 = count_arrangement(&new_input_1, groups);
            let mut new_input_2 = input.to_owned();
            new_input_2.replace_range(pos..=pos, "#");
            let count_arrangement2 = count_arrangement(&new_input_2, groups);
            c1 + count_arrangement2
        },
    )
}

#[aoc(day12, part1)]
pub fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut splits = line.split(' ');
            let springs = splits.next().unwrap();
            let groups = splits
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec();

            count_arrangement(springs, &groups)
        })
        .sum::<usize>()
        .to_string()
}

#[aoc(day12, part2)]
pub fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut splits = line.split(' ');
            let springs = splits.next().unwrap().chars().collect_vec();
            let springs = (0..5)
                .flat_map(|_| std::iter::once(&'?').chain(springs.iter()).copied())
                .skip(1)
                .collect_vec();
            let groups = splits
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec();
            let groups = (0..5).flat_map(|_| groups.iter().copied()).collect_vec();

            let mut memo = HashMap::new();
            count_arrangement2(&springs, &groups, &mut memo)
        })
        .sum::<usize>()
        .to_string()
}

fn count_arrangement2(
    springs: &[char],
    groups_needed: &[usize],
    memo: &mut HashMap<(String, String), usize>,
) -> usize {
    let springs_str = springs.iter().join("");
    let groups_str = groups_needed.iter().join(",");
    if let Some(&count) = memo.get(&(springs_str.clone(), groups_str.clone())) {
        return count;
    }

    if springs.is_empty() {
        if groups_needed.is_empty() {
            return 1;
        }
        return 0;
    }

    if groups_needed.is_empty() {
        if springs.iter().all(|&c| c == '.' || c == '?') {
            return 1;
        }
        return 0;
    }

    let answer = match springs[0] {
        '.' => count_arrangement2(&springs[1..], groups_needed, memo),
        '#' => {
            let first_group = groups_needed[0];
            if springs.len() >= first_group
                && (0..first_group).all(|i| springs[i] == '#' || springs[i] == '?')
            {
                let new_springs = &springs[first_group..];
                let new_groups = &groups_needed[1..];
                if new_springs.is_empty() {
                    usize::from(new_groups.is_empty())
                } else if new_springs[0] == '.' || new_springs[0] == '?' {
                    count_arrangement2(&new_springs[1..], new_groups, memo)
                } else {
                    0
                }
            } else {
                0
            }
        }
        '?' => {
            let count_if_dot = count_arrangement2(&springs[1..], groups_needed, memo);
            let first_group = groups_needed[0];
            count_if_dot
                + if springs.len() >= first_group
                    && (0..first_group).all(|i| springs[i] == '#' || springs[i] == '?')
                {
                    let new_springs = &springs[first_group..];
                    let new_groups = &groups_needed[1..];
                    if new_springs.is_empty() {
                        usize::from(new_groups.is_empty())
                    } else if new_springs[0] == '.' || new_springs[0] == '?' {
                        count_arrangement2(&new_springs[1..], new_groups, memo)
                    } else {
                        0
                    }
                } else {
                    0
                }
        }
        _ => unreachable!(),
    };

    memo.insert((springs_str, groups_str), answer);

    answer
}
