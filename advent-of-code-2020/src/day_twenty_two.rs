use std::collections::HashSet;
use std::collections::VecDeque;

use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

pub fn run() -> AdventOfCodeResult<u64, u64> {
    let input = include_str!("../input/day-22.txt");
    let parse_start = SystemTime::now();
    let (player_one, player_two) = parse_decks(input);
    let parse_duration = parse_start.elapsed().unwrap();

    let part_one = part_one(&player_one, &player_two, parse_duration);
    let part_two = part_two(&player_one, &player_two, parse_duration);

    Ok((part_one, part_two))
}

fn part_one(player_one: &Deck, player_two: &Deck, parse_duration: Duration) -> PartAnswer<u64> {
    //println!("part one");
    let start = SystemTime::now();

    let mut player_one_deck = player_one.clone();
    let mut player_two_deck = player_two.clone();

    while player_one_deck.len() > 0 && player_two_deck.len() > 0 {
        let player_one_card = player_one_deck.pop_front().unwrap();
        let player_two_card = player_two_deck.pop_front().unwrap();

        if player_one_card > player_two_card {
            player_one_deck.push_back(player_one_card);
            player_one_deck.push_back(player_two_card);
        } else {
            player_two_deck.push_back(player_two_card);
            player_two_deck.push_back(player_one_card);
        }
    }

    let winning_deck = if player_one_deck.is_empty() {
        player_two_deck
    } else {
        player_one_deck
    };

    let score: u64 = winning_deck.score();

    let elapsed = start.elapsed().unwrap();

    (score, elapsed + parse_duration).into()
}

fn part_two(player_one: &Deck, player_two: &Deck, parse_duration: Duration) -> PartAnswer<u64> {
    //println!("part two");
    let start = SystemTime::now();
    let winning_deck = play_game(1, player_one, player_two);
    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(winning_deck.score(), elapsed + parse_duration)
}

fn play_game(game_id: usize, player_one: &Deck, player_two: &Deck) -> Deck {
    let mut previous_rounds = HashSet::new();

    let mut player_one_deck = player_one.clone();
    let mut player_two_deck = player_two.clone();

    while !&player_one_deck.is_empty() && !&player_two_deck.is_empty() {
        let state_id = player_one_deck.deck_string() + &player_two_deck.deck_string();

        let is_new_state = previous_rounds.insert(state_id);

        if !is_new_state {
            return player_one_deck;
        }

        let player_one_card = player_one_deck.pop_front().unwrap() as usize;
        let player_two_card = player_two_deck.pop_front().unwrap() as usize;

        if player_one_card <= player_one_deck.len() && player_two_card <= player_two_deck.len() {
            let player_one_subdeck: Vec<u64> = player_one_deck.clone().cards.into_iter().collect();
            let player_one_subdeck = player_one_subdeck[0..player_one_card]
                .into_iter()
                .map(|n| *n)
                .collect();
            let player_one_subdeck = Deck {
                player: Player::PlayerOne,
                cards: player_one_subdeck,
            };

            let player_two_subdeck: Vec<u64> = player_two_deck.clone().cards.into_iter().collect();
            let player_two_subdeck = player_two_subdeck[0..player_two_card]
                .into_iter()
                .map(|n| *n)
                .collect();
            let player_two_subdeck = Deck {
                player: Player::PlayerTwo,
                cards: player_two_subdeck,
            };

            let winning_deck = play_game(game_id + 1, &player_one_subdeck, &player_two_subdeck);

            match winning_deck.player {
                Player::PlayerOne => {
                    player_one_deck.push_back(player_one_card as u64);
                    player_one_deck.push_back(player_two_card as u64);
                }
                Player::PlayerTwo => {
                    player_two_deck.push_back(player_two_card as u64);
                    player_two_deck.push_back(player_one_card as u64);
                }
            }
        } else {
            if player_one_card > player_two_card {
                player_one_deck.push_back(player_one_card as u64);
                player_one_deck.push_back(player_two_card as u64);
            } else {
                player_two_deck.push_back(player_two_card as u64);
                player_two_deck.push_back(player_one_card as u64);
            }
        }
    }

    if player_one_deck.is_empty() {
        player_two_deck
    } else {
        player_one_deck
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
enum Player {
    PlayerOne,
    PlayerTwo,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Deck {
    player: Player,
    cards: VecDeque<u64>,
}

impl Deck {
    fn len(&self) -> usize {
        self.cards.len()
    }

    fn pop_front(&mut self) -> Option<u64> {
        self.cards.pop_front()
    }

    fn push_back(&mut self, card: u64) {
        self.cards.push_back(card)
    }

    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn score(&self) -> u64 {
        let length = self.len();

        let score = &self
            .cards
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, value)| value * (length as u64 - index as u64))
            .sum();

        *score
    }

    fn deck_string(&self) -> String {
        format!("{:?}", self.cards)
    }
}

fn parse_decks(input: &str) -> (Deck, Deck) {
    decks(input).map(|(_, decks)| decks).unwrap()
}

fn player(i: &str) -> IResult<&str, Player> {
    let player_one = value(Player::PlayerOne, tag("Player 1:\n"));
    let player_two = value(Player::PlayerTwo, tag("Player 2:\n"));

    alt((player_one, player_two))(i)
}

fn decks(i: &str) -> IResult<&str, (Deck, Deck)> {
    separated_pair(deck, tag("\n\n"), deck)(i)
}

fn deck(i: &str) -> IResult<&str, Deck> {
    let cards = map(separated_list1(tag("\n"), number), |v| {
        v.into_iter().collect()
    });
    map(tuple((player, cards)), |(player, cards)| Deck {
        player,
        cards,
    })(i)
}

fn number(i: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse())(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck() {
        let cards = vec![9, 4, 3].into_iter().collect();
        let player_deck = Deck {
            cards,
            player: Player::PlayerOne,
        };
        assert_eq!(deck("Player 1:\n9\n4\n3"), Ok(("", player_deck)));
    }
}
