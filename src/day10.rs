use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("inputs/day10.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

type Point = (usize, usize);
type Fence = Vec<Point>;

fn get_loop(input: &str) -> (Vec<Vec<(char, [bool; 4])>>, Fence, Fence) {
    let symbols = HashMap::from([
        ('|', [true, false, true, false]),
        ('-', [false, true, false, true]),
        ('L', [true, true, false, false]),
        ('J', [true, false, false, true]),
        ('7', [false, false, true, true]),
        ('F', [false, true, true, false]),
        ('.', [false, false, false, false]),
        ('S', [false, false, false, false]),
    ]);
    let mut start_pos = (0, 0);
    let mut pipes = input
        .lines()
        .enumerate()
        .map(|(j, l)| {
            l.chars()
                .enumerate()
                .map(|(i, c)| {
                    if c == 'S' {
                        start_pos = (i, j);
                    }
                    (c, symbols[&c])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if start_pos.1 > 0 && pipes[start_pos.1 - 1][start_pos.0].1[2] {
        pipes[start_pos.1][start_pos.0].1[0] = true;
    }
    if start_pos.0 < pipes[start_pos.1].len() - 1 && pipes[start_pos.1][start_pos.0 + 1].1[3] {
        pipes[start_pos.1][start_pos.0].1[1] = true;
    }
    if start_pos.1 < pipes.len() - 1 && pipes[start_pos.1 + 1][start_pos.0].1[0] {
        pipes[start_pos.1][start_pos.0].1[2] = true;
    }
    if start_pos.0 > 0 && pipes[start_pos.1][start_pos.0 - 1].1[1] {
        pipes[start_pos.1][start_pos.0].1[3] = true;
    }

    let mut curr = start_pos;
    let mut fence = Vec::new();
    let mut poly = Vec::new();
    let mut dir = pipes[curr.1][curr.0]
        .1
        .iter()
        .enumerate()
        .find(|(_, &v)| v)
        .unwrap()
        .0;
    while fence.is_empty() || curr != start_pos {
        match dir {
            0 => curr.1 -= 1,
            1 => curr.0 += 1,
            2 => curr.1 += 1,
            3 => curr.0 -= 1,
            _ => unreachable!(),
        }

        let new_dir = pipes[curr.1][curr.0]
            .1
            .iter()
            .enumerate()
            .filter(|(d, _)| *d != (dir + 2) % 4)
            .find(|(_, &v)| v)
            .unwrap()
            .0;
        if new_dir != dir {
            poly.push(curr);
        }
        dir = new_dir;
        fence.push(curr);
    }

    (pipes, fence, poly)
}

fn part1(input: &str) -> u64 {
    get_loop(input).1.len() as u64 / 2
}

fn inside(poly: &[Point], point: Point) -> bool {
    let mut cross = false;
    for i in 0..poly.len() {
        let j = (i + 1) % poly.len();
        if ((poly[i].1 > point.1) != (poly[j].1 > point.1))
            && ((point.0 as f32)
                < (poly[j].0 as f32 - poly[i].0 as f32) * (point.1 as f32 - poly[i].1 as f32)
                    / (poly[j].1 as f32 - poly[i].1 as f32)
                    + poly[i].0 as f32)
        {
            cross = !cross;
        }
    }

    cross
}

fn part2(input: &str) -> u64 {
    let (pipes, fence, poly) = get_loop(input);
    let fence = fence.iter().collect::<HashSet<_>>();
    pipes
        .iter()
        .enumerate()
        .map(|(j, line)| {
            line.iter()
                .enumerate()
                .map(|(i, _)| (!fence.contains(&(i, j)) && inside(&poly, (i, j))))
                .filter(|&b| b)
                .count() as u64
        })
        .sum::<u64>()
}
