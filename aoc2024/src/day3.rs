use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    let lines = input.lines();
    let rx = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut result = 0;
    for line in lines {
        for cap in rx.captures_iter(line) {
            let a = cap[1].parse::<usize>().unwrap();
            let b = cap[2].parse::<usize>().unwrap();
            result += a * b;
        }
    }
    result
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    let lines = input.lines();
    let rx = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|don't\(\)|do\(\)").unwrap();
    let mut result = 0;
    let mut add = true;
    for line in lines {
        for cap in rx.captures_iter(line) {
            let code = &cap[0];
            if code == "do()" {
                add = true;
            } else if code == "don't()" {
                add = false;
            } else {
                let a = cap[1].parse::<usize>().unwrap();
                let b = cap[2].parse::<usize>().unwrap();
                result += if add { a * b } else { 0 };
            }
        }
    }
    result
}
