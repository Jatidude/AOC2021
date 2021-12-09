#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let vals = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .fold(([0; 12], 0), |mut acc, num| {
            for n in 0..12 {
                acc.0[n] += if (num & (0b000000000001 << 11 - n)) > 0 {
                    1
                } else {
                    0
                };
            }
            acc.1 += 1;
            return acc;
        });

    let gamma = vals.0.into_iter().enumerate().fold(0, |acc, (i, num)| {
        return acc ^ if num > vals.1 / 2 { 1 << 11 - i } else { 0 };
    });

    let epsilon = !gamma & 0b111111111111;

    return gamma * epsilon;
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let nums = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>();

    let mut oxygen = nums.clone();
    let mut co2 = nums.clone();

    for i in 0..12 {
        if 1 == oxygen.len() && 1 == co2.len() {
            break;
        }
        if oxygen.len() > 1 {
            oxygen = split_mask_for_size(oxygen, 0b000000000001 << 11 - i, true)
        }
        if co2.len() > 1 {
            co2 = split_mask_for_size(co2, 0b000000000001 << 11 - i, false)
        }
    }

    return oxygen[0] * co2[0];
}

fn split_mask_for_size(nums: Vec<usize>, mask: usize, bigger: bool) -> Vec<usize> {
    let split = nums.into_iter().fold((vec![], vec![]), |mut acc, num| {
        if num & mask > 0 {
            acc.0.push(num)
        } else {
            acc.1.push(num)
        }
        return acc;
    });

    if split.0.len() >= split.1.len() {
        if bigger {
            return split.0;
        } else {
            return split.1;
        }
    } else {
        if bigger {
            return split.1;
        } else {
            return split.0;
        }
    }
}
