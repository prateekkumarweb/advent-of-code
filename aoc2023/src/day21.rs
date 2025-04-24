use std::{
    collections::{HashMap, VecDeque},
    usize,
};

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    E,
    R,
    S,
    O,
}

struct Grid {
    grid: Vec<Vec<Item>>,
}

impl Grid {
    fn n_rows(&self) -> usize {
        self.grid.len()
    }

    fn n_cols(&self) -> usize {
        self.grid[0].len()
    }

    fn neighbors(&self, cell: (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = vec![];
        if cell.0 > 0 {
            result.push((cell.0 - 1, cell.1));
        }
        if cell.1 > 0 {
            result.push((cell.0, cell.1 - 1));
        }
        if cell.0 + 1 < self.n_rows() {
            result.push((cell.0 + 1, cell.1));
        }
        if cell.1 + 1 < self.n_cols() {
            result.push((cell.0, cell.1 + 1));
        }
        result
    }

    fn count_steps(&mut self, max: usize) -> HashMap<(usize, usize), usize> {
        assert_eq!(self.n_cols(), self.n_rows());
        let n = self.n_rows();
        let mut start = (0, 0);
        'outer: for i in 0..n {
            for j in 0..n {
                if self.grid[i][j] == Item::S {
                    start = (i, j);
                    self.grid[i][j] = Item::E;
                    break 'outer;
                }
            }
        }
        let mut map = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        while let Some((p, d)) = queue.pop_front() {
            if !map.contains_key(&p) && d <= max {
                map.insert(p, d);
                queue.extend(
                    self.neighbors(p)
                        .iter()
                        .filter(|p| !map.contains_key(p) && self.grid[p.0][p.1] != Item::R)
                        .map(|&p| (p, d + 1)),
                );
            }
        }
        map
    }
}

#[aoc(day21, part1)]
pub fn part_1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Item::E,
                    '#' => Item::R,
                    'S' => Item::S,
                    'O' => Item::O,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();
    let mut grid = Grid { grid };
    grid.count_steps(64)
        .values()
        .filter(|&d| d % 2 == 0)
        .count()
}

#[aoc(day21, part2)]
pub fn part_2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Item::E,
                    '#' => Item::R,
                    'S' => Item::S,
                    'O' => Item::O,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();
    let mut grid = Grid { grid };
    // https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    let step_count = 26501365;
    let steps = grid.count_steps(usize::MAX);
    let odd_corners = steps.values().filter(|&p| p % 2 == 1 && *p > 65).count();
    let even_corners = steps.values().filter(|&p| p % 2 == 0 && *p > 65).count();
    let even_block = steps.values().filter(|&p| p % 2 == 0).count();
    let odd_block = steps.values().filter(|&p| p % 2 == 1).count();
    let n = (step_count - grid.n_rows() / 2) / grid.n_rows();
    assert_eq!(n, 202300);
    let even = n * n;
    let odd = (n + 1) * (n + 1);
    (odd * odd_block) + (even * even_block) - ((n + 1) * odd_corners) + (n * even_corners)
}
