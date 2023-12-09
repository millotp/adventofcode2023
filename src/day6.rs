use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day6.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let (times, dists) = input.split_once("\n").unwrap();
    let times = times
        .split_once(":")
        .unwrap()
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    let dists = dists
        .split_once(":")
        .unwrap()
        .1
        .replace("\n", "")
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<f64>().unwrap() + 1.0)
        .collect::<Vec<_>>();
    let runs = times.iter().zip(dists).collect::<Vec<_>>();

    let mut res = 1;
    for (t, d) in runs {
        let delta = *t * *t - 4.0 * d;
        if delta < 0.0 {
            continue;
        }

        let x1 = ((*t - delta.sqrt()) / 2.0).ceil() as u64;
        let x2 = ((*t + delta.sqrt()) / 2.0).floor() as u64;
        res *= x2 - x1 + 1;
    }

    res
}

fn part2(input: &str) -> u64 {
    let (times, dists) = input.split_once("\n").unwrap();
    let t = times
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<f64>()
        .unwrap();
    let d = dists
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .replace("\n", "")
        .parse::<f64>()
        .unwrap()
        + 1.0;

    let delta = t * t - 4.0 * d;

    let x1 = ((t - delta.sqrt()) / 2.0).ceil() as u64;
    let x2 = ((t + delta.sqrt()) / 2.0).floor() as u64;

    return x2 - x1 + 1;
}
