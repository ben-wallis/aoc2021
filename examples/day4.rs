use aoc2021::utils::read_lines;

fn main() {
    let input: Vec<String> = read_lines("C:\\rust\\aoc2021\\day4_input.txt").collect();

    println!("Part 1: {}", part2(input.clone(), false));
    println!("Part 2: {}", part2(input, true))
}

type Board = Vec<(u32, bool)>;

fn part2(input: Vec<String>, last_winner: bool) -> u32 {
    // Parse called numbers
    let called_numbers = input[0]
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // Parse boards
    let mut boards: Vec<Board> = input[2..]
        .chunks(6)
        .map(|board_input| {
            board_input
                .iter()
                .take(5)
                .flat_map(|line| {
                    line.split_whitespace()
                        .map(|x| (x.parse::<u32>().unwrap(), false))
                })
                .collect()
        })
        .collect();

    let mut winners = Vec::<usize>::new();
    let mut last_score = 0;
    for number in called_numbers {
        for (i, board) in &mut boards.iter_mut().enumerate() {
            if let Some((_, ref mut mark)) =
                board.iter_mut().find(|(val, _)| *val == number).as_mut()
            {
                *mark = true;

                let hor_win = board
                    .chunks(5)
                    .any(|chunk| chunk.iter().filter(|(_, mark)| *mark).count() == 5);

                let vert_win = (0..5).any(|i| {
                    board
                        .iter()
                        .skip(i)
                        .step_by(5)
                        .filter(|(_, mark)| *mark)
                        .count()
                        == 5
                });

                if (hor_win || vert_win) && !winners.iter().any(|x| *x == i) {
                    let unmarked_sum = board
                        .iter()
                        .filter_map(|(value, mark)| if !*mark { Some(value) } else { None })
                        .sum::<u32>();
                    last_score = unmarked_sum * number;
                    if !last_winner {
                        return last_score;
                    }
                    winners.push(i);
                }
            }
        }
    }
    last_score
}
