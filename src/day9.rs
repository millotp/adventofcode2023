use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day9.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn get_next(curr: &[i64]) -> Vec<i64> {
    let mut next = vec![0; curr.len() - 1];
    for i in 0..next.len() {
        next[i] = curr[i + 1] - curr[i];
    }

    next
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut curr = l
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let mut last_sum = *curr.last().unwrap();
            while curr.iter().any(|x| *x != 0) {
                curr = get_next(&curr);
                last_sum += *curr.last().unwrap();
            }

            last_sum
        })
        .sum::<i64>() as u64
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut curr = l
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let mut firsts = vec![curr[0]];

            while curr.iter().any(|x| *x != 0) {
                curr = get_next(&curr);
                firsts.push(curr[0]);
            }

            let mut sum = 0;
            for v in firsts.iter().rev() {
                sum = v - sum;
            }

            sum
        })
        .sum::<i64>() as u64
}
