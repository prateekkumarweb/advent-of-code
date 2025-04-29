#![allow(clippy::needless_range_loop)]

use aoc::aoc;

#[aoc(day6, part1)]
pub fn part_1(input: &str) -> usize {
    let mut grid = vec![vec![0; 1000]; 1000];
    for line in input.lines() {
        if let Some(rest) = line.strip_prefix("turn on ") {
            let mut rest = rest.split_ascii_whitespace();
            let p1 = rest.next().unwrap().split_once(',').unwrap();
            let p1: (usize, usize) = (p1.0.parse().unwrap(), p1.1.parse().unwrap());
            rest.next().unwrap();
            let p2 = rest.next().unwrap().split_once(',').unwrap();
            let p2: (usize, usize) = (p2.0.parse().unwrap(), p2.1.parse().unwrap());
            for i in p1.0..=p2.0 {
                for j in p1.1..=p2.1 {
                    grid[i][j] = 1;
                }
            }
        }
        if let Some(rest) = line.strip_prefix("turn off ") {
            let mut rest = rest.split_ascii_whitespace();
            let p1 = rest.next().unwrap().split_once(',').unwrap();
            let p1: (usize, usize) = (p1.0.parse().unwrap(), p1.1.parse().unwrap());
            rest.next().unwrap();
            let p2 = rest.next().unwrap().split_once(',').unwrap();
            let p2: (usize, usize) = (p2.0.parse().unwrap(), p2.1.parse().unwrap());
            for i in p1.0..=p2.0 {
                for j in p1.1..=p2.1 {
                    grid[i][j] = 0;
                }
            }
        }
        if let Some(rest) = line.strip_prefix("toggle ") {
            let mut rest = rest.split_ascii_whitespace();
            let p1 = rest.next().unwrap().split_once(',').unwrap();
            let p1: (usize, usize) = (p1.0.parse().unwrap(), p1.1.parse().unwrap());
            rest.next().unwrap();
            let p2 = rest.next().unwrap().split_once(',').unwrap();
            let p2: (usize, usize) = (p2.0.parse().unwrap(), p2.1.parse().unwrap());
            for i in p1.0..=p2.0 {
                for j in p1.1..=p2.1 {
                    grid[i][j] = 1 - grid[i][j];
                }
            }
        }
    }
    grid.iter().map(|r| r.iter().sum::<usize>()).sum()
}

#[aoc(day6, part2)]
pub fn part_2(input: &str) -> usize {
    let mut grid = vec![vec![0; 1000]; 1000];
    for line in input.lines() {
        if let Some(rest) = line.strip_prefix("turn on ") {
            let mut rest = rest.split_ascii_whitespace();
            let p1 = rest.next().unwrap().split_once(',').unwrap();
            let p1: (usize, usize) = (p1.0.parse().unwrap(), p1.1.parse().unwrap());
            rest.next().unwrap();
            let p2 = rest.next().unwrap().split_once(',').unwrap();
            let p2: (usize, usize) = (p2.0.parse().unwrap(), p2.1.parse().unwrap());
            for i in p1.0..=p2.0 {
                for j in p1.1..=p2.1 {
                    grid[i][j] += 1;
                }
            }
        }
        if let Some(rest) = line.strip_prefix("turn off ") {
            let mut rest = rest.split_ascii_whitespace();
            let p1 = rest.next().unwrap().split_once(',').unwrap();
            let p1: (usize, usize) = (p1.0.parse().unwrap(), p1.1.parse().unwrap());
            rest.next().unwrap();
            let p2 = rest.next().unwrap().split_once(',').unwrap();
            let p2: (usize, usize) = (p2.0.parse().unwrap(), p2.1.parse().unwrap());
            for i in p1.0..=p2.0 {
                for j in p1.1..=p2.1 {
                    grid[i][j] = if grid[i][j] == 0 { 0 } else { grid[i][j] - 1 };
                }
            }
        }
        if let Some(rest) = line.strip_prefix("toggle ") {
            let mut rest = rest.split_ascii_whitespace();
            let p1 = rest.next().unwrap().split_once(',').unwrap();
            let p1: (usize, usize) = (p1.0.parse().unwrap(), p1.1.parse().unwrap());
            rest.next().unwrap();
            let p2 = rest.next().unwrap().split_once(',').unwrap();
            let p2: (usize, usize) = (p2.0.parse().unwrap(), p2.1.parse().unwrap());
            for i in p1.0..=p2.0 {
                for j in p1.1..=p2.1 {
                    grid[i][j] += 2;
                }
            }
        }
    }
    grid.iter().map(|r| r.iter().sum::<usize>()).sum()
}
