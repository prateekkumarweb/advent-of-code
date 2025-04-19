use std::{
    collections::{HashMap, VecDeque},
    u32,
};

use aoc_runner_derive::aoc;

struct Grid {
    grid: Vec<Vec<u32>>,
    n_rows: usize,
    n_cols: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    const fn left(self) -> Self {
        match self {
            Self::N => Self::E,
            Self::S => Self::W,
            Self::E => Self::S,
            Self::W => Self::N,
        }
    }

    const fn right(self) -> Self {
        match self {
            Self::N => Self::W,
            Self::S => Self::E,
            Self::E => Self::N,
            Self::W => Self::S,
        }
    }
}

impl Grid {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<u32>> = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let n_rows = grid.len();
        let n_cols = grid[0].len();
        Self {
            grid,
            n_rows,
            n_cols,
        }
    }

    fn at(&self, cell: (usize, usize)) -> u32 {
        self.grid[cell.0][cell.1]
    }

    const fn next_cell(&self, cell: (usize, usize), dir: Dir) -> Option<(usize, usize)> {
        match dir {
            Dir::N => {
                if cell.0 > 0 {
                    Some((cell.0 - 1, cell.1))
                } else {
                    None
                }
            }
            Dir::W => {
                if cell.1 + 1 < self.n_cols {
                    Some((cell.0, cell.1 + 1))
                } else {
                    None
                }
            }
            Dir::S => {
                if cell.0 + 1 < self.n_rows {
                    Some((cell.0 + 1, cell.1))
                } else {
                    None
                }
            }
            Dir::E => {
                if cell.1 > 0 {
                    Some((cell.0, cell.1 - 1))
                } else {
                    None
                }
            }
        }
    }
}

fn possible_next(
    grid: &Grid,
    mut start: (usize, usize),
    dir: Dir,
) -> Vec<((usize, usize), Dir, u32)> {
    let mut result = vec![];
    let mut cost = 0;
    for _ in 0..3 {
        let next = grid.next_cell(start, dir);
        if let Some(next) = next {
            start = next;
            cost += grid.at(start);
            result.push((start, dir.left(), cost));
            result.push((start, dir.right(), cost));
        } else {
            break;
        }
    }
    result
}

fn possible_next2(
    grid: &Grid,
    mut start: (usize, usize),
    dir: Dir,
) -> Vec<((usize, usize), Dir, u32)> {
    let mut result = vec![];
    let mut cost = 0;
    for i in 0..10 {
        let next = grid.next_cell(start, dir);
        if let Some(next) = next {
            start = next;
            cost += grid.at(start);
            if i >= 3 {
                result.push((start, dir.left(), cost));
                result.push((start, dir.right(), cost));
            }
        } else {
            break;
        }
    }
    result
}

#[aoc(day17, part1)]
pub fn part_1(input: &str) -> u32 {
    let grid = Grid::from(input);
    let mut memo: HashMap<((usize, usize), Dir), u32> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), Dir::S));
    queue.push_back(((0, 0), Dir::W));
    let mut min = u32::MAX;
    let end = (grid.n_rows - 1, grid.n_cols - 1);
    loop {
        let Some((cell, dir)) = queue.pop_front() else {
            break;
        };
        let cost = memo.get(&(cell, dir)).copied().unwrap_or(0);
        if cell == end {
            min = min.min(cost);
            continue;
        }
        let next = possible_next(&grid, cell, dir);
        for (cell, dir, c) in next {
            let old_cost = memo.get(&(cell, dir)).copied();
            if let Some(old_cost) = old_cost {
                if cost + c < old_cost {
                    memo.insert((cell, dir), cost + c);
                    queue.push_back((cell, dir));
                }
            } else {
                memo.insert((cell, dir), cost + c);
                queue.push_back((cell, dir));
            }
        }
    }
    min
}

#[aoc(day17, part2)]
pub fn part_2(input: &str) -> u32 {
    let grid = Grid::from(input);
    let mut memo: HashMap<((usize, usize), Dir), u32> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), Dir::S));
    queue.push_back(((0, 0), Dir::W));
    let mut min = u32::MAX;
    let end = (grid.n_rows - 1, grid.n_cols - 1);
    loop {
        let Some((cell, dir)) = queue.pop_front() else {
            break;
        };
        let cost = memo.get(&(cell, dir)).copied().unwrap_or(0);
        if cell == end {
            min = min.min(cost);
            continue;
        }
        let next = possible_next2(&grid, cell, dir);
        for (cell, dir, c) in next {
            let old_cost = memo.get(&(cell, dir)).copied();
            if let Some(old_cost) = old_cost {
                if cost + c < old_cost {
                    memo.insert((cell, dir), cost + c);
                    queue.push_back((cell, dir));
                }
            } else {
                memo.insert((cell, dir), cost + c);
                queue.push_back((cell, dir));
            }
        }
    }
    min
}
