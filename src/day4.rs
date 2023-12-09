use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("inputs/day4.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (winning, pulled) = line.split_once(":").unwrap().1.split_once("|").unwrap();
            let winning_set = winning
                .split(' ')
                .filter(|num| !num.is_empty())
                .collect::<HashSet<&str>>();

            let wins = pulled
                .split(' ')
                .filter(|num| winning_set.contains(num))
                .count();

            if wins == 0 {
                0
            } else {
                2usize.pow(wins as u32 - 1)
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let wins = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (winning, pulled) = line.split_once(":").unwrap().1.split_once("|").unwrap();
            let winning_set = winning
                .split(' ')
                .filter(|num| !num.is_empty())
                .collect::<HashSet<&str>>();

            pulled
                .split(' ')
                .filter(|num| winning_set.contains(num))
                .count()
        })
        .collect::<Vec<usize>>();

    let mut copies = vec![1; wins.len()];

    for i in 0..wins.len() {
        for j in 0..wins[i] {
            copies[i + j + 1] += copies[i];
        }
    }

    copies.iter().sum()
}
