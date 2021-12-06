use std::collections::HashMap;

fn main() {
    let input: Vec<u32> = include_str!("..\\day6_input.txt")
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    println!("Part 1: {}", solve(input.clone()));
    println!("Part 2: {}", solve2(input.clone()));
}

#[derive(Debug)]
struct LanternFish(u32);

fn solve(input: Vec<u32>) -> usize {
    const DAYS: usize = 256;

    let mut fishes: Vec<LanternFish> = input.iter().map(|x| LanternFish(*x)).collect();
    for i in 0..DAYS {
        println!("Day {}", i);
        let mut new_fish: Vec<LanternFish> = fishes
            .iter_mut()
            .filter_map(|fish| {
                let res = if fish.0 == 0 {
                    fish.0 = 7;
                    Some(LanternFish(8))
                } else {
                    None
                };
                fish.0 -= 1;
                res
            })
            .collect();
        fishes.append(&mut new_fish);
    }
    //dbg!(fishes);
    fishes.iter().map(|x| x.0).count()
}

fn solve2(input: Vec<u32>) -> u64 {
    const DAYS: usize = 256;

    let mut fishes = HashMap::<u32, u64>::new();
    for fish in input {
        *fishes.entry(fish).or_insert(0) += 1;
    }

    for i in 0..DAYS {
        println!("Day {}", i);
        // Should have used an array...
        let new_fish = *fishes.entry(0).or_insert(0);
        *fishes.entry(0).or_insert(0) = *fishes.entry(1).or_insert(0);
        *fishes.entry(1).or_insert(0) = *fishes.entry(2).or_insert(0);
        *fishes.entry(2).or_insert(0) = *fishes.entry(3).or_insert(0);
        *fishes.entry(3).or_insert(0) = *fishes.entry(4).or_insert(0);
        *fishes.entry(4).or_insert(0) = *fishes.entry(5).or_insert(0);
        *fishes.entry(5).or_insert(0) = *fishes.entry(6).or_insert(0);
        *fishes.entry(6).or_insert(0) = *fishes.entry(7).or_insert(0) + new_fish;
        *fishes.entry(7).or_insert(0) = *fishes.entry(8).or_insert(0);
        *fishes.entry(8).or_insert(0) = new_fish;
    }
    fishes.iter().map(|(_, v)| *v).sum()
}
