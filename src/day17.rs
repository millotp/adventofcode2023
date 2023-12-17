use std::{
    collections::{BinaryHeap, HashMap},
    fs, hash,
};

const DX: [i16; 4] = [0, 1, 0, -1];
const DY: [i16; 4] = [-1, 0, 1, 0];

fn main() {
    let input = fs::read_to_string("inputs/day17.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    pos: (i16, i16),
    dist: u32,
    dir: i8,
    straights: i8,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

fn part1(input: &str) -> u64 {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut heap = BinaryHeap::new();
    heap.push(State {
        pos: (0, 0),
        dist: 0,
        dir: -1,
        straights: -1,
    });

    let mut visited = HashMap::<((i16, i16), i8, i8), u32>::new();
    let mut min_dist = u32::MAX;
    while let Some(s) = heap.pop() {
        let hash_key = (s.pos, s.dir, s.straights);
        if visited.contains_key(&hash_key) && visited[&hash_key] <= s.dist {
            continue;
        }
        visited.insert(hash_key, s.dist);
        if s.pos == (grid[0].len() as i16 - 1, grid.len() as i16 - 1) && s.dist < min_dist {
            min_dist = s.dist;
        }

        for i in 0..4 {
            if (i + 2) % 4 == s.dir {
                continue;
            }
            let new_pos = (s.pos.0 + DX[i as usize], s.pos.1 + DY[i as usize]);
            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 >= grid[0].len() as i16
                || new_pos.1 >= grid.len() as i16
            {
                continue;
            }
            let new_straights = if i == s.dir { s.straights + 1 } else { 1 };
            if new_straights > 3 {
                continue;
            }
            heap.push(State {
                pos: new_pos,
                dist: s.dist + grid[new_pos.1 as usize][new_pos.0 as usize] as u32,
                dir: i,
                straights: new_straights,
            })
        }
    }

    min_dist as u64
}

fn part2(input: &str) -> u64 {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (R, C) = (grid.len() as i16, grid[0].len() as i16);
    let mut heap = BinaryHeap::new();
    heap.push(State {
        pos: (0, 0),
        dist: 0,
        dir: -1,
        straights: -1,
    });

    let mut visited = HashMap::<((i16, i16), i8, i8), u32>::new();
    let mut min_dist = u32::MAX;
    while let Some(s) = heap.pop() {
        let hash_key = (s.pos, s.dir, s.straights);
        if visited.contains_key(&hash_key) {
            continue;
        }
        visited.insert(hash_key, s.dist);
        if s.pos == (C - 1, R - 1) && s.dist < min_dist {
            min_dist = s.dist;
        }
        for i in 0..4 {
            if (i + 2) % 4 == s.dir {
                continue;
            }
            let new_pos = (s.pos.0 + DX[i as usize], s.pos.1 + DY[i as usize]);
            if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= C || new_pos.1 >= R {
                continue;
            }
            let new_straights = if i == s.dir { s.straights + 1 } else { 1 };
            if new_straights > 10 || (i != s.dir && s.straights < 4 && s.straights != -1) {
                continue;
            }
            if new_pos.0 == C - 1 && new_pos.1 == R - 1 && new_straights < 4 {
                continue;
            }
            heap.push(State {
                pos: new_pos,
                dist: s.dist + grid[new_pos.1 as usize][new_pos.0 as usize] as u32,
                dir: i,
                straights: new_straights,
            })
        }
    }

    min_dist as u64
}
