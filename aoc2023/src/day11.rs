use aoc::aoc;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let width = input.find('\n').unwrap();
        let input = input
            .chars()
            .filter(|&c| c == '#' || c == '.')
            .collect_vec();
        let height = input.len() / width;
        Self {
            grid: input,
            width,
            height,
        }
    }

    fn at(&self, pos: Pos) -> char {
        self.grid[pos.y * self.width + pos.x]
    }

    fn row(&self, y: usize) -> impl Iterator<Item = char> + '_ {
        self.grid[y * self.width..(y + 1) * self.width]
            .iter()
            .copied()
    }

    fn col(&self, x: usize) -> impl Iterator<Item = char> + '_ {
        (0..self.height).map(move |y| self.at(Pos { x, y }))
    }

    fn galaxies(&self) -> Vec<Pos> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| Pos { x, y })
            .filter(|&pos| self.at(pos) == '#')
            .collect()
    }

    fn is_expanded_rows(&self) -> Vec<bool> {
        (0..self.height)
            .map(|y| self.row(y).all(|c| c == '.'))
            .collect()
    }

    fn is_expanded_cols(&self) -> Vec<bool> {
        (0..self.width)
            .map(|x| self.col(x).all(|c| c == '.'))
            .collect()
    }
}

fn path_steps(
    a: Pos,
    b: Pos,
    is_expanded_rows: &[bool],
    is_expanded_cols: &[bool],
    expansion_inc: usize,
) -> usize {
    let exp_x = (a.x.min(b.x)..a.x.max(b.x))
        .filter(|&x| is_expanded_cols[x])
        .count();
    let exp_y = (a.y.min(b.y)..a.y.max(b.y))
        .filter(|&y| is_expanded_rows[y])
        .count();
    (exp_x + exp_y) * expansion_inc + a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

#[aoc(day11, part1)]
pub fn part_1(input: &str) -> String {
    let grid = Grid::new(input);
    let is_expanded_rows = grid.is_expanded_rows();
    let is_expanded_cols = grid.is_expanded_cols();

    let galaxies = grid.galaxies();
    (galaxies
        .iter()
        .cartesian_product(galaxies.iter())
        .map(|(&a, &b)| path_steps(a, b, &is_expanded_rows, &is_expanded_cols, 1))
        .sum::<usize>()
        / 2)
    .to_string()
}

#[aoc(day11, part2)]
pub fn part_2(input: &str) -> String {
    let grid = Grid::new(input);
    let is_expanded_rows = grid.is_expanded_rows();
    let is_expanded_cols = grid.is_expanded_cols();

    let galaxies = grid.galaxies();
    (galaxies
        .iter()
        .cartesian_product(galaxies.iter())
        .map(|(&a, &b)| path_steps(a, b, &is_expanded_rows, &is_expanded_cols, 999_999))
        .sum::<usize>()
        / 2)
    .to_string()
}
