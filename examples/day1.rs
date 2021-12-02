use aoc2021::utils::read_lines;

fn main() {
    let input: Vec<u32> = read_lines("C:\\rust\\aoc2021\\day1_input.txt")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input))
}

fn part1(input: Vec<u32>) -> usize {
    let mut count = 0;
    input
        .into_iter()
        .scan(0, |prev, x| {
            count += if x > *prev { 1 } else { 0 };
            *prev = x;
            Some(x)
        })
        .for_each(drop);
    count
}

fn part2(input: Vec<u32>) -> usize {
    let mut count = 0;
    input
        .windows(3)
        .scan(0, |prev, window| {
            let window_sum = window.iter().sum();
            count += if window_sum > *prev { 1 } else { 0 };
            *prev = window_sum;
            Some(window)
        })
        .for_each(drop);
    count
}
