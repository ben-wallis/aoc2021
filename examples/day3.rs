use aoc2021::utils::read_lines;
use std::convert::TryInto;

fn main() {
    let input: Vec<usize> = read_lines("C:\\rust\\aoc2021\\day3_input.txt")
        .map(|x| usize::from_str_radix(&x, 2).unwrap())
        .collect();

    // println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input))
}

fn part1(input: Vec<usize>) -> usize {
    const MAX_COL: usize = 11;
    let gamma_text = (0..=MAX_COL)
        .map(|i| {
            let sum: usize = input
                .iter()
                .map(|x| if x & (1 << (MAX_COL - i)) > 0 { 1 } else { 0 })
                .sum();
            if sum > input.len() / 2 {
                1
            } else {
                0
            }
        })
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>()
        .join("");

    let gamma = usize::from_str_radix(&gamma_text, 2).unwrap();
    let epsilon = gamma ^ 0b111111111111;

    println!(
        "Gamma: {:b} Epsilon: {:b} Power: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
    0
}

fn part2(mut input: Vec<usize>) -> usize {
    const MAX_COL: usize = 11;
    let gamma_text = (0..=MAX_COL)
        .map(|i| {
            let sum: usize = input
                .iter()
                .map(|x| (x & (1 << (MAX_COL - i))).min(1))
                .sum();

            println!("sum: {} input len /2: {}", sum, input.len() / 2);
            let most_common = if sum >= (input.len() - sum) { 0 } else { 1 };

            if input.len() > 1 {
                input.retain(|x| (x & (1 << (MAX_COL - i))).min(1) == most_common);
            }
            dbg!(most_common);
            most_common
        })
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>()
        .join("");

    dbg!(input);

    0
}
