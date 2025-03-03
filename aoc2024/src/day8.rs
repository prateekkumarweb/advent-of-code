use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut antennas = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell.is_alphanumeric() {
                antennas.entry(cell).or_insert(vec![]).push((i, j));
            }
        }
    }
    let mut antinodes = HashSet::new();
    for (_, positions) in antennas {
        for (&p1, &p2) in positions.iter().tuple_combinations() {
            let (i1, j1) = p1;
            let (i2, j2) = p2;
            let i1 = i1 as isize;
            let j1 = j1 as isize;
            let i2 = i2 as isize;
            let j2 = j2 as isize;
            let a1 = (i1 - (i2 - i1), j1 - (j2 - j1));
            let a2 = (i2 + (i2 - i1), j2 + (j2 - j1));
            if a1.0 >= 0 && a1.1 >= 0 && a1.0 < grid.len() as isize && a1.1 < grid[0].len() as isize
            {
                antinodes.insert(a1);
            }
            if a2.0 >= 0 && a2.1 >= 0 && a2.0 < grid.len() as isize && a2.1 < grid[0].len() as isize
            {
                antinodes.insert(a2);
            }
        }
    }

    antinodes.len()
}

use num::Integer;

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut antennas = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell.is_alphanumeric() {
                antennas.entry(cell).or_insert(vec![]).push((i, j));
            }
        }
    }
    let mut antinodes = HashSet::new();
    for (_, positions) in antennas {
        for (&p1, &p2) in positions.iter().tuple_combinations() {
            let (i1, j1) = p1;
            let (i2, j2) = p2;
            let i1 = i1 as isize;
            let j1 = j1 as isize;
            let i2 = i2 as isize;
            let j2 = j2 as isize;
            let slope = (i2 - i1, j2 - j1);
            if slope.0 == 0 {
                for j in 0..grid[0].len() {
                    antinodes.insert((i1, j as isize));
                }
                continue;
            }
            if slope.1 == 0 {
                for i in 0..grid.len() {
                    antinodes.insert((i as isize, j1));
                }
                continue;
            }
            let gcd = slope.0.gcd(&slope.1);
            let slope = (slope.0 / gcd, slope.1 / gcd);
            for k in 0.. {
                let p = (i1 + k * slope.0, j1 + k * slope.1);
                if p.0 >= 0 && p.1 >= 0 && p.0 < grid.len() as isize && p.1 < grid[0].len() as isize
                {
                    antinodes.insert(p);
                } else {
                    break;
                }
            }
            for k in 0.. {
                let p = (i1 - k * slope.0, j1 - k * slope.1);
                if p.0 >= 0 && p.1 >= 0 && p.0 < grid.len() as isize && p.1 < grid[0].len() as isize
                {
                    antinodes.insert(p);
                } else {
                    break;
                }
            }
        }
    }

    antinodes.len()
}
