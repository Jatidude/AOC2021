use std::collections::HashSet;

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[i32]) -> i32 {
    let mut locations = input.to_vec();
    locations.sort();
    let midpoint = locations.len() / 2;
    let median = if midpoint % 2 == 0 {
        (locations[midpoint] + locations[midpoint - 1]) / 2
    } else {
        locations[midpoint]
    };

    locations
        .into_iter()
        .map(|location| (location - median).abs())
        .sum()
}

fn calc_cost(start: i32, finish: i32) -> i32 {
    let distance = (start - finish).abs();
    (distance + 1) * distance / 2 //Sum of arithmetic sequence
}

#[aoc(day7, part2, bruteForce)]
fn part2(input: &[i32]) -> i32 {
    let mut crabs = input.to_vec();

    crabs.sort();

    let mut costs = HashSet::new();

    for target in crabs[0]..crabs[crabs.len() - 1] {
        costs.insert(input.iter().map(|crab| calc_cost(*crab, target)).sum());
    }

    *costs.iter().min().unwrap()
}
#[aoc(day7, part2, mean)]
fn part2mean(input: &[i32]) -> i32 {
    let mean: i32 = input.iter().sum::<i32>() / input.len() as i32;
    input.iter().map(|crab| calc_cost(*crab, mean)).sum()
}
