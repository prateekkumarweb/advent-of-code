use aoc::aoc;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.len() - {
                let s = &l[1..l.len() - 1];
                let mut chars = s.chars().peekable();
                let mut count = 0;
                while let Some(c) = chars.next() {
                    if c == '\\' {
                        if let Some(&n) = chars.peek() {
                            if n == '"' || n == '\\' {
                                chars.next().unwrap();
                            } else if n == 'x' {
                                chars.next().unwrap();
                                chars.next().unwrap();
                                chars.next().unwrap();
                            }
                        }
                    }
                    count += 1;
                }
                count
            }
        })
        .sum()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.chars().filter(|&c| c == '"' || c == '\\').count() + 2)
        .sum()
}
