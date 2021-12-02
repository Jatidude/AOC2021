pub enum Instruction {
    Up(usize),
    Down(usize),
    Forward(usize),
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .split("\n")
        .map(|str| {
            let mut instruction_iter = str.split(" ");
            return match instruction_iter.next().unwrap() {
                "up" => Instruction::Up(instruction_iter.next().unwrap().parse().unwrap()),
                "down" => Instruction::Down(instruction_iter.next().unwrap().parse().unwrap()),
                "forward" => {
                    Instruction::Forward(instruction_iter.next().unwrap().parse().unwrap())
                }
                _ => panic!("The input is malformed"),
            };
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Instruction]) -> usize {
    let (x, y): (usize, usize) =
        input
            .into_iter()
            .fold((0, 0), |acc, instruction| match instruction {
                Instruction::Up(s) => (acc.0, acc.1 - s),
                Instruction::Down(s) => (acc.0, acc.1 + s),
                Instruction::Forward(s) => (acc.0 + s, acc.1),
            });

    return x * y;
}

#[aoc(day2, part2)]
pub fn part2(input: &[Instruction]) -> usize {
    let (x, y, _aim): (usize, usize, usize) =
        input
            .into_iter()
            .fold((0, 0, 0), |acc, instruction| match instruction {
                Instruction::Up(s) => (acc.0, acc.1, acc.2 - s),
                Instruction::Down(s) => (acc.0, acc.1, acc.2 + s),
                Instruction::Forward(s) => (acc.0 + s, acc.1 + (acc.2 * s), acc.2),
            });

    return x * y;
}
