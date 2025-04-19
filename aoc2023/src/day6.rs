#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]

use aoc_runner_derive::aoc;

fn find_num_wins(time: u64, distance: u64) -> u64 {
    let d = ((time * time - 4 * distance) as f64).sqrt();
    let l = (time as f64 - d) / 2.0;
    let u = (time as f64 + d) / 2.0;
    let mut l = l.ceil() as u64;
    if l * (time - l) == distance {
        l += 1;
    }
    let mut u = u.floor() as u64;
    if u * (time - u) == distance {
        u -= 1;
    }

    u - l + 1
}

#[aoc(day6, part1)]
pub fn part_1(input: &str) -> String {
    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap());

    let distances = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap());

    let product = times
        .zip(distances)
        .map(|(time, distance)| find_num_wins(time, distance))
        .product::<u64>();

    format!("{product}")
}

#[aoc(day6, part2)]
pub fn part_2(input: &str) -> String {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    format!("{}", find_num_wins(time, distance))
}
