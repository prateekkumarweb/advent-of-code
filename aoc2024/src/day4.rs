use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let lines = input.lines();
    let mut grid = vec![];
    for line in lines {
        let row = line.chars().collect::<Vec<_>>();
        grid.push(row);
    }
    let row_len = grid[0].len();
    let col_len = grid.len();
    let mut count = 0;
    let word = ['M', 'A', 'S'];
    for i in 0..col_len {
        for j in 0..row_len {
            if grid[i][j] == 'X' {
                for p in possible((i as i32, j as i32)) {
                    if p.iter().zip(word.iter()).all(|((i, j), c)| {
                        if *i < 0 || *j < 0 || *i >= col_len as i32 || *j >= row_len as i32 {
                            return false;
                        }
                        grid[*i as usize][*j as usize] == *c
                    }) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn possible((i, j): (i32, i32)) -> Vec<Vec<(i32, i32)>> {
    vec![
        vec![(i + 1, j), (i + 2, j), (i + 3, j)],
        vec![(i - 1, j), (i - 2, j), (i - 3, j)],
        vec![(i, j + 1), (i, j + 2), (i, j + 3)],
        vec![(i, j - 1), (i, j - 2), (i, j - 3)],
        vec![(i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)],
        vec![(i - 1, j - 1), (i - 2, j - 2), (i - 3, j - 3)],
        vec![(i + 1, j - 1), (i + 2, j - 2), (i + 3, j - 3)],
        vec![(i - 1, j + 1), (i - 2, j + 2), (i - 3, j + 3)],
    ]
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let lines = input.lines();
    let mut grid = vec![];
    for line in lines {
        let row = line.chars().collect::<Vec<_>>();
        grid.push(row);
    }
    let row_len = grid[0].len();
    let col_len = grid.len();
    let mut count = 0;
    let word = ['M', 'A', 'S'];
    for i in 0..col_len {
        for j in 0..row_len {
            if grid[i][j] == 'A' {
                let mut c = 0;
                for p in possible2((i as i32, j as i32)) {
                    if p.iter().zip(word.iter()).all(|((i, j), c)| {
                        if *i < 0 || *j < 0 || *i >= col_len as i32 || *j >= row_len as i32 {
                            return false;
                        }
                        grid[*i as usize][*j as usize] == *c
                    }) || p.iter().rev().zip(word.iter()).all(|((i, j), c)| {
                        if *i < 0 || *j < 0 || *i >= col_len as i32 || *j >= row_len as i32 {
                            return false;
                        }
                        grid[*i as usize][*j as usize] == *c
                    }) {
                        c += 1;
                    }
                }
                if c == 2 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn possible2((i, j): (i32, i32)) -> Vec<Vec<(i32, i32)>> {
    vec![
        vec![(i - 1, j - 1), (i, j), (i + 1, j + 1)],
        vec![(i - 1, j + 1), (i, j), (i + 1, j - 1)],
    ]
}
