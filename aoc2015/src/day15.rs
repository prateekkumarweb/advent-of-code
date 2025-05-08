use std::ops::Add;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Ingredient {
    cap: i32,
    dur: i32,
    fla: i32,
    tex: i32,
    cal: i32,
}

impl Add for Ingredient {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cap: self.cap + rhs.cap,
            dur: self.dur + rhs.dur,
            fla: self.fla + rhs.fla,
            tex: self.tex + rhs.tex,
            cal: self.cal + rhs.cal,
        }
    }
}

impl Ingredient {
    fn times(self, mul: i32) -> Self {
        Self {
            cap: self.cap * mul,
            dur: self.dur * mul,
            fla: self.fla * mul,
            tex: self.tex * mul,
            cal: self.cal * mul,
        }
    }

    fn max_0(self) -> Self {
        Self {
            cap: self.cap.max(0),
            dur: self.dur.max(0),
            fla: self.fla.max(0),
            tex: self.tex.max(0),
            cal: self.cal,
        }
    }

    fn val(self) -> i32 {
        self.cap * self.dur * self.fla * self.tex
    }
}

#[aoc::aoc(day15, part1)]
fn part_1(input: &str) -> i32 {
    let rg = Regex::new("^\\w+: capacity (-?\\d+), durability (-?\\d+), flavor (-?\\d+), texture (-?\\d+), calories (\\d+)$").unwrap();
    let input = input
        .lines()
        .map(|line| {
            let mat = rg.captures(line).unwrap();
            Ingredient {
                cap: mat[1].parse().unwrap(),
                dur: mat[2].parse().unwrap(),
                fla: mat[3].parse().unwrap(),
                tex: mat[4].parse().unwrap(),
                cal: mat[5].parse().unwrap(),
            }
        })
        .collect_vec();
    const MAX: i32 = 100;

    (0..=MAX)
        .flat_map(|a| {
            (0..=(MAX - a))
                .flat_map(move |b| (0..=(MAX - a - b)).map(move |c| (a, b, c, MAX - a - b - c)))
        })
        .map(|(a, b, c, d)| {
            input[0].times(a) + input[1].times(b) + input[2].times(c) + input[3].times(d)
        })
        .map(|i| i.max_0().val())
        .max()
        .unwrap()
}

#[aoc::aoc(day15, part2)]
fn part_2(input: &str) -> i32 {
    let rg = Regex::new("^\\w+: capacity (-?\\d+), durability (-?\\d+), flavor (-?\\d+), texture (-?\\d+), calories (\\d+)$").unwrap();
    let input = input
        .lines()
        .map(|line| {
            let mat = rg.captures(line).unwrap();
            Ingredient {
                cap: mat[1].parse().unwrap(),
                dur: mat[2].parse().unwrap(),
                fla: mat[3].parse().unwrap(),
                tex: mat[4].parse().unwrap(),
                cal: mat[5].parse().unwrap(),
            }
        })
        .collect_vec();
    const MAX: i32 = 100;

    (0..=MAX)
        .flat_map(|a| {
            (0..=(MAX - a))
                .flat_map(move |b| (0..=(MAX - a - b)).map(move |c| (a, b, c, MAX - a - b - c)))
        })
        .map(|(a, b, c, d)| {
            input[0].times(a) + input[1].times(b) + input[2].times(c) + input[3].times(d)
        })
        .filter(|i| i.cal == 500)
        .map(|i| i.max_0().val())
        .max()
        .unwrap()
}
