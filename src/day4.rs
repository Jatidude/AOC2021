#[derive(Debug, Clone)]
pub struct BingoGame {
    numbers: Vec<usize>,
    boards: Vec<BingoBoard>,
}
#[derive(Debug, Clone)]
struct BingoBoard {
    has_won: bool,
    board: Vec<Vec<(usize, bool)>>,
}

impl BingoBoard {
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, Vec<(usize, bool)>> {
        self.board.iter_mut()
    }

    fn check_for_win(&mut self) -> bool {
        for i in 0..5 {
            if (self[i][0].1 && self[i][1].1 && self[i][2].1 && self[i][3].1 && self[i][4].1)
                || (self[0][i].1 && self[1][i].1 && self[2][i].1 && self[3][i].1 && self[4][i].1)
            {
                self.has_won = true;
                return true;
            }
        }
        return false;
    }

    fn calc_board_score(&self, number: usize) -> usize {
        let sum = self.into_iter().fold(0, |acc, row| {
            acc + row
                .into_iter()
                .filter_map(|(num, bool)| match bool {
                    false => Some(num),
                    true => None,
                })
                .sum::<usize>()
        });
        number * sum
    }
}

impl FromIterator<Vec<(usize, bool)>> for BingoBoard {
    fn from_iter<T: IntoIterator<Item = Vec<(usize, bool)>>>(iter: T) -> Self {
        BingoBoard {
            has_won: false,
            board: iter.into_iter().collect(),
        }
    }
}

impl std::ops::Index<usize> for BingoBoard {
    type Output = Vec<(usize, bool)>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.board[index]
    }
}

impl IntoIterator for &BingoBoard {
    type Item = Vec<(usize, bool)>;
    type IntoIter = std::vec::IntoIter<Vec<(usize, bool)>>;

    fn into_iter(self) -> Self::IntoIter {
        self.board.clone().into_iter()
    }
}

impl BingoGame {
    fn play(&mut self, number: &usize) {
        self.boards.iter_mut().for_each(|board| {
            board.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|cell| {
                    if cell.0 == *number {
                        cell.1 = true;
                    }
                });
            });
            board.check_for_win();
        });
    }
    fn check_for_winner(&mut self) -> Option<usize> {
        for (index, board) in self.boards.clone().iter_mut().enumerate() {
            if board.check_for_win() {
                return Some(index);
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
            Some(board_index) => {
                result = our_game.boards[board_index].calc_board_score(*number);
                break;
            }
            None => {
                continue;
            }
        }
    }
    return result;
}

#[aoc(day4, part2)]
pub fn part2(game: &BingoGame) -> usize {
    let mut our_game = game.clone();
    let mut result = 0;
    for number in &game.numbers {
        our_game.play(number);
        if our_game.boards.len() == 1 && our_game.boards[0].has_won {
            result = our_game.boards[0].calc_board_score(*number);
            break;
        }
        our_game.boards = our_game
            .boards
            .into_iter()
            .filter(|board| !board.has_won)
            .collect();
    }
    result
}
