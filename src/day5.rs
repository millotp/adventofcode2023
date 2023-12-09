use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day5.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug)]
struct Conversion {
    dest_range: u64,
    source_range: u64,
    len: u64,
}

impl Conversion {
    fn convert(&self, n: u64) -> u64 {
        if n >= self.source_range && n < self.source_range + self.len {
            return self.dest_range + (n - self.source_range);
        }

        return n;
    }
}

fn part1(input: &str) -> u64 {
    let (first, rest) = input.split_once("\n").unwrap();
    let seeds = first
        .split_once(":")
        .unwrap()
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut tables = Vec::<Vec<Conversion>>::new();
    let mut currentTable = Vec::<Conversion>::new();

    for line in rest.lines() {
        if line.is_empty() && !currentTable.is_empty() {
            tables.push(currentTable);
            currentTable = Vec::<Conversion>::new();

            continue;
        }
        if line.chars().nth(0).unwrap_or('a').is_alphabetic() {
            continue;
        }

        let nums = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        currentTable.push(Conversion {
            dest_range: nums[0],
            source_range: nums[1],
            len: nums[2],
        })
    }

    let mut lowest = u64::MAX;
    for seed in seeds {
        let mut current = seed;
        for table in &tables {
            for conversion in table {
                let next = conversion.convert(current);
                if next != current {
                    current = next;
                    break;
                }
            }
        }

        if current < lowest {
            lowest = current;
        }
    }

    lowest as u64
}

fn part2(input: &str) -> u64 {
    let (first, rest) = input.split_once("\n").unwrap();
    let seeds = first
        .split_once(":")
        .unwrap()
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut tables = Vec::<Vec<Conversion>>::new();
    let mut currentTable = Vec::<Conversion>::new();

    for line in rest.lines() {
        if line.is_empty() && !currentTable.is_empty() {
            tables.push(currentTable);
            currentTable = Vec::<Conversion>::new();

            continue;
        }
        if line.chars().nth(0).unwrap_or('a').is_alphabetic() {
            continue;
        }

        let nums = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        currentTable.push(Conversion {
            dest_range: nums[0],
            source_range: nums[1],
            len: nums[2],
        })
    }

    let mut lowest = u64::MAX;
    for range in seeds.chunks(2) {
        for seed in range[0]..=(range[0] + range[1]) {
            let mut current = seed;
            for table in &tables {
                for conversion in table {
                    let next = conversion.convert(current);
                    if next != current {
                        current = next;
                        break;
                    }
                }
            }

            if current < lowest {
                lowest = current;
            }
        }
    }

    lowest as u64
}
