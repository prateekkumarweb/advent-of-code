use std::collections::HashMap;

use aoc_runner_derive::aoc;
use itertools::Itertools;
use regex::Regex;

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let moves = lines.next().unwrap().chars().collect_vec();
    lines.next().unwrap();
    let re = Regex::new(r"([A-Z0-9][A-Z0-9][A-Z0-9]) \= \(([A-Z0-9][A-Z0-9][A-Z0-9])\, ([A-Z0-9][A-Z0-9][A-Z0-9])\)").unwrap();
    let map = lines
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let start = caps.get(1).unwrap().as_str();
            let left = caps.get(2).unwrap().as_str();
            let right = caps.get(3).unwrap().as_str();
            (start, (left, right))
        })
        .collect::<HashMap<_, _>>();
    (moves, map)
}

fn find_steps<F>(start: &str, moves: &[char], map: &HashMap<&str, (&str, &str)>, is_end: F) -> u64
where
    F: Fn(&str) -> bool,
{
    let mut steps = 0;
    let mut current = start;
    loop {
        for m in moves {
            let (left, right) = map.get(current).unwrap();
            if *m == 'L' {
                current = left;
                steps += 1;
            } else if *m == 'R' {
                current = right;
                steps += 1;
            }
            if is_end(current) {
                return steps;
            }
        }
    }
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    // Copied from https://gist.github.com/victor-iyi/8a84185c1d52419b0d4915a648d5e3e1
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[aoc(day8, part1)]
pub fn part_1(input: &str) -> String {
    let (moves, map) = parse_input(input);

    find_steps("AAA", &moves, &map, |x| x == "ZZZ").to_string()
}

#[aoc(day8, part2)]
pub fn part_2(input: &str) -> String {
    let (moves, map) = parse_input(input);

    let start_vec = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .copied()
        .collect_vec();

    let steps_vec = start_vec
        .iter()
        .map(|start| find_steps(start, &moves, &map, |x| x.ends_with('Z')))
        .collect_vec();

    let lcm = steps_vec.iter().fold(1, |acc, x| {
        let gcd = gcd(acc, *x);
        acc * x / gcd
    });

    lcm.to_string()
}
