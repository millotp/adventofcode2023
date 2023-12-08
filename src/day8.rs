use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("inputs/day8.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let path = lines.nth(0).unwrap();
    let paths = lines
        .skip(1)
        .map(|l| l.split_once(" = ").unwrap())
        .map(|(p, d)| {
            let dirs = d.to_owned().replace("(", "").replace(")", "").to_owned();
            let dirs = dirs.split_once(", ").unwrap();

            (p, (dirs.0.to_owned(), dirs.1.to_owned()))
        })
        .collect::<HashMap<_, _>>();
    let mut curr = "AAA";
    let mut steps = 0;
    while curr != "ZZZ" {
        for d in path.chars() {
            curr = match d {
                'L' => &paths[&curr].0,
                'R' => &paths[&curr].1,
                _ => panic!("Invalid direction"),
            };
            steps += 1;
        }
    }

    steps
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let path = lines.nth(0).unwrap();
    let mut starting_points = Vec::new();
    let paths = lines
        .skip(1)
        .map(|l| l.split_once(" = ").unwrap())
        .map(|(p, d)| {
            let dirs = d.to_owned().replace("(", "").replace(")", "").to_owned();
            let dirs = dirs.split_once(", ").unwrap();

            if p.ends_with("A") {
                starting_points.push(p);
            }

            (p, (dirs.0.to_owned(), dirs.1.to_owned()))
        })
        .collect::<HashMap<_, _>>();
    let mut currs = starting_points.clone();
    let mut lens = Vec::new();
    let mut steps = 0;
    while !currs.is_empty() {
        for d in path.chars() {
            for curr in currs.iter_mut() {
                *curr = match d {
                    'L' => &paths[curr].0,
                    'R' => &paths[curr].1,
                    _ => panic!("Invalid direction"),
                };

                if curr.ends_with("Z") {
                    lens.push(steps + 1);
                }
            }
            steps += 1;

            currs = currs
                .iter()
                .filter(|c| !c.ends_with("Z"))
                .map(|c| *c)
                .collect();
        }
    }

    lcm(&lens)
}

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
