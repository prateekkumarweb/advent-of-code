use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day2, part1)]
pub fn part_1(input: &str) -> String {
    let re = Regex::new(r"Game (\d+): (.*)").unwrap();
    let (red, green, blue) = (12, 13, 14);

    let numbers = input.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        let game = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let line = caps.get(2).unwrap().as_str();
        let rounds = line.split("; ").all(|l| {
            let splits = l.split(", ");
            let (mut r, mut g, mut b) = (0, 0, 0);
            for sp in splits {
                let mut sp = sp.split(' ');
                let num = sp.next().unwrap().parse::<u32>().unwrap();
                let color = sp.next().unwrap();
                match color {
                    "red" => {
                        r = num;
                    }
                    "green" => {
                        g = num;
                    }
                    "blue" => {
                        b = num;
                    }
                    _ => {}
                }
            }
            r <= red && b <= blue && g <= green
        });
        if rounds { game } else { 0 }
    });

    format!("{}", numbers.sum::<u32>())
}

#[aoc(day2, part2)]
pub fn part_2(input: &str) -> String {
    let re = Regex::new(r"Game (\d+): (.*)").unwrap();

    let numbers = input.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        let line = caps.get(2).unwrap().as_str();
        let rounds = line
            .split("; ")
            .map(|l| {
                let splits = l.split(", ");
                let (mut r, mut g, mut b) = (0, 0, 0);
                for sp in splits {
                    let mut sp = sp.split(' ');
                    let num = sp.next().unwrap().parse::<u32>().unwrap();
                    let color = sp.next().unwrap();
                    match color {
                        "red" => {
                            r = num;
                        }
                        "green" => {
                            g = num;
                        }
                        "blue" => {
                            b = num;
                        }
                        _ => {}
                    }
                }
                (r, g, b)
            })
            .reduce(|a, b| (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2)))
            .unwrap();

        rounds.0 * rounds.1 * rounds.2
    });

    format!("{}", numbers.sum::<u32>())
}
