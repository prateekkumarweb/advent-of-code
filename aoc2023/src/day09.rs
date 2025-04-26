use aoc::aoc;
use itertools::Itertools;

fn find_diff(series: &[i64]) -> Vec<i64> {
    series.windows(2).map(|w| w[1] - w[0]).collect_vec()
}

fn find_next(series: Vec<i64>) -> i64 {
    let mut diffs = vec![series];
    loop {
        let diff = find_diff(&diffs[diffs.len() - 1]);
        if diff.iter().all(|x| *x == 0) {
            break;
        }
        diffs.push(diff);
    }

    let x = diffs[diffs.len() - 1][0];
    diffs.last_mut().unwrap().push(x);

    for i in (0..diffs.len() - 1).rev() {
        let prev_diff = &diffs[i + 1];
        let x = prev_diff[prev_diff.len() - 1] + diffs[i][diffs[i].len() - 1];
        diffs[i].push(x);
    }

    diffs[0][diffs[0].len() - 1]
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect_vec()
}

fn parse_line_rev(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .rev()
        .collect_vec()
}

#[aoc(day9, part1)]
pub fn part_1(input: &str) -> String {
    let input = input.lines().map(parse_line).collect_vec();

    let sum = input.into_iter().map(find_next).sum::<i64>();

    sum.to_string()
}

#[aoc(day9, part2)]
pub fn part_2(input: &str) -> String {
    let input = input.lines().map(parse_line_rev).collect_vec();

    let sum = input.into_iter().map(find_next).sum::<i64>();

    sum.to_string()
}
