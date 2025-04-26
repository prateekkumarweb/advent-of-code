use std::iter::repeat_n;

use aoc::aoc;
use itertools::Itertools;

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> usize {
    let lines = input.lines();
    let mut result = 0;
    for line in lines {
        let mut splits = line.split(':').map(|p| p.trim());
        let amount = splits.next().unwrap().parse().unwrap();
        let rest = splits.next().unwrap();
        let vals = rest
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();
        let ops = ['+', '*'];
        for p in repeat_n(ops.iter(), vals.len() - 1).multi_cartesian_product() {
            let mut total = vals[0];
            for i in 0..p.len() {
                match p[i] {
                    '+' => total += vals[i + 1],
                    '*' => total *= vals[i + 1],
                    _ => unreachable!(),
                }
            }
            if total == amount {
                result += total;
                break;
            }
        }
    }
    result
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> usize {
    let lines = input.lines();
    let mut result = 0;
    for line in lines {
        let mut splits = line.split(':').map(|p| p.trim());
        let amount = splits.next().unwrap().parse().unwrap();
        let rest = splits.next().unwrap();
        let vals = rest
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();
        let ops = ['+', '*', '|'];
        for p in repeat_n(ops.iter(), vals.len() - 1).multi_cartesian_product() {
            let mut total = vals[0];
            for i in 0..p.len() {
                match p[i] {
                    '+' => total += vals[i + 1],
                    '*' => total *= vals[i + 1],
                    '|' => {
                        total = (total * 10usize.pow(vals[i + 1].checked_ilog10().unwrap_or(0) + 1))
                            + vals[i + 1]
                    }
                    _ => unreachable!(),
                }
            }
            if total == amount {
                result += total;
                break;
            }
        }
    }
    result
}
