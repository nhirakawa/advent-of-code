use std::{collections::HashSet, iter};

use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-9.txt");

    let directions = parse(input);

    let part_one = part_one(&directions);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(directions: &[HeadMoveDirection]) -> PartAnswer {
    let start = SystemTime::now();

    let mut rope = Rope::new();

    let mut tail_positions = HashSet::new();

    for direction in directions {
        rope.move_rope(direction);
        tail_positions.insert(rope.tail);
    }

    let answer = tail_positions.len();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

fn part_two() -> PartAnswer {
    let start = SystemTime::now();
    let elapsed = start.elapsed().unwrap();
    PartAnswer::default()
}

struct Rope {
    head: (isize, isize),
    tail: (isize, isize),
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: (0, 0),
            tail: (0, 0),
        }
    }

    fn move_rope(&mut self, direction: &HeadMoveDirection) {
        self.head = direction.apply_to(&self.head);
        if !self.are_head_and_tail_touching() {
            let tail_move_direction = TailMoveDirection::of(&self.head, &self.tail);
            self.tail = tail_move_direction.apply_to(&self.tail);
        }
    }

    fn are_head_and_tail_touching(&self) -> bool {
        let (head_x, head_y) = self.head;
        let (tail_x, tail_y) = self.tail;

        let distance_x = head_x.abs_diff(tail_x);
        let distance_y = head_y.abs_diff(tail_y);

        if distance_x > 1 || distance_y > 1 {
            false
        } else {
            true
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum HeadMoveDirection {
    Left,
    Right,
    Up,
    Down,
}

impl HeadMoveDirection {
    fn apply_to(&self, head: &(isize, isize)) -> (isize, isize) {
        let (x, y) = *head;

        match self {
            HeadMoveDirection::Up => (x, y + 1),
            HeadMoveDirection::Right => (x + 1, y),
            HeadMoveDirection::Down => (x, y - 1),
            HeadMoveDirection::Left => (x - 1, y),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum TailMoveDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl TailMoveDirection {
    fn of(head: &(isize, isize), tail: &(isize, isize)) -> TailMoveDirection {
        let (head_x, head_y) = head;
        let (tail_x, tail_y) = tail;

        if head_x == tail_x {
            // tail moves up or down

            if head_y > tail_y {
                TailMoveDirection::Up
            } else {
                TailMoveDirection::Down
            }
        } else if head_y == tail_y {
            // tail moves left or right
            if head_x > tail_x {
                TailMoveDirection::Right
            } else {
                TailMoveDirection::Left
            }
        } else if head_x > tail_x {
            // tail moves to the right, either up or down
            if head_y > tail_y {
                TailMoveDirection::UpRight
            } else {
                TailMoveDirection::DownRight
            }
        } else {
            // head_x < tail_x
            // tail moves to the left, either up or down
            if head_y > tail_y {
                TailMoveDirection::UpLeft
            } else {
                TailMoveDirection::DownLeft
            }
        }
    }

    fn apply_to(&self, tail: &(isize, isize)) -> (isize, isize) {
        let (x, y) = *tail;

        match self {
            TailMoveDirection::Up => (x, y + 1),
            TailMoveDirection::UpRight => (x + 1, y + 1),
            TailMoveDirection::Right => (x + 1, y),
            TailMoveDirection::DownRight => (x + 1, y - 1),
            TailMoveDirection::Down => (x, y - 1),
            TailMoveDirection::DownLeft => (x - 1, y - 1),
            TailMoveDirection::Left => (x - 1, y),
            TailMoveDirection::UpLeft => (x - 1, y + 1),
        }
    }
}

fn parse(i: &str) -> Vec<HeadMoveDirection> {
    map(finish(all_directions), |vec_of_vecs| {
        vec_of_vecs.into_iter().flatten().collect()
    })(i)
    .unwrap()
    .1
}

fn all_directions(i: &str) -> IResult<&str, Vec<Vec<HeadMoveDirection>>> {
    separated_list1(tag("\n"), head_move_directions)(i)
}

fn head_move_directions(i: &str) -> IResult<&str, Vec<HeadMoveDirection>> {
    map(
        separated_pair(head_move_direction, tag(" "), unsigned_number),
        |(direction, count)| iter::repeat(direction).take(count).collect(),
    )(i)
}

fn head_move_direction(i: &str) -> IResult<&str, HeadMoveDirection> {
    alt((up, right, down, left))(i)
}

fn up(i: &str) -> IResult<&str, HeadMoveDirection> {
    value(HeadMoveDirection::Up, tag("U"))(i)
}

fn right(i: &str) -> IResult<&str, HeadMoveDirection> {
    value(HeadMoveDirection::Right, tag("R"))(i)
}

fn down(i: &str) -> IResult<&str, HeadMoveDirection> {
    value(HeadMoveDirection::Down, tag("D"))(i)
}

fn left(i: &str) -> IResult<&str, HeadMoveDirection> {
    value(HeadMoveDirection::Left, tag("L"))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_head_and_tail_touching() {
        let mut rope = Rope::new();
        assert_eq!(rope.are_head_and_tail_touching(), true);

        rope.head = (1, 0);
        assert_eq!(rope.are_head_and_tail_touching(), true);

        rope.head = (1, 1);
        assert_eq!(rope.are_head_and_tail_touching(), true);

        rope.head = (3, 4);
        rope.tail = (1, 2);
        assert_eq!(rope.are_head_and_tail_touching(), false);
    }

    #[test]
    fn test_tail_direction_of() {
        assert_eq!(
            TailMoveDirection::of(&(1, 2), &(0, 0)),
            TailMoveDirection::UpRight
        );
    }
}
