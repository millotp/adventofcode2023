use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day12.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn is_valid(ini: &str, arran: &[i32]) -> bool {
    let mut group = 0;
    let mut current_match = 0;
    for j in ini.chars() {
        if j == '#' {
            group += 1;
        } else if group > 0 {
            if current_match < arran.len() && group == arran[current_match] {
                current_match += 1;
            } else {
                break;
            }
            group = 0;
        }
    }

    current_match == arran.len()
        || (current_match == arran.len() - 1 && group == arran[current_match])
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (ini, arran) = l.split_once(" ").unwrap();
            let arran = arran
                .split(",")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let damaged_count = ini.chars().filter(|&c| c == '#').count();
            let total_target = arran.iter().sum::<i32>() - damaged_count as i32;
            let missing = ini.chars().filter(|&c| c == '?').count();

            let mut matches = 0;

            for i in 0..2usize.pow(missing as u32) {
                if i.count_ones() as i32 != total_target {
                    continue;
                }

                let mut combi = String::with_capacity(ini.len());
                let mut k = 0;
                for c in ini.chars() {
                    if c == '?' {
                        if i & (1 << k) > 0 {
                            combi.push('#');
                        } else {
                            combi.push('.');
                        }
                        k += 1;
                    } else {
                        combi.push(c);
                    }
                }

                if is_valid(&combi, &arran) {
                    matches += 1;
                }
            }

            matches as u64
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut lines = Vec::new();
    let (mut currIni, mut currArran) = (Vec::new(), Vec::new());
    let mut count = 0;
    for l in input.lines() {
        let (ini, arran) = l.split_once(" ").unwrap();
        currIni.push(ini);
        currArran.push(arran);
        count += 1;
        if count == 5 {
            lines.push(format!("{} {}", currIni.join("?"), currArran.join(",")));
            currIni.clear();
            currArran.clear();
            count = 0;
        }
    }

    lines
        .iter()
        .map(|l| {
            let (ini, arran) = l.split_once(" ").unwrap();
            let arran = arran
                .split(",")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let (ini, arran) = l.split_once(" ").unwrap();
            let arran = arran
                .split(",")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let damaged_count = ini.chars().filter(|&c| c == '#').count();
            let total_target = arran.iter().sum::<i32>() - damaged_count as i32;
            let missing = ini.chars().filter(|&c| c == '?').count();

            let mut matches = 0;

            println!("{}", 2usize.pow(missing as u32));

            0
        })
        .sum()
}
