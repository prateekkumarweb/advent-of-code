use aoc::aoc;
use regex::Regex;

#[aoc(day1, part1)]
pub fn part_1(input: &str) -> String {
    let numbers = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_numeric())
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for num in numbers {
        let first = num.first().unwrap();
        let last = num.last().unwrap();
        sum += first * 10 + last;
    }

    format!("{sum}")
}

#[aoc(day1, part2)]
pub fn part_2(input: &str) -> String {
    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re2 = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    let sum = input
        .lines()
        .map(|line| {
            let match1 = re.find(line).unwrap().as_str();
            let rev_line = line.chars().rev().collect::<String>();
            let match2 = re2.find(rev_line.as_str()).unwrap().as_str();
            let match1 = match1
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9")
                .parse::<u32>()
                .unwrap();
            let match2 = match2
                .chars()
                .rev()
                .collect::<String>()
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9")
                .parse::<u32>()
                .unwrap();
            match1 * 10 + match2
        })
        .sum::<u32>();

    format!("{sum}")
}
