use common::prelude::*;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list1,
    sequence::separated_pair, IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-2.txt");

    let strategy_guide = parse(input);

    let part_one = part_one(&strategy_guide);
    let part_two = part_two(&strategy_guide);

    Ok((part_one, part_two))
}

fn part_one(strategy_guide: &[Strategy]) -> PartAnswer {
    let start = SystemTime::now();

    let score: u32 = strategy_guide.iter().map(Strategy::score).sum();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(score, elapsed)
}

fn part_two(strategy_guide: &[Strategy]) -> PartAnswer {
    PartAnswer::default()
}

struct Strategy {
    them: char,
    me: char,
}

impl Strategy {
    fn score(&self) -> u32 {
        let shape_selected_score = match self.me {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => unreachable!(),
        };

        let outcome_score = match self.outcome() {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        };

        shape_selected_score + outcome_score
    }

    fn outcome(&self) -> Outcome {
        if self.them == 'A' && self.me == 'X'
            || self.them == 'B' && self.me == 'Y'
            || self.them == 'C' && self.me == 'Z'
        {
            Outcome::Draw
        } else {
            let did_i_win = match self.them {
                'A' => self.me == 'Y',
                'B' => self.me == 'Z',
                'C' => self.me == 'X',
                _ => unreachable!(),
            };

            if did_i_win {
                Outcome::Win
            } else {
                Outcome::Loss
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

fn parse(i: &str) -> Vec<Strategy> {
    finish(strategy_guide)(i).unwrap().1
}

fn strategy_guide(i: &str) -> IResult<&str, Vec<Strategy>> {
    separated_list1(tag("\n"), strategy)(i)
}

fn strategy(i: &str) -> IResult<&str, Strategy> {
    map(separated_pair(them, tag(" "), me), |(them, me)| Strategy {
        them,
        me,
    })(i)
}

fn them(i: &str) -> IResult<&str, char> {
    map(alt((tag("A"), tag("B"), tag("C"))), |s: &str| {
        s.chars().next().unwrap()
    })(i)
}

fn me(i: &str) -> IResult<&str, char> {
    map(alt((tag("X"), tag("Y"), tag("Z"))), |s: &str| {
        s.chars().next().unwrap()
    })(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_did_i_win() {
        let strategy = Strategy { them: 'A', me: 'Y' };
        assert_eq!(strategy.outcome(), Outcome::Win);

        let strategy = Strategy { them: 'B', me: 'X' };
        assert_eq!(strategy.outcome(), Outcome::Loss);

        let strategy = Strategy { them: 'C', me: 'Z' };
        assert_eq!(strategy.outcome(), Outcome::Draw);
    }

    #[test]
    fn test_score() {
        let strategy = Strategy { them: 'A', me: 'Y' };
        assert_eq!(strategy.score(), 8);

        let strategy = Strategy { them: 'B', me: 'X' };
        assert_eq!(strategy.score(), 1);

        let strategy = Strategy { them: 'C', me: 'Z' };
        assert_eq!(strategy.score(), 6);
    }
}
