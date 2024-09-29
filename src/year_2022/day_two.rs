use std::time::SystemTime;
use crate::common::answer::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{into, value},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use crate::common::parse::finish;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("input/day-2.txt");

    let strategy_guide = parse(input);

    let part_one = part_one(&strategy_guide);
    let part_two = part_two(&strategy_guide);

    Ok((part_one, part_two))
}

fn part_one(strategy_guide: &[Round]) -> PartAnswer {
    let start = SystemTime::now();

    let score: u32 = strategy_guide.iter().map(score_part_one).sum();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(score, elapsed)
}

fn part_two(strategy_guide: &[Round]) -> PartAnswer {
    let start = SystemTime::now();

    let score: u32 = strategy_guide.iter().map(score_part_two).sum();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(score, elapsed)
}

fn get_choice_for_outcome(them: &Choice, outcome: &Outcome) -> Choice {
    match outcome {
        Outcome::Loss => them.loss(),
        Outcome::Draw => them.draw(),
        Outcome::Win => them.win(),
    }
}

fn score_part_one(round: &Round) -> u32 {
    score(&round.them, &round.me)
}

fn score_part_two(round: &Round) -> u32 {
    score(
        &round.them,
        &get_choice_for_outcome(&round.them, &round.outcome),
    )
}

fn score(them: &Choice, me: &Choice) -> u32 {
    let outcome_score = if me.is_win(them) {
        6
    } else if me.is_draw(them) {
        3
    } else {
        0
    };

    let choice_score = match me {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    outcome_score + choice_score
}

struct Round {
    them: Choice,
    me: Choice,
    outcome: Outcome,
}

impl From<(Choice, (Choice, Outcome))> for Round {
    fn from(raw: (Choice, (Choice, Outcome))) -> Round {
        let (them, (me, outcome)) = raw;
        Round { them, me, outcome }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn is_win(&self, other: &Self) -> bool {
        self.loss() == *other
    }

    fn is_draw(&self, other: &Self) -> bool {
        self.draw() == *other
    }

    fn win(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    fn draw(&self) -> Choice {
        *self
    }

    fn loss(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }
}

fn parse(i: &str) -> Vec<Round> {
    finish(strategy_guide)(i).unwrap().1
}

fn strategy_guide(i: &str) -> IResult<&str, Vec<Round>> {
    separated_list1(tag("\n"), strategy)(i)
}

fn strategy(i: &str) -> IResult<&str, Round> {
    into(separated_pair(them, tag(" "), me))(i)
}

fn them(i: &str) -> IResult<&str, Choice> {
    alt((rock, paper, scissors))(i)
}

fn me(i: &str) -> IResult<&str, (Choice, Outcome)> {
    alt((
        value((Choice::Rock, Outcome::Loss), tag("X")),
        value((Choice::Paper, Outcome::Draw), tag("Y")),
        value((Choice::Scissors, Outcome::Win), tag("Z")),
    ))(i)
}

fn rock(i: &str) -> IResult<&str, Choice> {
    value(Choice::Rock, alt((tag("A"), tag("X"))))(i)
}

fn paper(i: &str) -> IResult<&str, Choice> {
    value(Choice::Paper, alt((tag("B"), tag("Y"))))(i)
}

fn scissors(i: &str) -> IResult<&str, Choice> {
    value(Choice::Scissors, alt((tag("C"), tag("Z"))))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_choice_for_outcome() {
        assert_eq!(
            get_choice_for_outcome(&Choice::Rock, &Outcome::Win),
            Choice::Paper
        );

        assert_eq!(
            get_choice_for_outcome(&Choice::Rock, &Outcome::Draw),
            Choice::Rock
        );

        assert_eq!(
            get_choice_for_outcome(&Choice::Rock, &Outcome::Loss),
            Choice::Scissors
        );

        assert_eq!(
            get_choice_for_outcome(&Choice::Paper, &Outcome::Win),
            Choice::Scissors
        );

        assert_eq!(
            get_choice_for_outcome(&Choice::Paper, &Outcome::Draw),
            Choice::Paper
        );

        assert_eq!(
            get_choice_for_outcome(&Choice::Paper, &Outcome::Loss),
            Choice::Rock
        );

        assert_eq!(
            get_choice_for_outcome(&Choice::Scissors, &Outcome::Win),
            Choice::Rock
        );

        assert_eq!(
            get_choice_for_outcome(&Choice::Scissors, &Outcome::Draw),
            Choice::Scissors
        );

        assert_eq!(
            get_choice_for_outcome(&Choice::Scissors, &Outcome::Loss),
            Choice::Paper
        );
    }

    #[test]
    fn test_is_win() {
        let choice = Choice::Scissors;
        assert_eq!(choice.is_win(&Choice::Paper), true);
        assert_eq!(choice.is_win(&Choice::Rock), false);
        assert_eq!(choice.is_win(&choice), false);
    }

    #[test]
    fn test_score_part_one() {}

    #[test]
    fn test_score_part_two() {
        let round = Round {
            them: Choice::Rock,
            me: Choice::Paper,
            outcome: Outcome::Draw,
        };
        assert_eq!(score_part_two(&round), 4);

        let round = Round {
            them: Choice::Paper,
            me: Choice::Rock,
            outcome: Outcome::Loss,
        };
        assert_eq!(score_part_two(&round), 1);
    }
}
