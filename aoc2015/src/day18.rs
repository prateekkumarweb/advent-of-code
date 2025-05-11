#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum State {
    Off,
    On,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<State>>,
}

impl Grid {
    fn at(&self, (x, y): (usize, usize)) -> State {
        self.grid[x][y]
    }

    fn set(&mut self, (x, y): (usize, usize), state: State) {
        self.grid[x][y] = state;
    }

    fn neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let size = self.grid.len() as i64;
        let mut ns = vec![];
        for (dx, dy) in [-1, 0, 1]
            .into_iter()
            .cartesian_product([-1, 0, 1].into_iter())
        {
            if dx == 0 && dy == 0 {
                continue;
            }
            let x1 = x as i64 + dx;
            let y1 = y as i64 + dy;
            if x1 >= 0 && x1 < size && y1 >= 0 && y1 < size {
                ns.push((x1 as usize, y1 as usize));
            }
        }
        ns
    }
}

#[aoc::aoc(day18, part1)]
fn part_1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { State::On } else { State::Off })
                .collect_vec()
        })
        .collect_vec();
    let mut grid = Grid { grid };
    for _ in 0..100 {
        let mut next_grid = grid.clone();
        let size = next_grid.grid.len();
        for p in (0..size).cartesian_product(0..size) {
            let ns = grid.neighbors(p);
            let c = ns
                .into_iter()
                .map(|p| grid.at(p))
                .filter(|x| matches!(x, State::On))
                .count();
            if matches!(grid.at(p), State::On) {
                if c == 2 || c == 3 {
                    next_grid.set(p, State::On);
                } else {
                    next_grid.set(p, State::Off);
                }
            } else if c == 3 {
                next_grid.set(p, State::On);
            } else {
                next_grid.set(p, State::Off);
            }
        }
        grid = next_grid;
    }
    grid.grid
        .into_iter()
        .flat_map(std::iter::IntoIterator::into_iter)
        .filter(|x| matches!(x, State::On))
        .count()
}

#[aoc::aoc(day18, part2)]
fn part_2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { State::On } else { State::Off })
                .collect_vec()
        })
        .collect_vec();
    let mut grid = Grid { grid };
    let size = grid.grid.len();
    grid.set((0, 0), State::On);
    grid.set((0, size - 1), State::On);
    grid.set((size - 1, 0), State::On);
    grid.set((size - 1, size - 1), State::On);
    for _ in 0..100 {
        let mut next_grid = grid.clone();
        for p in (0..size).cartesian_product(0..size) {
            let ns = grid.neighbors(p);
            let c = ns
                .into_iter()
                .map(|p| grid.at(p))
                .filter(|x| matches!(x, State::On))
                .count();
            if matches!(grid.at(p), State::On) {
                if c == 2 || c == 3 {
                    next_grid.set(p, State::On);
                } else {
                    next_grid.set(p, State::Off);
                }
            } else if c == 3 {
                next_grid.set(p, State::On);
            } else {
                next_grid.set(p, State::Off);
            }
        }
        grid = next_grid;
        grid.set((0, 0), State::On);
        grid.set((0, size - 1), State::On);
        grid.set((size - 1, 0), State::On);
        grid.set((size - 1, size - 1), State::On);
    }
    grid.grid
        .into_iter()
        .flat_map(std::iter::IntoIterator::into_iter)
        .filter(|x| matches!(x, State::On))
        .count()
}
