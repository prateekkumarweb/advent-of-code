use aoc::aoc;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let lines = input.lines();
    let array = lines
        .map(|line| {
            let mut line = line.split(' ').filter(|p| !p.is_empty());
            (
                line.next().unwrap().parse().unwrap(),
                line.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(i32, i32)>>();
    let mut first = array.iter().map(|(a, _)| *a).collect::<Vec<i32>>();
    first.sort_unstable();
    let mut second = array.iter().map(|(_, b)| *b).collect::<Vec<i32>>();
    second.sort_unstable();
    let diff: i32 = first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    diff
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let lines = input.lines();
    let array = lines
        .map(|line| {
            let mut line = line.split(' ').filter(|p| !p.is_empty());
            (
                line.next().unwrap().parse().unwrap(),
                line.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(i32, i32)>>();
    let mut first = array.iter().map(|(a, _)| *a).collect::<Vec<i32>>();
    first.sort_unstable();
    let mut second = array.iter().map(|(_, b)| *b).collect::<Vec<i32>>();
    second.sort_unstable();
    let diff: i32 = first
        .iter()
        .map(|a| a * second.iter().filter(|b| a == *b).count() as i32)
        .sum();
    diff
}
