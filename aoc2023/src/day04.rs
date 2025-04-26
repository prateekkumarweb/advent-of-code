use aoc::aoc;
use regex::Regex;

fn find_common_count(line: &str) -> usize {
    let re = Regex::new(r"Card\s+(\d+):\s+([\d\s]+) \| ([\d\s]+)").unwrap();
    let matches = re.captures(line).unwrap();

    let winning_numbers = matches
        .get(2)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let numbers_with_you = matches
        .get(3)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let common_count = numbers_with_you
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count();

    common_count
}

#[aoc(day4, part1)]
pub fn part_1(input: &str) -> String {
    let sum = input
        .lines()
        .map(find_common_count)
        .map(|common_count| {
            if common_count > 0 {
                1 << (common_count - 1)
            } else {
                0
            }
        })
        .sum::<u32>();

    format!("{sum}")
}

#[aoc(day4, part2)]
pub fn part_2(input: &str) -> String {
    let matching_counts = input.lines().map(find_common_count).collect::<Vec<_>>();

    let mut collected_cards = vec![1; matching_counts.len()];

    for i in 0..matching_counts.len() {
        let card_count = collected_cards[i];
        let matching_count = matching_counts[i];

        for j in 0..matching_count {
            collected_cards[i + j + 1] += card_count;
        }
    }

    format!("{}", collected_cards.iter().sum::<u32>())
}
