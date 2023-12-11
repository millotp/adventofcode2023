use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day11.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn get_common(input: &str, expansion: i32) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let expand_y = grid
        .iter()
        .map(|r| !r.iter().any(|&c| c))
        .collect::<Vec<_>>();

    let mut expand_x = Vec::new();
    for i in 0..grid[0].len() {
        let mut have_galaxy = false;
        for row in grid.iter() {
            if row[i] {
                have_galaxy = true;
                break;
            }
        }
        expand_x.push(!have_galaxy);
    }

    let galaxies = grid
        .iter()
        .enumerate()
        .flat_map(|(j, r)| {
            r.iter()
                .enumerate()
                .filter(|(_, &c)| c)
                .map(|(i, _)| (i as i32, j as i32))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut dist = 0;
    for i in 0..(galaxies.len() - 1) {
        for j in i + 1..galaxies.len() {
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];
            let mut dx = (x1 - x2).abs();
            let mut dy = (y1 - y2).abs();
            for k in 0..=dx {
                if expand_x[(x1.min(x2) + k) as usize] {
                    dx += expansion;
                }
            }
            for k in 0..=dy {
                if expand_y[(y1.min(y2) + k) as usize] {
                    dy += expansion;
                }
            }
            dist += dx as u64 + dy as u64;
        }
    }

    dist
}

fn part1(input: &str) -> u64 {
    get_common(input, 1)
}

fn part2(input: &str) -> u64 {
    get_common(input, 999999)
}
