#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split("\n")
        .map(|str| str.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day1, part1, prev)]
pub fn part1(input: &[usize]) -> usize {
    let mut prev = usize::MAX;
    input.into_iter().fold(0, |acc, cur| {
        let mut ret = acc;
        if cur > &prev {
            ret = acc + 1;
        }
        prev = *cur;
        return ret;
    })
}

#[aoc(day1, part1, windows)]
pub fn part1windows(input: &[usize]) -> usize {
    input
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

#[aoc(day1, part2)]
pub fn part2(input: &[usize]) -> usize {
    let grouped_sums = input
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect::<Vec<_>>();

    grouped_sums
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}
