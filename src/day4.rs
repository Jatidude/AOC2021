#[derive(Debug, Clone)]
pub struct BingoGame {
    numbers: Vec<usize>,
    boards: Vec<Vec<Vec<(usize, bool)>>>,
}

impl BingoGame {
    fn play(&mut self, number: &usize) {
        self.boards.iter_mut().for_each(|board| {
            board.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|cell| {
                    if cell.0 == *number {
                        cell.1 = true;
                    }
                })
            })
        })
    }
    fn check_for_winner(&self) -> Option<&Vec<Vec<(usize, bool)>>> {
        for board in &self.boards {
            for i in 0..5 {
                if board[i][0].1 && board[i][1].1 && board[i][2].1 && board[i][3].1 && board[i][4].1
                {
                    println!("{:#?}", board);
                    return Some(board);
                }
                if board[0][i].1 && board[1][i].1 && board[2][i].1 && board[3][i].1 && board[4][i].1
                {
                    println!("{:?}", board);
                    return Some(board);
                }
            }
        }

        return None;
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> BingoGame {
    let sections = input.split_once("\n").unwrap();
    let numbers = sections
        .0
        .split(",")
        .map(|str| str.parse().unwrap())
        .collect();

    let boards = sections
        .1
        .trim()
        .split("\n\n")
        .map(|board_string| {
            board_string
                .split("\n")
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| (num.parse().unwrap(), false))
                        .collect()
                })
                .collect()
        })
        .collect();
    BingoGame { numbers, boards }
}

#[aoc(day4, part1)]
pub fn part1(game: &BingoGame) -> usize {
    let mut result = 0;
    let mut our_game = game.clone();
    for number in &game.numbers {
        our_game.play(number);
        match our_game.check_for_winner() {
            Some(board) => {
                result = number
                    * board.into_iter().fold(0, |acc, row| {
                        acc + row
                            .into_iter()
                            .filter_map(|(num, bool)| match bool {
                                false => Some(num),
                                true => None,
                            })
                            .sum::<usize>()
                    });
                break;
            }
            None => {
                continue;
            }
        }
    }
    return result;
}
