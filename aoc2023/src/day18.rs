use std::ops::Div;

use aoc::aoc;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(Debug, Clone, Copy)]
struct Point(i64, i64);

impl Point {
    fn move_(self, dir: Dir, len: i64) -> Self {
        match dir {
            Dir::U => Point(self.0 - len, self.1),
            Dir::D => Point(self.0 + len, self.1),
            Dir::L => Point(self.0, self.1 - len),
            Dir::R => Point(self.0, self.1 + len),
        }
    }

    fn taxi_distance(self, p: &Point) -> i64 {
        (self.0 - p.0).abs() + (self.1 - p.1).abs()
    }
}

// Calculate are and perimeter
fn shoelace_formula(points: &[Point]) -> (i64, i64) {
    let (a2, p) = points
        .iter()
        .circular_tuple_windows()
        .map(|(p1, p2)| (p1.0 * p2.1 - p2.0 * p1.1, p1.taxi_distance(p2)))
        .fold((0, 0), |p1, p2| (p1.0 + p2.0, p1.1 + p2.1));
    (a2.abs().div(2), p)
}

#[aoc(day18, part1)]
pub fn part_1(input: &str) -> i64 {
    let input = input.lines().map(|l| {
        let mut splits = l.split_ascii_whitespace();
        let d = match splits.next().unwrap() {
            "U" => Dir::U,
            "D" => Dir::D,
            "L" => Dir::L,
            "R" => Dir::R,
            _ => unreachable!(),
        };
        let steps: i64 = splits.next().unwrap().parse().unwrap();
        (d, steps)
    });
    let mut points = vec![Point(0, 0)];
    let mut current = points[0];
    for (d, steps) in input {
        let next = current.move_(d, steps);
        points.push(next);
        current = next;
    }
    let (a, p) = shoelace_formula(&points);
    a + p.div(2) + 1
}

#[aoc(day18, part2)]
pub fn part_2(input: &str) -> i64 {
    let input = input.lines().map(|l| {
        let mut splits = l.split_ascii_whitespace();
        splits.next().unwrap();
        splits.next().unwrap();
        let hex = &splits.next().unwrap()[2..];
        let d = match &hex[5..6] {
            "0" => Dir::R,
            "1" => Dir::D,
            "2" => Dir::L,
            "3" => Dir::U,
            _ => unreachable!(),
        };
        (d, i64::from_str_radix(&hex[0..5], 16).unwrap())
    });
    let mut points = vec![Point(0, 0)];
    let mut current = points[0];
    for (d, steps) in input {
        let next = current.move_(d, steps);
        points.push(next);
        current = next;
    }
    let (a, p) = shoelace_formula(&points);
    a + p.div(2) + 1
}
