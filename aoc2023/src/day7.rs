#![allow(clippy::cast_possible_truncation)]

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Debug)]
struct HandBid {
    hand: Hand,
    bid: u32,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u8>,
    ty: Type,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

fn determine_type(cards: Vec<u8>) -> Hand {
    let mut count_map = cards.iter().counts();
    let j_count = count_map.remove(&1).unwrap_or(0);
    let mut count_map = count_map
        .into_iter()
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .collect_vec();

    if count_map.is_empty() {
        return Hand {
            cards,
            ty: Type::FiveKind,
        };
    }

    count_map[0].1 += j_count;

    let ty = if count_map.len() == 1 {
        Type::FiveKind
    } else if count_map[0].1 == 4 {
        Type::FourKind
    } else if count_map[0].1 == 3 && count_map[1].1 == 2 {
        Type::FullHouse
    } else if count_map[0].1 == 3 {
        Type::ThreeKind
    } else if count_map[0].1 == 2 && count_map[1].1 == 2 {
        Type::TwoPair
    } else if count_map[0].1 == 2 {
        Type::OnePair
    } else {
        Type::HighCard
    };

    Hand { cards, ty }
}

const fn char_map(c: char, joker: bool) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if joker {
                1
            } else {
                11
            }
        }
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u8,
    }
}

fn parse_input(input: &str, joker: bool) -> Vec<HandBid> {
    input
        .lines()
        .map(|line| {
            let mut splits = line.split(' ');
            let hand = splits
                .next()
                .unwrap()
                .chars()
                .map(|c| char_map(c, joker))
                .collect();
            let bid = splits.next().unwrap().parse::<u32>().unwrap();
            HandBid {
                hand: determine_type(hand),
                bid,
            }
        })
        .collect()
}

fn sort_handbids(input: &mut [HandBid]) {
    input.sort_by(|a, b| {
        if a.hand.ty == b.hand.ty {
            a.hand
                .cards
                .iter()
                .zip(b.hand.cards.iter())
                .map(|(a, b)| a.cmp(b))
                .find(|&x| x != std::cmp::Ordering::Equal)
                .unwrap_or(std::cmp::Ordering::Equal)
        } else {
            a.hand.ty.cmp(&b.hand.ty)
        }
    });
}

fn total_winnings(mut input: Vec<HandBid>) -> u32 {
    sort_handbids(&mut input);
    input
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum()
}

#[aoc(day7, part1)]
pub fn part_1(input: &str) -> String {
    let input = parse_input(input, false);

    format!("{}", total_winnings(input))
}

#[aoc(day7, part2)]
pub fn part_2(input: &str) -> String {
    let input = parse_input(input, true);

    format!("{}", total_winnings(input))
}
