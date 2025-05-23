use std::collections::{HashMap, HashSet, VecDeque};

use aoc::aoc;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Item {
    Empty,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    const fn opp(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    const fn perp(self) -> (Self, Self) {
        match self {
            Self::Up => (Self::Left, Self::Right),
            Self::Down => (Self::Right, Self::Left),
            Self::Left => (Self::Down, Self::Up),
            Self::Right => (Self::Up, Self::Down),
        }
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &str) -> usize {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = (i, j);
            } else if cell == 'E' {
                end = (i, j);
            }
        }
    }
    let grid = grid
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|c| if c == '#' { Item::Wall } else { Item::Empty })
                .collect_vec()
        })
        .collect_vec();
    let width = grid[0].len();
    let height = grid.len();
    let mut costs = HashMap::new();
    let mut queue = neighbours(end, height, width)
        .into_iter()
        .map(|n| (n.0, n.1, n.2.opp()))
        .filter(|n| grid[n.0][n.1] != Item::Wall)
        .collect::<VecDeque<_>>();
    costs.insert((end.0, end.1, Dir::Up), 0);
    costs.insert((end.0, end.1, Dir::Down), 0);
    costs.insert((end.0, end.1, Dir::Left), 0);
    costs.insert((end.0, end.1, Dir::Right), 0);
    for n in &queue {
        costs.insert((n.0, n.1, n.2), 1);
    }
    while let Some((i, j, dir)) = queue.pop_front() {
        let pos = (i, j);
        if grid[i][j] == Item::Wall {
            unreachable!();
        }
        let n_neighbours = neighbours(pos, height, width)
            .into_iter()
            .map(|n| (n.0, n.1, n.2.opp()))
            .filter(|n| !(grid[n.0][n.1] == Item::Wall || n.2 == dir.opp()))
            .collect_vec();
        let current_cost = *costs.get(&(i, j, dir)).unwrap();
        for n in n_neighbours {
            let cost = if n.2 == dir { 1 } else { 1001 } + current_cost;
            if let Some(c) = costs.get(&(n.0, n.1, n.2)) {
                if cost < *c {
                    costs.insert((n.0, n.1, n.2), cost);
                    queue.push_back((n.0, n.1, n.2));
                }
            } else {
                costs.insert((n.0, n.1, n.2), cost);
                queue.push_back((n.0, n.1, n.2));
            }
        }
    }
    let start_neighbours = neighbours(start, height, width);
    let mut min_cost = usize::MAX;
    for n in start_neighbours {
        if n.2 == Dir::Left || grid[n.0][n.1] == Item::Wall {
            continue;
        }
        if let Some(c) = costs.get(&(n.0, n.1, n.2)) {
            let cost = if n.2 == Dir::Right { 1 } else { 1001 } + *c;
            min_cost = min_cost.min(cost);
        }
    }
    min_cost
}

fn neighbours(pos: (usize, usize), height: usize, width: usize) -> Vec<(usize, usize, Dir)> {
    let mut result = vec![];
    if pos.0 > 0 {
        result.push((pos.0 - 1, pos.1, Dir::Up));
    }
    if pos.0 < height - 1 {
        result.push((pos.0 + 1, pos.1, Dir::Down));
    }
    if pos.1 > 0 {
        result.push((pos.0, pos.1 - 1, Dir::Left));
    }
    if pos.1 < width - 1 {
        result.push((pos.0, pos.1 + 1, Dir::Right));
    }
    result
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &str) -> usize {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = (i, j);
            } else if cell == 'E' {
                end = (i, j);
            }
        }
    }
    let grid = grid
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|c| if c == '#' { Item::Wall } else { Item::Empty })
                .collect_vec()
        })
        .collect_vec();
    let width = grid[0].len();
    let height = grid.len();
    let mut costs = HashMap::new();
    let mut queue = neighbours(end, height, width)
        .into_iter()
        .map(|n| (n.0, n.1, n.2.opp()))
        .filter(|n| grid[n.0][n.1] != Item::Wall)
        .collect::<VecDeque<_>>();
    costs.insert((end.0, end.1, Dir::Up), 0);
    costs.insert((end.0, end.1, Dir::Down), 0);
    costs.insert((end.0, end.1, Dir::Left), 0);
    costs.insert((end.0, end.1, Dir::Right), 0);
    for n in &queue {
        costs.insert((n.0, n.1, n.2), 1);
    }
    while let Some((i, j, dir)) = queue.pop_front() {
        let pos = (i, j);
        if grid[i][j] == Item::Wall {
            unreachable!();
        }
        let n_neighbours = neighbours(pos, height, width)
            .into_iter()
            .map(|n| (n.0, n.1, n.2.opp()))
            .filter(|n| !(grid[n.0][n.1] == Item::Wall || n.2 == dir.opp()))
            .collect_vec();
        let current_cost = *costs.get(&(i, j, dir)).unwrap();
        for n in n_neighbours {
            let cost = if n.2 == dir { 1 } else { 1001 } + current_cost;
            if let Some(c) = costs.get(&(n.0, n.1, n.2)) {
                if cost < *c {
                    costs.insert((n.0, n.1, n.2), cost);
                    queue.push_back((n.0, n.1, n.2));
                }
            } else {
                costs.insert((n.0, n.1, n.2), cost);
                queue.push_back((n.0, n.1, n.2));
            }
        }
    }
    let start_neighbours = neighbours(start, height, width);
    let mut min_cost = usize::MAX;
    for n in start_neighbours {
        if n.2 == Dir::Left || grid[n.0][n.1] == Item::Wall {
            continue;
        }
        if let Some(c) = costs.get(&(n.0, n.1, n.2)) {
            let cost = if n.2 == Dir::Right { 1 } else { 1001 } + *c;
            min_cost = min_cost.min(cost);
        }
    }
    costs.insert((start.0, start.1, Dir::Right), min_cost);

    let mut queue = VecDeque::from([(start, Dir::Right)]);
    let mut cells = HashSet::new();
    cells.insert(start);

    while let Some((pos, dir)) = queue.pop_front() {
        let neighbours = neighbours(pos, height, width);
        let min_cost = *costs.get(&(pos.0, pos.1, dir)).unwrap();
        for n in neighbours {
            if n.2 == dir.opp() || grid[n.0][n.1] == Item::Wall {
                continue;
            }
            let n_cost = costs.get(&(n.0, n.1, n.2));
            if let Some(n_cost) = n_cost {
                let cost = if n.2 == dir { 1 } else { 1001 } + n_cost;
                if cost == min_cost {
                    queue.push_back(((n.0, n.1), n.2));
                    cells.insert((n.0, n.1));
                }
            }
            let perp = n.2.perp();
            for p in [perp.0, perp.1] {
                let p_cost = costs.get(&(n.0, n.1, p));
                if let Some(p_cost) = p_cost {
                    let cost = if n.2 == dir { 1 } else { 1001 } + p_cost + 1000;
                    if cost == min_cost {
                        queue.push_back(((n.0, n.1), p));
                        cells.insert((n.0, n.1));
                    }
                }
            }
        }
    }

    cells.len()
}
