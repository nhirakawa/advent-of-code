use common::prelude::*;
use log::{debug, trace};

pub fn run() -> AdventOfCodeResult {
    let part_one = part_one(9, 6);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(player_one_place: u32, player_two_place: u32) -> PartAnswer {
    let start = SystemTime::now();

    let mut player_one_score = 0;
    let mut player_two_score = 0;

    let mut player_one_place = player_one_place;
    let mut player_two_place = player_two_place;

    let mut turn = Turn::PlayerOne;

    let mut roll = 1;
    let mut die_rolls = 0;

    while player_one_score < 1000 && player_two_score < 1000 {
        let mut rolls = Vec::with_capacity(3);
        for _ in 0..3 {
            rolls.push(roll);
            roll = if roll == 100 { 1 } else { roll + 1 };
        }
        die_rolls += 3;
        let spaces_forward = rolls.iter().sum();
        match turn {
            Turn::PlayerOne => {
                player_one_place = get_next_space(player_one_place, spaces_forward);
                player_one_score += player_one_place;
                trace!(
                    "player 1 rolls {:?}, moves to {} for score {}",
                    rolls,
                    player_one_place,
                    player_one_score
                );
                turn = Turn::PlayerTwo;
            }
            Turn::PlayerTwo => {
                player_two_place = get_next_space(player_two_place, spaces_forward);
                player_two_score += player_two_place;
                trace!(
                    "player 2 rolls {:?}, moves to {} for score {}",
                    rolls,
                    player_two_place,
                    player_two_score
                );
                turn = Turn::PlayerOne;
            }
        }
    }

    debug!(
        "player 1: {}, player 2: {}",
        player_one_score, player_two_score
    );

    let solution = if player_one_score < 1000 {
        player_one_score
    } else {
        player_two_score
    };

    debug!("losing score: {}, die rolls: {}", solution, die_rolls);
    let solution = solution * die_rolls;

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn get_next_space(current_position: u32, spaces_forward: u32) -> u32 {
    let next_space = current_position + spaces_forward;

    if next_space <= 10 {
        next_space
    } else if next_space % 10 == 0 {
        10
    } else {
        u32::max(1, next_space % 10)
    }
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

enum Turn {
    PlayerOne,
    PlayerTwo,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_space() {
        // player 1
        assert_eq!(get_next_space(4, 1 + 2 + 3), 10);
        assert_eq!(get_next_space(10, 7 + 8 + 9), 4);
        assert_eq!(get_next_space(4, 13 + 14 + 15), 6);
        assert_eq!(get_next_space(6, 19 + 20 + 21), 6);
        assert_eq!(get_next_space(4, 91 + 92 + 93), 10);

        //player 2
        assert_eq!(get_next_space(8, 4 + 5 + 6), 3);
        assert_eq!(get_next_space(3, 10 + 11 + 12), 6);
        assert_eq!(get_next_space(6, 16 + 17 + 18), 7);
        assert_eq!(get_next_space(7, 22 + 23 + 24), 6);
        assert_eq!(get_next_space(6, 88 + 89 + 90), 3);
    }
}
