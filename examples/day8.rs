use aoc2021::utils::read_lines;

fn main() {
    let input: Vec<String> = read_lines("C:\\rust\\aoc2021\\day8_input.txt").collect();

    let data = input
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            let signals: Vec<&str> = parts[0].split(' ').filter(|x| x.len() > 0).collect();
            let outputs: Vec<&str> = parts[1].split(' ').filter(|x| x.len() > 0).collect();
            (signals, outputs)
        })
        .collect::<Vec<(Vec<&str>, Vec<&str>)>>();
    let y = solve(data);
    //dbg!(data);
    println!("Answer: {:?}", y);
}

fn solve(input: Vec<(Vec<&str>, Vec<&str>)>) -> u32 {
    input
        .into_iter()
        .flat_map(|x| x.1)
        .map(|x| match x.len() {
            2 => 1,
            3 => 1,
            4 => 1,
            7 => 1,
            _ => 0,
        })
        .sum()
}

fn solve2(input: Vec<(Vec<&str>, Vec<&str>)>) -> u32 {
    for (signals, output) in input {
        let two_chars = signals.iter().filter(|x| x.len() == 2); // 1 - cf
        let four_chars = signals.iter.filter(|x| x.len() == 4); // 4 - bcdf
        let three_chars = signals.iter.filter(|x| x.len() == 3); // 7 - acf
        let seven_chars = signals.iter.filter(|x| x.len() == 7); // 8 - abcdefg
        let a = seven_chars.chars().
    }

    0
}
