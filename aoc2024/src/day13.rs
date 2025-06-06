use aoc::aoc;
use itertools::Itertools;
use regex::Regex;

#[aoc(day13, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let button_regex = Regex::new(r"Button [A|B]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let result = input
        .lines()
        .chunks(4)
        .into_iter()
        .map(|mut chunk| {
            let button_a = chunk.next().unwrap();
            let button_b = chunk.next().unwrap();
            let prize = chunk.next().unwrap();
            let button_a = button_regex.captures(button_a).unwrap();
            let button_b = button_regex.captures(button_b).unwrap();
            let prize = prize_regex.captures(prize).unwrap();

            let button_a = (
                button_a[1].parse::<i32>().unwrap(),
                button_a[2].parse::<i32>().unwrap(),
            );
            let button_b = (
                button_b[1].parse::<i32>().unwrap(),
                button_b[2].parse::<i32>().unwrap(),
            );
            let prize = (
                prize[1].parse::<i32>().unwrap(),
                prize[2].parse::<i32>().unwrap(),
            );

            let square_maxtrix = ((button_a.0, button_b.0), (button_a.1, button_b.1));
            let det =
                square_maxtrix.0.0 * square_maxtrix.1.1 - square_maxtrix.0.1 * square_maxtrix.1.0;
            let inv_0 = (
                (square_maxtrix.1.1, -square_maxtrix.0.1),
                (-square_maxtrix.1.0, square_maxtrix.0.0),
            );
            let inv_0_rhs = (
                inv_0.0.0 * prize.0 + inv_0.0.1 * prize.1,
                inv_0.1.0 * prize.0 + inv_0.1.1 * prize.1,
            );
            let inv_rhs_rem = (inv_0_rhs.0 % det, inv_0_rhs.1 % det);
            let inv_rhs_div = (inv_0_rhs.0 / det, inv_0_rhs.1 / det);
            if inv_rhs_rem == (0, 0) {
                inv_rhs_div.0 * 3 + inv_rhs_div.1
            } else {
                0
            }
        })
        .sum::<i32>();

    result
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let button_regex = Regex::new(r"Button [A|B]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    const INC: i64 = 10000000000000;

    let result = input
        .lines()
        .chunks(4)
        .into_iter()
        .map(|mut chunk| {
            let button_a = chunk.next().unwrap();
            let button_b = chunk.next().unwrap();
            let prize = chunk.next().unwrap();
            let button_a = button_regex.captures(button_a).unwrap();
            let button_b = button_regex.captures(button_b).unwrap();
            let prize = prize_regex.captures(prize).unwrap();

            let button_a = (
                button_a[1].parse::<i64>().unwrap(),
                button_a[2].parse::<i64>().unwrap(),
            );
            let button_b = (
                button_b[1].parse::<i64>().unwrap(),
                button_b[2].parse::<i64>().unwrap(),
            );
            let prize = (
                prize[1].parse::<i64>().unwrap() + INC,
                prize[2].parse::<i64>().unwrap() + INC,
            );

            let square_maxtrix = ((button_a.0, button_b.0), (button_a.1, button_b.1));
            let det =
                square_maxtrix.0.0 * square_maxtrix.1.1 - square_maxtrix.0.1 * square_maxtrix.1.0;
            let inv_0 = (
                (square_maxtrix.1.1, -square_maxtrix.0.1),
                (-square_maxtrix.1.0, square_maxtrix.0.0),
            );
            let inv_0_rhs = (
                inv_0.0.0 * prize.0 + inv_0.0.1 * prize.1,
                inv_0.1.0 * prize.0 + inv_0.1.1 * prize.1,
            );
            let inv_rhs_rem = (inv_0_rhs.0 % det, inv_0_rhs.1 % det);
            let inv_rhs_div = (inv_0_rhs.0 / det, inv_0_rhs.1 / det);
            if inv_rhs_rem == (0, 0) {
                inv_rhs_div.0 * 3 + inv_rhs_div.1
            } else {
                0
            }
        })
        .sum::<i64>();
    result
}
