use crate::year_2021::day_twenty_one::game::{Player, Turn};
use anyhow::{anyhow, bail};
use itertools::Itertools;
use std::collections::HashMap;

// Calculated with external script
const DIRAC_ROLLS: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (player_one_position, player_two_position) = parse_starting_positions(input)?;

    let mut die = (1..=100).cycle();
    let mut roll_count = 0;
    let mut player_one = Player::new(player_one_position);
    let mut player_two = Player::new(player_two_position);
    let mut turn = Turn::PlayerOne;

    while player_one.score < 1000 && player_two.score < 1000 {
        let player = match &turn {
            Turn::PlayerOne => &mut player_one,
            Turn::PlayerTwo => &mut player_two,
        };

        let count = (&mut die).take(3).sum();
        roll_count += 3;
        player.advance(count);

        turn = turn.next();
    }

    let loser = if player_one.score > player_two.score {
        &player_two
    } else {
        &player_one
    };

    Ok(loser.score * roll_count)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let (player_one_position, player_two_position) = parse_starting_positions(input)?;

    let state = RecurseState::new(player_one_position, player_two_position);
    let mut cache = HashMap::new();

    let result = play_dirac_game(state, &mut cache);

    // 267086464416104 is too low
    Ok(usize::max(result.0, result.1))
}

fn play_dirac_game(
    state: RecurseState,
    cache: &mut HashMap<RecurseState, (usize, usize)>,
) -> (usize, usize) {
    if let Some(&result) = cache.get(&state) {
        return result;
    }

    // Base case - player 1 wins
    if state.player_one.score >= 21 {
        return (1, 0);
    }
    // Base case - player 2 wins
    if state.player_two.score >= 21 {
        return (0, 1);
    }

    let mut player_one_wins = 0;
    let mut player_two_wins = 0;

    // Someone needs to take a turn
    // sum represents the combined sum of the 3 die rolls
    // count represents how often that roll occurs
    for (sum, count) in DIRAC_ROLLS {
        let player_one = if state.turn == Turn::PlayerOne {
            let mut player = state.player_one.clone();
            player.advance(sum);
            player
        } else {
            state.player_one.clone()
        };
        let player_two = if state.turn == Turn::PlayerTwo {
            let mut player = state.player_two.clone();
            player.advance(sum);
            player
        } else {
            state.player_two.clone()
        };

        let next_state = RecurseState::from_states(player_one, player_two, state.turn.next());

        let (player_one_win, player_two_win) = play_dirac_game(next_state, cache);

        player_one_wins += player_one_win * count;
        player_two_wins += player_two_win * count;
    }

    let result = (player_one_wins, player_two_wins);
    cache.insert(state, result);
    result
}

#[derive(PartialEq, Eq, Hash)]
struct RecurseState {
    player_one: Player,
    player_two: Player,
    turn: Turn,
}

impl RecurseState {
    fn new(player_one_position: usize, player_two_position: usize) -> Self {
        let player_one = Player::new(player_one_position);
        let player_two = Player::new(player_two_position);
        Self {
            player_one,
            player_two,
            turn: Turn::PlayerOne,
        }
    }

    fn from_states(player_one: Player, player_two: Player, turn: Turn) -> Self {
        Self {
            player_one,
            player_two,
            turn,
        }
    }
}

mod game {
    #[derive(Clone, PartialEq, Eq, Hash)]
    struct Track(usize);

    impl Track {
        fn new(position: usize) -> Self {
            Self(position)
        }

        fn position(&mut self) -> usize {
            self.0
        }

        fn advance(&mut self, count: usize) {
            for _ in 0..count {
                self.0 += 1;
                if self.0 == 11 {
                    self.0 = 1;
                }
            }
        }
    }

    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Player {
        track: Track,
        pub score: usize,
    }

    impl Player {
        pub fn new(position: usize) -> Self {
            let track = Track::new(position);
            Self { track, score: 0 }
        }

        pub fn advance(&mut self, count: usize) {
            self.track.advance(count);
            self.score += self.track.position();
        }
    }

    #[derive(PartialEq, Eq, Hash)]
    pub enum Turn {
        PlayerOne,
        PlayerTwo,
    }

    impl Turn {
        pub fn next(&self) -> Self {
            match self {
                Turn::PlayerOne => Turn::PlayerTwo,
                Turn::PlayerTwo => Turn::PlayerOne,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn track_starts_at_given_position() {
            let mut track = Track::new(4);
            assert_eq!(track.position(), 4);
        }

        #[test]
        fn track_advance_moves_forward() {
            let mut track = Track::new(4);
            track.advance(3);
            assert_eq!(track.position(), 7);
        }

        #[test]
        fn track_advance_wraps_around_ten() {
            let mut track = Track::new(9);
            track.advance(5);
            assert_eq!(track.position(), 4);
        }

        #[test]
        fn track_advance_wraps_exactly_at_boundary() {
            let mut track = Track::new(1);
            track.advance(9);
            assert_eq!(track.position(), 10);
            track.advance(1);
            assert_eq!(track.position(), 1);
        }
    }
}

fn parse_starting_positions(s: &str) -> anyhow::Result<(usize, usize)> {
    let lines = s.trim().lines().collect_vec();

    if lines.len() != 2 {
        bail!("Expected two lines, got {}", lines.len());
    }

    #[allow(clippy::get_first)]
    let player_one_line = lines.get(0).expect("could not get first line");
    let player_two_line = lines.get(1).expect("could not get second line");

    let player_one_position = player_one_line
        .strip_prefix("Player 1 starting position: ")
        .ok_or(anyhow!("Could not strip prefix in player_one_line"))?;

    let player_two_position = player_two_line
        .strip_prefix("Player 2 starting position: ")
        .ok_or(anyhow!("Could not strip prefix in player_two_line"))?;

    let player_one_position = player_one_position.parse::<usize>()?;
    let player_two_position = player_two_position.parse::<usize>()?;

    Ok((player_one_position, player_two_position))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_starting_positions() {
        let input = "Player 1 starting position: 4\nPlayer 2 starting position: 8";
        let result = parse_starting_positions(input).unwrap();
        assert_eq!(result, (4, 8));
    }
}
