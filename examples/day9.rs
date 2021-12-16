use aoc2021::utils::read_lines;
use std::collections::HashSet;

fn main() {
    let input: Vec<String> = read_lines("C:\\rust\\aoc2021\\day9_input.txt").collect();

    let row_length = input[0].len();
    let data = input
        .concat()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let part1 = solve(data.clone(), row_length);
    let part2 = solve2(data, row_length);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

fn solve(input: Vec<u32>, row_length: usize) -> u32 {
    let risk = input
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, height)| {
            let left = if i % row_length == 0 {
                10
            } else {
                input[i - 1]
            };
            let right = if i % row_length == row_length - 1 {
                10
            } else {
                input[i + 1]
            };
            let up = if i < row_length {
                10
            } else {
                input[i - row_length]
            };
            let down = if i + row_length >= input.len() {
                10
            } else {
                input[i + row_length]
            };

            if height < left && height < right && height < up && height < down {
                1 + height
            } else {
                0
            }
        })
        .sum();
    risk
}

fn solve2(input: Vec<u32>, row_length: usize) -> u32 {
    let mut basin_sizes: Vec<usize> = Vec::new();
    let mut basin_members: HashSet<usize> = HashSet::new();

    for (i, height) in input.clone().into_iter().enumerate() {
        if height == 9 {
            // It's a basin wall, skip it
            continue;
        }
        if basin_members.contains(&i) {
            // It's already in a basin, skip it
            continue;
        }

        // We've found the start of a new basin
        let mut current_basin_members: HashSet<usize> = HashSet::new();
        current_basin_members.insert(i);
        let mut scanned_basin_members: HashSet<usize> = HashSet::new();

        loop {
            if let Some(z) = current_basin_members
                .clone()
                .iter()
                .filter(|x| !scanned_basin_members.contains(x))
                .next()
            {
                let mut add_if_valid = |y: usize| {
                    if input[y] != 9 && !current_basin_members.contains(&y) {
                        current_basin_members.insert(y);
                    }
                };
                if z % row_length > 0 {
                    add_if_valid(z - 1)
                };
                if z % row_length != row_length - 1 {
                    add_if_valid(z + 1)
                }
                if *z >= row_length {
                    add_if_valid(z - row_length)
                }
                if z + row_length < input.len() {
                    add_if_valid(z + row_length)
                }
                scanned_basin_members.insert(*z);
            } else {
                basin_members.extend(current_basin_members.clone());
                basin_sizes.push(current_basin_members.len());
                break;
            }
        }
    }

    basin_sizes.sort();
    basin_sizes
        .iter()
        .rev()
        .take(3)
        .fold(1, |acc, &x| acc * x as u32)
}
