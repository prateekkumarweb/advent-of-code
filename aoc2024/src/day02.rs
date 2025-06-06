use aoc::aoc;

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let lines = input.lines();
    let result = lines
        .map(|line| {
            let line = line.split(' ').filter(|p| !p.is_empty());
            let diff = line
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<i32>>();
            let safe = diff.iter().all(|&x| x == -1 || x == -2 || x == -3)
                || diff.iter().all(|&x| x == 1 || x == 2 || x == 3);
            i32::from(safe)
        })
        .sum::<i32>();
    result
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let lines = input.lines();
    let result = lines
        .map(|line| {
            let line = line.split(' ').filter(|p| !p.is_empty());
            let diff = line
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<i32>>();
            let mut safe = diff.iter().all(|&x| x == -1 || x == -2 || x == -3)
                || diff.iter().all(|&x| x == 1 || x == 2 || x == 3);
            if !safe {
                for i in 1..diff.len() {
                    let mut diff2 = diff.clone();
                    diff2[i] += diff2[i - 1];
                    diff2.remove(i - 1);
                    safe = diff2.iter().all(|&x| x == -1 || x == -2 || x == -3)
                        || diff2.iter().all(|&x| x == 1 || x == 2 || x == 3);
                    if safe {
                        break;
                    }
                }
                if !safe {
                    let mut diff2 = diff.clone();
                    diff2.remove(0);
                    safe = diff2.iter().all(|&x| x == -1 || x == -2 || x == -3)
                        || diff2.iter().all(|&x| x == 1 || x == 2 || x == 3);
                }
                if !safe {
                    let mut diff2 = diff;
                    diff2.remove(diff2.len() - 1);
                    safe = diff2.iter().all(|&x| x == -1 || x == -2 || x == -3)
                        || diff2.iter().all(|&x| x == 1 || x == 2 || x == 3);
                }
            }
            i32::from(safe)
        })
        .sum::<i32>();
    result
}
