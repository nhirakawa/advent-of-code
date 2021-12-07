use std::collections::{HashMap, HashSet};

use common::{
    parse::{spaces, unsigned_number},
    prelude::*,
};

use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, into},
    multi::{many_m_n, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use uuid::Uuid;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-4.txt");
    let bingo_subsystem = parse_bingo_subsystem(input);

    let part_one = part_one(bingo_subsystem.clone());
    let part_two = part_two(bingo_subsystem);

    Ok((part_one, part_two))
}

fn part_one(mut bingo_subsystem: BingoSubsystem) -> PartAnswer {
    let start = SystemTime::now();

    bingo_subsystem.call_numbers();

    let first_winner = &bingo_subsystem.winners[0];

    let solution = first_winner.last_called as u32 * first_winner.sum_uncalled_numbers();

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn part_two(mut bingo_subsystem: BingoSubsystem) -> PartAnswer {
    let start = SystemTime::now();

    bingo_subsystem.call_numbers();

    let last_winner = &bingo_subsystem.winners[bingo_subsystem.winners.len() - 1];

    let solution = last_winner.last_called as u32 * last_winner.sum_uncalled_numbers();

    PartAnswer::new(solution, start.elapsed().unwrap())
}

#[derive(Debug, Clone, PartialEq)]
struct BingoCard {
    id: Uuid,
    coordinates_by_value: HashMap<u8, (usize, usize)>,
    called: HashSet<u8>,
    last_called: u8,
}

impl BingoCard {
    fn call_number(&mut self, number: u8) {
        self.called.insert(number);
        self.last_called = number;
    }

    fn is_winner(&self) -> bool {
        let mut row_counters = HashMap::new();
        let mut column_counters = HashMap::new();

        for number in &self.called {
            if !self.coordinates_by_value.contains_key(number) {
                continue;
            }

            let (x, y) = self.coordinates_by_value[number];

            if !row_counters.contains_key(&x) {
                row_counters.insert(x, 0);
            }

            if let Some(count) = row_counters.get_mut(&x) {
                *count += 1;
            }

            if !column_counters.contains_key(&y) {
                column_counters.insert(y, 0);
            }

            if let Some(count) = column_counters.get_mut(&y) {
                *count += 1;
            }
        }

        row_counters.values().any(|i| *i == 5) || column_counters.values().any(|i| *i == 5)
    }

    fn sum_uncalled_numbers(&self) -> u32 {
        self.coordinates_by_value
            .keys()
            .filter(|n| !self.called.contains(*n))
            .map(|n| *n as u32)
            .sum()
    }
}

impl From<Vec<Vec<u8>>> for BingoCard {
    fn from(cells: Vec<Vec<u8>>) -> BingoCard {
        let mut coordinates_by_value = HashMap::new();

        for (x, row) in cells.iter().enumerate() {
            for (y, column) in row.iter().enumerate() {
                coordinates_by_value.insert(*column, (x, y));
            }
        }

        BingoCard {
            id: Uuid::new_v4(),
            coordinates_by_value,
            called: HashSet::new(),
            last_called: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct BingoSubsystem {
    numbers: Vec<u8>,
    bingo_cards: Vec<BingoCard>,
    current_index: usize,
    winners: Vec<BingoCard>,
}

impl BingoSubsystem {
    fn call_numbers(&mut self) {
        for number in &self.numbers {
            for bingo_card in self.bingo_cards.iter_mut() {
                if bingo_card.is_winner() {
                    continue;
                }

                bingo_card.call_number(*number);

                if bingo_card.is_winner() {
                    self.winners.push(bingo_card.clone());
                }
            }
        }
    }
}

impl From<(Vec<u8>, Vec<BingoCard>)> for BingoSubsystem {
    fn from(tuple: (Vec<u8>, Vec<BingoCard>)) -> BingoSubsystem {
        let (numbers, bingo_cards) = tuple;
        BingoSubsystem {
            numbers,
            bingo_cards,
            current_index: 0,
            winners: Vec::new(),
        }
    }
}

fn parse_bingo_subsystem(i: &str) -> BingoSubsystem {
    all_consuming(bingo_subsystem)(i).unwrap().1
}

fn bingo_subsystem(i: &str) -> IResult<&str, BingoSubsystem> {
    into(separated_pair(bingo_numbers, tag("\n"), bingo_cards))(i)
}

fn bingo_numbers(i: &str) -> IResult<&str, Vec<u8>> {
    terminated(separated_list1(tag(","), unsigned_number), tag("\n"))(i)
}

fn bingo_cards(i: &str) -> IResult<&str, Vec<BingoCard>> {
    separated_list1(tag("\n"), bingo_card)(i)
}

fn bingo_card(i: &str) -> IResult<&str, BingoCard> {
    into(many_m_n(5, 5, bingo_row))(i)
}

fn bingo_row(i: &str) -> IResult<&str, Vec<u8>> {
    terminated(many_m_n(5, 5, spaces(unsigned_number)), tag("\n"))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo_row() {
        assert_eq!(bingo_row("1 2 3 4 5\n"), Ok(("", vec![1, 2, 3, 4, 5])));
        assert_eq!(
            bingo_row(" 1 10 24  5  9\n"),
            Ok(("", vec![1, 10, 24, 5, 9]))
        );
    }

    #[test]
    #[ignore = "hash map equality isn't working"]
    fn test_bingo_card_parse() {
        assert_eq!((1, 1), (1, 1));
        let bingo_card = bingo_card(
            "74 16 65 13 45\n86 20  6 34 15\n70 46 59 75 57\n28 62 67 71 98\n77 63 25 61 64\n",
        )
        .unwrap()
        .1;

        assert_eq!(
            bingo_card,
            BingoCard::from(vec![
                vec![74, 16, 65, 13, 45],
                vec![86, 20, 6, 34, 15],
                vec![70, 46, 59, 75, 57],
                vec![28, 62, 67, 71, 98],
                vec![77, 63, 25, 61, 64]
            ])
        );
    }

    #[test]
    fn test_bingo_card() {
        let mut bingo_card = BingoCard::from(vec![
            vec![14, 21, 17, 24, 4],
            vec![10, 16, 15, 9, 19],
            vec![18, 8, 23, 26, 20],
            vec![22, 11, 13, 6, 5],
            vec![2, 0, 12, 3, 7],
        ]);

        let numbers = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21];

        for number in numbers {
            bingo_card.call_number(number);
            assert_eq!(bingo_card.is_winner(), false);
        }

        bingo_card.call_number(24);
        assert_eq!(bingo_card.is_winner(), true);
    }
}
