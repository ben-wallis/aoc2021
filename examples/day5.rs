use aoc2021::utils::read_lines;
use griditer::BresenhamIter;
use regex::{Captures, Regex};
use std::collections::HashMap;

fn main() {
    //let input: Vec<String> = read_lines("C:\\rust\\aoc2021\\day5_input.txt").collect();
    let input = include_str!("..\\day5_input.txt");

    println!("Part 1: {}", solve(input.clone(), false));
    println!("Part 2: {}", solve(input.clone(), true));
}

type Point = (isize, isize);

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl<'a> From<Captures<'a>> for Line {
    fn from(cap: Captures) -> Self {
        Line {
            start: (
                cap[1].parse::<isize>().unwrap(),
                cap[2].parse::<isize>().unwrap(),
            ),
            end: (
                cap[3].parse::<isize>().unwrap(),
                cap[4].parse::<isize>().unwrap(),
            ),
        }
    }
}
fn solve(input: &str, include_non_horizontal: bool) -> usize {
    let re = Regex::new(r"(\d+),(\d+)\s-> (\d+),(\d+)").unwrap();

    let mut grid = HashMap::<Point, u32>::new(); // 100x100
    let lines: Vec<Line> = re
        .captures_iter(input)
        .map(|cap| cap.into())
        .filter(|line: &Line| {
            include_non_horizontal || line.start.0 == line.end.0 || line.start.1 == line.end.1
        })
        .collect();

    for point in lines
        .iter()
        .flat_map(|line| BresenhamIter::new(line.start, line.end))
    {
        *grid.entry(point).or_insert(0) += 1
    }

    grid.iter().filter(|(k, v)| **v >= 2).count()
}
