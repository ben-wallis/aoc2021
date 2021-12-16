fn main() {
    let input: Vec<usize> = include_str!("..\\day7_input.txt")
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    println!("Answer: {}", solve(input.clone()));
}

fn solve(input: Vec<usize>) -> usize {
    const MAX_POS: usize = 1958;
    let mut crabs = [0; MAX_POS + 1 as usize];

    for i in input {
        crabs[i as usize] += 1;
    }

    (0..MAX_POS)
        .map(|pos| {
            (
                pos,
                crabs
                    .iter()
                    .enumerate()
                    .map(|(x, crabs)| {
                        calc_cost(((x as isize) - (pos as isize)).abs() as usize) as usize * crabs
                    })
                    .sum::<usize>(),
            )
        })
        .min_by_key(|(_, sum)| *sum)
        .map(|(_, sum)| sum)
        .unwrap()
}

fn calc_cost(num: usize) -> usize {
    let mut x = 0;
    (1..=num).for_each(|i| x += i);
    x
}
