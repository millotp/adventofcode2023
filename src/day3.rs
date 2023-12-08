use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("inputs/day3.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let input = input.lines().collect::<Vec<&str>>();
    let mut sum = 0;

    for (j, line) in input.iter().enumerate() {
        let mut num_buffer = String::new();
        let mut is_part = false;
        for (i, c) in line.chars().enumerate() {
            if !c.is_numeric() {
                if is_part && num_buffer.len() > 0 {
                    sum += num_buffer.parse::<usize>().unwrap();
                }

                is_part = false;
                num_buffer.clear();
                continue;
            }
            num_buffer.push(c);

            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 {
                        continue;
                    }

                    let x = i as i32 + x;
                    let y = j as i32 + y;
                    if x < 0 || y < 0 || x >= line.len() as i32 || y >= input.len() as i32 {
                        continue;
                    }

                    let neigh = input[y as usize].chars().nth(x as usize).unwrap();
                    if !neigh.is_numeric() && neigh != '.' {
                        is_part = true;
                    }
                }
            }
        }

        if is_part && num_buffer.len() > 0 {
            sum += num_buffer.parse::<usize>().unwrap();
        }
    }

    sum
}

fn part2(input: &str) -> usize {
    let input = input.lines().collect::<Vec<&str>>();
    let mut grid = Vec::new();

    for (j, line) in input.iter().enumerate() {
        let mut num_buffer = String::new();
        let mut row = (0..line.len())
            .map(|k| (k + j * line.len(), 0i32))
            .collect::<Vec<(usize, i32)>>();
        for (i, c) in line.chars().enumerate() {
            if !c.is_numeric() {
                if c == '*' {
                    row[i].1 = -1;
                }
                if num_buffer.len() > 0 {
                    let num = num_buffer.parse::<usize>().unwrap() as i32;
                    for k in 0..num_buffer.len() {
                        row[i - k - 1] = (i, num);
                    }
                }

                num_buffer.clear();
                continue;
            }
            num_buffer.push(c);
        }

        if num_buffer.len() > 0 {
            let num = num_buffer.parse::<usize>().unwrap() as i32;
            for k in 0..num_buffer.len() {
                row[line.len() - 1 - k].1 = num;
            }
        }

        grid.push(row);
    }

    let mut sum = 0;
    for (j, line) in grid.iter().enumerate() {
        for (i, cell) in line.iter().enumerate() {
            if cell.1 != -1 {
                continue;
            }

            let mut neighboors = HashSet::new();

            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 {
                        continue;
                    }

                    let x = i as i32 + x;
                    let y = j as i32 + y;
                    if x < 0 || y < 0 || x >= line.len() as i32 || y >= grid.len() as i32 {
                        continue;
                    }
                    let neigh = grid[y as usize][x as usize];

                    if neigh.1 > 0 {
                        neighboors.insert(neigh);
                    }
                }
            }

            if neighboors.len() == 2 {
                let neighboors = neighboors.iter().collect::<Vec<&(usize, i32)>>();
                sum += (neighboors[0].1 * neighboors[1].1) as usize;
            }
        }
    }

    sum
}
