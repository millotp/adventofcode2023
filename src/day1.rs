use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day1.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .split("\n")
        .map(|line| -> usize {
            (line.chars().find(|c| c.is_numeric()).unwrap_or('0') as usize - '0' as usize) * 10
                + (line.chars().rev().find(|c| c.is_numeric()).unwrap_or('0') as usize
                    - '0' as usize)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .split("\n")
        .map(|line| -> usize { (get_first_number(line) * 10) + get_last_number(line) })
        .sum()
}

fn get_first_number(input: &str) -> usize {
    let mut pos_digit = input
        .chars()
        .position(|c| c.is_numeric())
        .unwrap_or(input.len() * 2);

    let mut digit = input
        .chars()
        .find(|c| c.is_numeric())
        .unwrap_or('0')
        .to_digit(10)
        .unwrap() as usize;

    for (val, spelled) in [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    {
        let pos = input.find(spelled).unwrap_or(input.len() * 2);
        if pos < pos_digit {
            digit = val + 1;
            pos_digit = pos;
        }
    }

    return digit as usize;
}

fn get_last_number(input: &str) -> usize {
    let rev = input.chars().rev().collect::<String>();
    let mut pos_digit = rev
        .chars()
        .position(|c| c.is_numeric())
        .unwrap_or(input.len() * 2);

    let mut digit = rev
        .chars()
        .find(|c| c.is_numeric())
        .unwrap_or('0')
        .to_digit(10)
        .unwrap() as usize;

    for (val, spelled) in [
        "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ]
    .iter()
    .enumerate()
    {
        let pos = rev.find(spelled).unwrap_or(input.len() * 2);
        if pos < pos_digit {
            digit = val + 1;
            pos_digit = pos;
        }
    }

    return digit as usize;
}
