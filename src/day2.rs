use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day2.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .split("\r\n")
        .enumerate()
        .map(|(index, line)| -> usize {
            let game = line.splitn(2, ": ").nth(1).unwrap();
            let draws = game.split("; ").collect::<Vec<&str>>();
            if draws.iter().all(|draw| {
                let mut gems = draw.split(", ").map(|gem| {
                    let cubes = gem.split(" ").collect::<Vec<&str>>();
                    (cubes[0].parse::<usize>().unwrap(), cubes[1])
                });

                gems.all(|(count, color)| match color {
                    "red" => count <= 12,
                    "green" => count <= 13,
                    "blue" => count <= 14,
                    _ => false,
                })
            }) {
                index + 1
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .split("\r\n")
        .map(|line| -> usize {
            let game = line.splitn(2, ": ").nth(1).unwrap();
            let draws = game.split("; ").collect::<Vec<&str>>();

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            draws.iter().for_each(|draw| {
                let gems = draw.split(", ").map(|gem| {
                    let cubes = gem.split(" ").collect::<Vec<&str>>();
                    (cubes[0].parse::<usize>().unwrap(), cubes[1])
                });

                gems.for_each(|(count, color)| match color {
                    "red" => max_red = max_red.max(count),
                    "green" => max_green = max_green.max(count),
                    "blue" => max_blue = max_blue.max(count),
                    _ => (),
                });
            });

            max_red * max_green * max_blue
        })
        .sum()
}
