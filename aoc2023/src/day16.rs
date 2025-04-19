use std::collections::VecDeque;

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Tile>>,
    energized: Vec<Vec<bool>>,
    explored: Vec<Vec<HasExplored>>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy)]
struct HasExplored(bool, bool, bool, bool);

impl HasExplored {
    const fn right(self) -> bool {
        self.0
    }

    const fn left(self) -> bool {
        self.1
    }

    const fn up(self) -> bool {
        self.2
    }

    const fn down(self) -> bool {
        self.3
    }

    const fn set_right(&mut self) {
        self.0 = true;
    }

    const fn set_left(&mut self) {
        self.1 = true;
    }

    const fn set_up(&mut self) {
        self.2 = true;
    }

    const fn set_down(&mut self) {
        self.3 = true;
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Horiz,
    Vert,
    Slash,
    BackSlash,
}

#[derive(Debug, Clone, Copy)]
enum BeamDir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
struct BeamPos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Beam {
    pos: BeamPos,
    dir: BeamDir,
}

impl Grid {
    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn tile_at(&self, pos: BeamPos) -> Tile {
        self.grid[pos.y][pos.x]
    }

    fn next_pos(&self, pos: BeamPos, dir: BeamDir) -> Option<BeamPos> {
        let next_beam_pos = match dir {
            BeamDir::Left => {
                if pos.x == 0 {
                    return None;
                }
                BeamPos {
                    x: pos.x - 1,
                    y: pos.y,
                }
            }
            BeamDir::Right => {
                if pos.x == self.grid[pos.y].len() - 1 {
                    return None;
                }
                BeamPos {
                    x: pos.x + 1,
                    y: pos.y,
                }
            }
            BeamDir::Up => {
                if pos.y == 0 {
                    return None;
                }
                BeamPos {
                    x: pos.x,
                    y: pos.y - 1,
                }
            }
            BeamDir::Down => {
                if pos.y == self.grid.len() - 1 {
                    return None;
                }
                BeamPos {
                    x: pos.x,
                    y: pos.y + 1,
                }
            }
        };
        Some(next_beam_pos)
    }

    fn step(&mut self, beam: Beam) -> Vec<Beam> {
        let tile = self.tile_at(beam.pos);
        let next_dirs = match (tile, beam.dir) {
            (Tile::Empty, dir) => vec![dir],
            (Tile::Horiz, BeamDir::Left)
            | (Tile::Slash, BeamDir::Down)
            | (Tile::BackSlash, BeamDir::Up) => vec![BeamDir::Left],
            (Tile::Horiz, BeamDir::Right)
            | (Tile::Slash, BeamDir::Up)
            | (Tile::BackSlash, BeamDir::Down) => vec![BeamDir::Right],
            (Tile::Horiz, BeamDir::Up | BeamDir::Down) => vec![BeamDir::Left, BeamDir::Right],
            (Tile::Vert, BeamDir::Up)
            | (Tile::Slash, BeamDir::Right)
            | (Tile::BackSlash, BeamDir::Left) => vec![BeamDir::Up],
            (Tile::Vert, BeamDir::Down)
            | (Tile::Slash, BeamDir::Left)
            | (Tile::BackSlash, BeamDir::Right) => vec![BeamDir::Down],
            (Tile::Vert, BeamDir::Left | BeamDir::Right) => vec![BeamDir::Up, BeamDir::Down],
        };

        next_dirs
            .into_iter()
            .filter_map(|dir| self.next_pos(beam.pos, dir).map(|pos| Beam { pos, dir }))
            .collect_vec()
    }

    fn is_explored(&self, beam: Beam) -> bool {
        let Beam { pos, dir } = beam;
        match dir {
            BeamDir::Left => self.explored[pos.y][pos.x].left(),
            BeamDir::Right => self.explored[pos.y][pos.x].right(),
            BeamDir::Up => self.explored[pos.y][pos.x].up(),
            BeamDir::Down => self.explored[pos.y][pos.x].down(),
        }
    }

    fn explore(&mut self, beam: &Beam) {
        let Beam { pos, dir } = beam;
        match dir {
            BeamDir::Left => self.explored[pos.y][pos.x].set_left(),
            BeamDir::Right => self.explored[pos.y][pos.x].set_right(),
            BeamDir::Up => self.explored[pos.y][pos.x].set_up(),
            BeamDir::Down => self.explored[pos.y][pos.x].set_down(),
        }
    }

    fn energize(&mut self, start: Beam) {
        let mut beams = VecDeque::new();
        beams.push_back(start);
        self.energized[start.pos.y][start.pos.x] = true;

        while !beams.is_empty() {
            let beam = beams.pop_front().unwrap();
            if self.is_explored(beam) {
                continue;
            }
            self.explore(&beam);
            let next_beams = self.step(beam);
            for beam in &next_beams {
                self.energized[beam.pos.y][beam.pos.x] = true;
            }
            beams.extend(next_beams);
        }
    }

    fn tiles_energized_count(&self) -> usize {
        self.energized
            .iter()
            .map(|row| row.iter().filter(|&&e| e).count())
            .sum()
    }

    fn reset(&mut self) {
        self.energized = vec![vec![false; self.grid[0].len()]; self.grid.len()];
        self.explored = vec![
            vec![HasExplored(false, false, false, false); self.grid[0].len()];
            self.grid.len()
        ];
    }
}

#[aoc(day16, part1)]
pub fn part_1(input: &str) -> String {
    let input = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|c| match c {
                    b'.' => Tile::Empty,
                    b'-' => Tile::Horiz,
                    b'|' => Tile::Vert,
                    b'/' => Tile::Slash,
                    b'\\' => Tile::BackSlash,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut grid = Grid {
        energized: vec![vec![false; input[0].len()]; input.len()],
        explored: vec![vec![HasExplored(false, false, false, false); input[0].len()]; input.len()],
        grid: input,
    };

    grid.energize(Beam {
        dir: BeamDir::Right,
        pos: BeamPos { x: 0, y: 0 },
    });

    grid.tiles_energized_count().to_string()
}

#[aoc(day16, part2)]
pub fn part_2(input: &str) -> String {
    let input = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|c| match c {
                    b'.' => Tile::Empty,
                    b'-' => Tile::Horiz,
                    b'|' => Tile::Vert,
                    b'/' => Tile::Slash,
                    b'\\' => Tile::BackSlash,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut grid = Grid {
        energized: vec![vec![false; input[0].len()]; input.len()],
        explored: vec![vec![HasExplored(false, false, false, false); input[0].len()]; input.len()],
        grid: input,
    };

    let mut corner_beams = vec![];

    for y in 0..grid.height() {
        corner_beams.push(Beam {
            dir: BeamDir::Right,
            pos: BeamPos { x: 0, y },
        });
        corner_beams.push(Beam {
            dir: BeamDir::Left,
            pos: BeamPos {
                x: grid.width() - 1,
                y,
            },
        });
    }

    for x in 0..grid.width() {
        corner_beams.push(Beam {
            dir: BeamDir::Down,
            pos: BeamPos { x, y: 0 },
        });
        corner_beams.push(Beam {
            dir: BeamDir::Up,
            pos: BeamPos {
                x,
                y: grid.height() - 1,
            },
        });
    }

    corner_beams
        .iter()
        .map(|&beam| {
            grid.reset();
            grid.energize(beam);
            grid.tiles_energized_count()
        })
        .max()
        .unwrap()
        .to_string()
}
