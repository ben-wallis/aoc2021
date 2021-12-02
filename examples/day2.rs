use aoc2021::utils::read_lines;
use std::convert::TryInto;

fn main() {
    let input: Vec<String> = read_lines("C:\\rust\\aoc2021\\day2_input.txt").collect();

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input))
}

fn part1(input: Vec<String>) -> usize {
    let mut depth = 0;
    let mut hor = 0;
    input.iter().for_each(|x| {
        let split = x.split(' ').collect::<Vec<&str>>();
        let (command, value) = (split[0], split[1].parse::<u32>().unwrap());
        match command {
            "forward" => hor += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => {}
        };
    });

    (depth * hor).try_into().unwrap()
}

fn part2(input: Vec<String>) -> usize {
    let mut depth = 0;
    let mut hor = 0;
    let mut aim = 0;
    input.iter().for_each(|x| {
        let split = x.split(' ').collect::<Vec<&str>>();
        let (command, value) = (split[0], split[1].parse::<u32>().unwrap());
        match command {
            "forward" => {
                hor += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => {}
        };
    });

    (depth * hor).try_into().unwrap()
}
