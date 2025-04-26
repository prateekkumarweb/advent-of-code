use aoc::aoc;
use itertools::Itertools;

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn get(&self, pos: (usize, usize)) -> char {
        self.grid[pos.0][pos.1]
    }

    fn set(&mut self, pos: (usize, usize), val: char) {
        self.grid[pos.0][pos.1] = val;
    }

    fn next_pos(&self, pos: (usize, usize), dir: char) -> Option<(usize, usize)> {
        match dir {
            '>' => {
                if pos.1 < self.width() - 1 {
                    Some((pos.0, pos.1 + 1))
                } else {
                    None
                }
            }
            '<' => {
                if pos.1 > 0 {
                    Some((pos.0, pos.1 - 1))
                } else {
                    None
                }
            }
            '^' => {
                if pos.0 > 0 {
                    Some((pos.0 - 1, pos.1))
                } else {
                    None
                }
            }
            'v' => {
                if pos.0 < self.height() - 1 {
                    Some((pos.0 + 1, pos.1))
                } else {
                    None
                }
            }
            _ => panic!("Invalid direction"),
        }
    }

    fn move_pos(&mut self, pos: (usize, usize), dir: char) -> (usize, usize) {
        let next_pos = self.next_pos(pos, dir);
        if let Some(next_pos) = next_pos {
            if self.get(next_pos) == '.' {
                self.set(next_pos, self.get(pos));
                self.set(pos, '.');
                next_pos
            } else if self.get(next_pos) == '#' {
                pos
            } else if self.get(next_pos) == 'O' {
                self.move_pos(next_pos, dir);
                if self.get(next_pos) == '.' {
                    self.set(next_pos, self.get(pos));
                    self.set(pos, '.');
                    next_pos
                } else {
                    pos
                }
            } else {
                pos
            }
        } else {
            pos
        }
    }
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut grid = lines
        .by_ref()
        .take_while(|l| !l.trim().is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let moves = lines.join("");
    let mut pos = (0, 0);
    for (r, row) in grid.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == '@' {
                pos = (r, c);
            }
        }
    }
    grid[pos.0][pos.1] = '.';
    let mut grid = Grid { grid };
    for m in moves.chars() {
        pos = grid.move_pos(pos, m);
    }
    let mut result = 0;
    for (r, row) in grid.grid.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            if grid.get((r, c)) == 'O' {
                result += r * 100 + c;
            }
        }
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Item {
    Empty,
    Wall,
    BoxOpen,
    BoxClose,
}

struct Grid2 {
    grid: Vec<Vec<Item>>,
}

impl Grid2 {
    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn get(&self, pos: (usize, usize)) -> Item {
        self.grid[pos.0][pos.1]
    }

    fn set(&mut self, pos: (usize, usize), val: Item) {
        self.grid[pos.0][pos.1] = val;
    }

    fn next_position(&self, pos: (usize, usize), dir: Dir) -> Option<(usize, usize)> {
        match dir {
            Dir::Up => {
                if pos.0 == 0 {
                    None
                } else {
                    Some((pos.0 - 1, pos.1))
                }
            }
            Dir::Down => {
                if pos.0 == self.height() - 1 {
                    None
                } else {
                    Some((pos.0 + 1, pos.1))
                }
            }
            Dir::Left => {
                if pos.1 == 0 {
                    None
                } else {
                    Some((pos.0, pos.1 - 1))
                }
            }
            Dir::Right => {
                if pos.1 == self.width() - 1 {
                    None
                } else {
                    Some((pos.0, pos.1 + 1))
                }
            }
        }
    }

    fn move_pos(&mut self, pos: (usize, usize), dir: Dir) -> (usize, usize) {
        if let Some(next_pos) = self.next_position(pos, dir) {
            let next_item = self.get(next_pos);
            match (next_item, dir) {
                (Item::Empty, _) => next_pos,
                (Item::Wall, _) => pos,
                (Item::BoxOpen, _) => {
                    if self.is_movable(next_pos, dir) {
                        self.move_box(next_pos, dir);
                        next_pos
                    } else {
                        pos
                    }
                }
                (Item::BoxClose, _) => {
                    if self.is_movable(self.next_position(next_pos, Dir::Left).unwrap(), dir) {
                        self.move_box(self.next_position(next_pos, Dir::Left).unwrap(), dir);
                        next_pos
                    } else {
                        pos
                    }
                }
            }
        } else {
            pos
        }
    }

