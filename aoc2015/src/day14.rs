use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Rd {
    speed: usize,
    fly_time: usize,
    rest_time: usize,
}

#[aoc::aoc(day14, part1)]
fn part_1(input: &str) -> usize {
    let rg =
        Regex::new("([a-zA-Z0-9]+) can fly ([0-9]+) km/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds.")
            .unwrap();
    let time = 2503;
    input
        .lines()
        .map(|line| {
            let mat = rg.captures(line).unwrap();
            Rd {
                speed: mat[2].parse().unwrap(),
                fly_time: mat[3].parse().unwrap(),
                rest_time: mat[4].parse().unwrap(),
            }
        })
        .map(|rd| {
            let t = rd.fly_time + rd.rest_time;
            let mut d = time / t;
            let mut r = time % t;
            if r >= rd.fly_time {
                d += 1;
                r = 0;
            }
            d * rd.fly_time * rd.speed + r * rd.speed
        })
        .max()
        .unwrap()
}

#[derive(Debug, Clone, Copy)]
enum Current {
    Fly(usize),
    Rest(usize),
}

impl Current {
    const fn next(self, fly_time: usize, rest_time: usize) -> Self {
        match self {
            Self::Fly(t) => {
                if t == 0 {
                    Self::Rest(rest_time - 1)
                } else {
                    Self::Fly(t - 1)
                }
            }
            Self::Rest(t) => {
                if t == 0 {
                    Self::Fly(fly_time - 1)
                } else {
                    Self::Rest(t - 1)
                }
            }
        }
    }
}

#[aoc::aoc(day14, part2)]
fn part_2(input: &str) -> usize {
    let rg =
        Regex::new("([a-zA-Z0-9]+) can fly ([0-9]+) km/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds.")
            .unwrap();
    let time = 2503;
    let mut rds = input
        .lines()
        .map(|line| {
            let mat = rg.captures(line).unwrap();
            Rd {
                speed: mat[2].parse().unwrap(),
                fly_time: mat[3].parse().unwrap(),
                rest_time: mat[4].parse().unwrap(),
            }
        })
        .map(|rd| (rd, Current::Fly(rd.fly_time - 1), 0, 0))
        .collect_vec();
    for _ in 0..time {
        for rd in &mut rds {
            rd.2 += match rd.1 {
                Current::Fly(_) => rd.0.speed,
                Current::Rest(_) => 0,
            };
            rd.1 = rd.1.next(rd.0.fly_time, rd.0.rest_time);
        }
        let max = rds.iter().map(|r| r.2).max().unwrap();
        for rd in &mut rds {
            if rd.2 == max {
                rd.3 += 1;
            }
        }
    }
    rds.into_iter().map(|x| x.3).max().unwrap()
}
