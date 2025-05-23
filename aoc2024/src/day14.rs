use aoc::aoc;
use itertools::Itertools;

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|x| {
            let mut l = x.split_whitespace();
            let p = l
                .next()
                .unwrap()
                .strip_prefix("p=")
                .unwrap()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let v = l
                .next()
                .unwrap()
                .strip_prefix("v=")
                .unwrap()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (p, v)
        })
        .collect_vec();

    let (width, height) = if input.len() == 12 {
        (11, 7)
    } else {
        (101, 103)
    };

    let mut robots = input;

    for r in &mut robots {
        r.0[0] = (r.0[0] + 100 * r.1[0]).rem_euclid(width as i32);
        r.0[1] = (r.0[1] + 100 * r.1[1]).rem_euclid(height as i32);
    }

    let mut grid = vec![vec![0u8; width]; height];
    for r in &robots {
        grid[r.0[1] as usize][r.0[0] as usize] += 1;
    }

    let q1 = ((0, height as i32 / 2), (0, width as i32 / 2));
    let q2 = (
        (0, height as i32 / 2),
        (width as i32 - width as i32 / 2, width as i32),
    );
    let q3 = (
        (height as i32 - height as i32 / 2, height as i32),
        (0, width as i32 / 2),
    );
    let q4 = (
        (height as i32 - height as i32 / 2, height as i32),
        (width as i32 - width as i32 / 2, width as i32),
    );

    let mut q1_count = 0;
    let mut q2_count = 0;
    let mut q3_count = 0;
    let mut q4_count = 0;

    for r in &robots {
        if r.0[0] >= q1.1.0 && r.0[0] < q1.1.1 && r.0[1] >= q1.0.0 && r.0[1] < q1.0.1 {
            q1_count += 1;
        } else if r.0[0] >= q2.1.0 && r.0[0] < q2.1.1 && r.0[1] >= q2.0.0 && r.0[1] < q2.0.1 {
            q2_count += 1;
        } else if r.0[0] >= q3.1.0 && r.0[0] < q3.1.1 && r.0[1] >= q3.0.0 && r.0[1] < q3.0.1 {
            q3_count += 1;
        } else if r.0[0] >= q4.1.0 && r.0[0] < q4.1.1 && r.0[1] >= q4.0.0 && r.0[1] < q4.0.1 {
            q4_count += 1;
        }
    }

    q1_count * q2_count * q3_count * q4_count
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &str) -> usize {
    let input = input
        .lines()
        .map(|x| {
            let mut l = x.split_whitespace();
            let p = l
                .next()
                .unwrap()
                .strip_prefix("p=")
                .unwrap()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let v = l
                .next()
                .unwrap()
                .strip_prefix("v=")
                .unwrap()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (p, v)
        })
        .collect_vec();

    let (width, height) = if input.len() == 12 {
        (11, 7)
    } else {
        (101, 103)
    };

    let mut robots = input;

    for iter in 1.. {
        for r in &mut robots {
            r.0[0] = (r.0[0] + r.1[0]).rem_euclid(width as i32);
            r.0[1] = (r.0[1] + r.1[1]).rem_euclid(height as i32);
        }

        let mut grid = vec![vec![0u8; width]; height];
        for r in &robots {
            grid[r.0[1] as usize][r.0[0] as usize] += 1;
        }

        // find rows with contingous non zero numbers
        let mut rows = vec![];
        for r in &grid {
            let mut row = vec![];
            let mut count = 0;
            for c in r {
                if *c > 0 {
                    count += 1;
                } else if count > 0 {
                    row.push(count);
                    count = 0;
                }
            }
            if count > 0 {
                row.push(count);
            }
            rows.push(row.into_iter().max().unwrap_or(0));
        }
        if rows.iter().max().unwrap_or(&0) > &20 {
            return iter;
        }
    }

    unreachable!()
}