    fn is_movable(&self, pos: (usize, usize), dir: Dir) -> bool {
        if let Some(next_pos) = self.next_position(pos, dir) {
            if let Some(next_pos_1) = self.next_position(next_pos, Dir::Right) {
                let next_item = self.get(next_pos);
                let next_item_1 = self.get(next_pos_1);
                match (next_item, next_item_1, dir) {
                    (Item::Empty, Item::Empty, _) => true,
                    (Item::Wall, _, _) => false,
                    (_, Item::Wall, _) => false,
                    (Item::BoxOpen, Item::BoxClose, Dir::Up | Dir::Down) => {
                        self.is_movable(next_pos, dir)
                    }
                    (Item::BoxClose, Item::BoxOpen, Dir::Up | Dir::Down) => {
                        self.is_movable(self.next_position(next_pos, Dir::Left).unwrap(), dir)
                            && self.is_movable(next_pos_1, dir)
                    }
                    (Item::Empty, Item::BoxOpen, Dir::Up | Dir::Down) => {
                        self.is_movable(next_pos_1, dir)
                    }
                    (Item::BoxClose, Item::Empty, Dir::Up | Dir::Down) => {
                        self.is_movable(self.next_position(next_pos, Dir::Left).unwrap(), dir)
                    }
                    (Item::Empty, _, Dir::Left) => true,
                    (_, Item::Empty, Dir::Right) => true,
                    (Item::BoxClose, _, Dir::Left) => {
                        self.is_movable(self.next_position(next_pos, Dir::Left).unwrap(), dir)
                    }
                    (_, Item::BoxOpen, Dir::Right) => self.is_movable(next_pos_1, dir),
                    (Item::Empty, Item::BoxClose, _) => unreachable!(),
                    (Item::BoxOpen, Item::Empty, _) => unreachable!(),
                    (Item::BoxOpen, Item::BoxOpen, _) => unreachable!(),
                    (Item::BoxClose, Item::BoxClose, _) => unreachable!(),
                    (Item::BoxOpen, Item::BoxClose, Dir::Left | Dir::Right) => unreachable!(),
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn move_box(&mut self, pos: (usize, usize), dir: Dir) {
        let pos_1 = self.next_position(pos, Dir::Right).unwrap();
        let next_pos = self.next_position(pos, dir).unwrap();
        let next_pos_1 = self.next_position(next_pos, Dir::Right).unwrap();
        let next_item = self.get(next_pos);
        let next_item_1 = self.get(next_pos_1);
        match (next_item, next_item_1, dir) {
            (Item::Empty, Item::Empty, _) => {}
            (Item::Wall, _, _) => unreachable!(),
            (_, Item::Wall, _) => unreachable!(),
            (Item::BoxOpen, Item::BoxClose, Dir::Up | Dir::Down) => self.move_box(next_pos, dir),
            (Item::BoxClose, Item::BoxOpen, Dir::Up | Dir::Down) => {
                self.move_box(self.next_position(next_pos, Dir::Left).unwrap(), dir);
                self.move_box(next_pos_1, dir);
            }
            (Item::Empty, Item::BoxOpen, Dir::Up | Dir::Down) => self.move_box(next_pos_1, dir),
            (Item::BoxClose, Item::Empty, Dir::Up | Dir::Down) => {
                self.move_box(self.next_position(next_pos, Dir::Left).unwrap(), dir);
            }
            (Item::Empty, _, Dir::Left) => {}
            (_, Item::Empty, Dir::Right) => {}
            (Item::BoxClose, _, Dir::Left) => {
                self.move_box(self.next_position(next_pos, Dir::Left).unwrap(), dir)
            }
            (_, Item::BoxOpen, Dir::Right) => self.move_box(next_pos_1, dir),
            (Item::Empty, Item::BoxClose, _) => unreachable!(),
            (Item::BoxOpen, Item::Empty, _) => unreachable!(),
            (Item::BoxOpen, Item::BoxOpen, _) => unreachable!(),
            (Item::BoxClose, Item::BoxClose, _) => unreachable!(),
            (Item::BoxOpen, Item::BoxClose, Dir::Left | Dir::Right) => unreachable!(),
        }
        self.set(pos, Item::Empty);
        self.set(pos_1, Item::Empty);
        self.set(next_pos, Item::BoxOpen);
        self.set(next_pos_1, Item::BoxClose);
    }
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut grid = lines
        .by_ref()
        .take_while(|l| !l.trim().is_empty())
        .map(|l| {
            l.chars()
                .flat_map(|c| {
                    if c == '#' {
                        "##"
                    } else if c == '.' {
                        ".."
                    } else if c == 'O' {
                        "[]"
                    } else if c == '@' {
                        "@."
                    } else {
                        ""
                    }
                    .chars()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let moves = lines.join("");
    let mut pos = (0, 0);
    for (r, row) in grid.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == '@' {
                pos = (r, c);
            }
        }
    }
    grid[pos.0][pos.1] = '.';
    let mut grid = Grid2 {
        grid: grid
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .map(|c| match c {
                        '#' => Item::Wall,
                        '.' => Item::Empty,
                        '[' => Item::BoxOpen,
                        ']' => Item::BoxClose,
                        _ => panic!("Invalid item"),
                    })
                    .collect()
            })
            .collect(),
    };
    for m in moves.chars() {
        let m = match m {
            '^' => Dir::Up,
            'v' => Dir::Down,
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => panic!("Invalid move"),
        };
        pos = grid.move_pos(pos, m);
    }
    let mut result = 0;
    for (r, row) in grid.grid.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            if grid.get((r, c)) == Item::BoxOpen {
                result += r * 100 + c;
            }
        }
    }
    result
}
