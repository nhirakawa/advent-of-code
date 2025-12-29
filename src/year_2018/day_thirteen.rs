use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, AddAssign},
    str::FromStr,
};

use anyhow::{anyhow, bail};
use itertools::Itertools;
use log::{debug, error};

use crate::common::{
    self,
    base::{Day, Year},
    debug,
};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut track = Track::from_str(input)?;

    debug::write(Year::Year2018, Day::Day13, "iter-0.txt", track.to_string())?;

    for i in 1..3000 {
        track.advance();

        debug::write(
            Year::Year2018,
            Day::Day13,
            format!("iter-{i}.txt").as_str(),
            track.to_string(),
        )?;

        // Log every 100 moves or so
        if i % 100 == 0 {
            debug!("=== After {} moves ===", i);
            for cart in &track.carts {
                debug!("Cart {}: {} (tick {})", cart.id, cart.position, cart.tick);
            }
        }

        if let Some((position, tick)) = track.find_collision() {
            debug!(
                "COLLISION DETECTED at move {} at position {} (tick {})",
                i, position, tick
            );
            return Ok(position);
        }
    }

    bail!("No solution found")
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let mut track = Track::from_str(input)?;

    for i in 1.. {
        track.advance();

        // Log every 100 moves or so
        if i % 100 == 0 {
            debug!("=== After {} moves ===", i);
            for cart in &track.carts {
                debug!("Cart {}: {} (tick {})", cart.id, cart.position, cart.tick);
            }
        }

        if let Some((position, tick)) = track.find_collision() {
            debug!(
                "COLLISION DETECTED at move {} at position {} (tick {})",
                i, position, tick
            );
            track.remove_carts(&position);
        }

        if track.carts.len() == 1 {
            return Ok(track.carts[0].position);
        }
    }

    bail!("No solution found")
}

#[derive(Debug, PartialEq, Eq)]
struct Track {
    segments: HashMap<Position, TrackSegment>,
    carts: Vec<Cart>,
}

impl Track {
    fn new<S: IntoIterator<Item = (Position, TrackSegment)>, C: IntoIterator<Item = Cart>>(
        segments: S,
        carts: C,
    ) -> Track {
        let segments = segments.into_iter().collect();
        let carts = carts.into_iter().collect();
        Track { segments, carts }
    }

    fn advance(&mut self) {
        if let Some(next_cart_to_move) = self.carts.iter_mut().sorted().next() {
            let old_pos = next_cart_to_move.position;
            next_cart_to_move.advance();
            debug!(
                "Cart {} moved from {} to {} (tick {})",
                next_cart_to_move.id, old_pos, next_cart_to_move.position, next_cart_to_move.tick
            );

            if let Some(segment) = self.segments.get(&next_cart_to_move.position) {
                next_cart_to_move.turn(&segment.shape);
            } else {
                error!("Cart {} is not on a segment", next_cart_to_move.id);
            }
        }
    }

    fn find_collision(&self) -> Option<(Position, usize)> {
        for outer in &self.carts {
            for inner in &self.carts {
                if outer.id == inner.id {
                    continue;
                }

                if outer.position == inner.position {
                    debug!(
                        "Collision found: Cart {} (tick {}) and Cart {} (tick {}) both at {}",
                        outer.id, outer.tick, inner.id, inner.tick, outer.position
                    );
                    return Some((outer.position, outer.tick));
                }
            }
        }
        None
    }

    fn remove_carts(&mut self, position: &Position) {
        self.carts = self
            .carts
            .iter()
            .filter(|cart| &cart.position != position)
            .copied()
            .collect();
    }
}

impl FromStr for Track {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = HashMap::new();
        let mut carts = Vec::new();

        let mut next_cart_id = 1;

        for ((x, y), c) in common::parse::griderator(s) {
            if c == ' ' {
                continue;
            } else if let Ok(cart) = Cart::try_from((next_cart_id, (x, y), c)) {
                carts.push(cart);
                let shape = match cart.direction {
                    CartDirection::Up | CartDirection::Down => TrackSegmentShape::Vertical,
                    CartDirection::Left | CartDirection::Right => TrackSegmentShape::Horizontal,
                };
                let segment = TrackSegment::from(((x, y), shape));
                segments.insert(segment.position, segment);
                next_cart_id += 1;
            } else if let Ok(segment) = TrackSegment::try_from(((x, y), c)) {
                segments.insert(segment.position, segment);
            } else {
                return Err(anyhow!("Found invalid value at {x},{y} - {c}"));
            }
        }

        Ok(Track::new(segments, carts))
    }
}

impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.segments.is_empty() {
            return Ok(());
        }

        let min_x = self.segments.keys().map(|p| p.x).min().unwrap_or(0);
        let max_x = self.segments.keys().map(|p| p.x).max().unwrap_or(0);
        let min_y = self.segments.keys().map(|p| p.y).min().unwrap_or(0);
        let max_y = self.segments.keys().map(|p| p.y).max().unwrap_or(0);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pos = Position { x, y };

                // Check if there's a cart at this position
                if let Some(cart) = self.carts.iter().find(|cart| cart.position == pos) {
                    let cart_char = match cart.direction {
                        CartDirection::Up => '↑',
                        CartDirection::Left => '←',
                        CartDirection::Down => '↓',
                        CartDirection::Right => '→',
                    };
                    write!(f, "{}", cart_char)?;
                } else if let Some(segment) = self.segments.get(&pos) {
                    let segment_char = match segment.shape {
                        TrackSegmentShape::Vertical => '│',
                        TrackSegmentShape::Horizontal => '─',
                        TrackSegmentShape::Intersection => '┼',
                        TrackSegmentShape::LeftToRightRising => '┘', // connects bottom-left to top-right
                        TrackSegmentShape::LeftToRightFalling => '┐', // connects top-left to bottom-right
                    };
                    write!(f, "{}", segment_char)?;
                } else {
                    write!(f, " ")?;
                }
            }
            if y < max_y {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl From<(isize, isize)> for Position {
    fn from((x, y): (isize, isize)) -> Self {
        Position { x, y }
    }
}

impl Add<CartDirection> for Position {
    type Output = Position;

    fn add(self, rhs: CartDirection) -> Self::Output {
        let x = self.x;
        let y = self.y;
        let next = match rhs {
            CartDirection::Up => (x, y - 1),
            CartDirection::Left => (x - 1, y),
            CartDirection::Down => (x, y + 1),
            CartDirection::Right => (x + 1, y),
        };
        next.into()
    }
}

impl AddAssign<CartDirection> for Position {
    fn add_assign(&mut self, rhs: CartDirection) {
        match rhs {
            CartDirection::Up => {
                self.y -= 1;
            }
            CartDirection::Left => {
                self.x -= 1;
            }
            CartDirection::Down => {
                self.y += 1;
            }
            CartDirection::Right => {
                self.x += 1;
            }
        };
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct TrackSegment {
    position: Position,
    shape: TrackSegmentShape,
}

impl<P: Into<Position>> From<(P, TrackSegmentShape)> for TrackSegment {
    fn from((position, shape): (P, TrackSegmentShape)) -> Self {
        let position = position.into();
        TrackSegment { position, shape }
    }
}

impl<P: Into<Position>> TryFrom<(P, char)> for TrackSegment {
    type Error = anyhow::Error;

    fn try_from((position, c): (P, char)) -> Result<Self, Self::Error> {
        let shape = TrackSegmentShape::try_from(c)?;
        Ok((position, shape).into())
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum TrackSegmentShape {
    Vertical,           // |
    Horizontal,         // -
    Intersection,       // +
    LeftToRightRising,  // /
    LeftToRightFalling, // \
}

impl TryFrom<char> for TrackSegmentShape {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(TrackSegmentShape::Vertical),
            '-' => Ok(TrackSegmentShape::Horizontal),
            '+' => Ok(TrackSegmentShape::Intersection),
            '/' => Ok(TrackSegmentShape::LeftToRightRising),
            '\\' => Ok(TrackSegmentShape::LeftToRightFalling),
            _ => Err(anyhow!("Invalid value - {value}")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Cart {
    id: usize,
    tick: usize,
    position: Position,
    direction: CartDirection,
    intersection_choice: IntersectionChoice,
}

impl Cart {
    fn advance(&mut self) {
        self.position += self.direction;
        self.tick += 1;
    }

    fn turn(&mut self, shape: &TrackSegmentShape) {
        match shape {
            TrackSegmentShape::Vertical => {}
            TrackSegmentShape::Horizontal => {}
            TrackSegmentShape::LeftToRightRising => {
                self.direction = match self.direction {
                    CartDirection::Up => CartDirection::Right,
                    CartDirection::Left => CartDirection::Down,
                    CartDirection::Down => CartDirection::Left,
                    CartDirection::Right => CartDirection::Up,
                }
            }
            TrackSegmentShape::LeftToRightFalling => {
                self.direction = match self.direction {
                    CartDirection::Up => CartDirection::Left,
                    CartDirection::Left => CartDirection::Up,
                    CartDirection::Down => CartDirection::Right,
                    CartDirection::Right => CartDirection::Down,
                }
            }
            TrackSegmentShape::Intersection => {
                self.direction = self.direction + self.intersection_choice;
                self.intersection_choice = self.intersection_choice.next();
            }
        };
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.tick
            .cmp(&other.tick)
            .then(self.position.cmp(&other.position))
    }
}

impl<P: Into<Position>> From<(usize, P, CartDirection)> for Cart {
    fn from((id, position, direction): (usize, P, CartDirection)) -> Self {
        let position = position.into();
        Cart {
            id,
            tick: 0,
            position,
            direction,
            intersection_choice: IntersectionChoice::Left,
        }
    }
}

impl<P: Into<Position>> TryFrom<(usize, P, char)> for Cart {
    type Error = anyhow::Error;

    fn try_from((id, position, c): (usize, P, char)) -> Result<Self, Self::Error> {
        let direction = CartDirection::try_from(c)?;
        Ok((id, position, direction).into())
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum CartDirection {
    Up,
    Left,
    Down,
    Right,
}

impl CartDirection {
    fn left(&self) -> CartDirection {
        match self {
            CartDirection::Up => CartDirection::Left,
            CartDirection::Left => CartDirection::Down,
            CartDirection::Down => CartDirection::Right,
            CartDirection::Right => CartDirection::Up,
        }
    }

    fn straight(&self) -> CartDirection {
        *self
    }

    fn right(&self) -> CartDirection {
        match self {
            CartDirection::Up => CartDirection::Right,
            CartDirection::Left => CartDirection::Up,
            CartDirection::Down => CartDirection::Left,
            CartDirection::Right => CartDirection::Down,
        }
    }
}

impl TryFrom<char> for CartDirection {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(CartDirection::Up),
            '<' => Ok(CartDirection::Left),
            'v' => Ok(CartDirection::Down),
            '>' => Ok(CartDirection::Right),
            _ => Err(anyhow!("Invalid value - {value}")),
        }
    }
}

impl Add<IntersectionChoice> for CartDirection {
    type Output = CartDirection;

    fn add(self, rhs: IntersectionChoice) -> Self::Output {
        match rhs {
            IntersectionChoice::Left => self.left(),
            IntersectionChoice::Stright => self.straight(),
            IntersectionChoice::Right => self.right(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
enum IntersectionChoice {
    #[default]
    Left,
    Stright,
    Right,
}

impl IntersectionChoice {
    fn next(&self) -> Self {
        match self {
            IntersectionChoice::Left => IntersectionChoice::Stright,
            IntersectionChoice::Stright => IntersectionChoice::Right,
            IntersectionChoice::Right => IntersectionChoice::Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_track_segment() {
        assert_eq!(
            TrackSegmentShape::try_from('|').unwrap(),
            TrackSegmentShape::Vertical
        );
        assert_eq!(
            TrackSegmentShape::try_from('-').unwrap(),
            TrackSegmentShape::Horizontal
        );
        assert_eq!(
            TrackSegmentShape::try_from('+').unwrap(),
            TrackSegmentShape::Intersection
        );
        assert_eq!(
            TrackSegmentShape::try_from('/').unwrap(),
            TrackSegmentShape::LeftToRightRising
        );
        assert_eq!(
            TrackSegmentShape::try_from('\\').unwrap(),
            TrackSegmentShape::LeftToRightFalling
        );

        assert!(TrackSegmentShape::try_from('x').is_err());
        assert!(TrackSegmentShape::try_from(' ').is_err());
    }

    #[test]
    fn test_parse_cart_direction() {
        assert_eq!(CartDirection::try_from('^').unwrap(), CartDirection::Up);
        assert_eq!(CartDirection::try_from('<').unwrap(), CartDirection::Left);
        assert_eq!(CartDirection::try_from('v').unwrap(), CartDirection::Down);
        assert_eq!(CartDirection::try_from('>').unwrap(), CartDirection::Right);

        assert!(CartDirection::try_from('x').is_err());
        assert!(CartDirection::try_from('|').is_err());
    }

    #[test]
    fn test_intersection_choice_default() {
        assert_eq!(IntersectionChoice::default(), IntersectionChoice::Left);
    }

    #[test]
    fn test_intersection_choice_next() {
        assert_eq!(IntersectionChoice::Left.next(), IntersectionChoice::Stright);
        assert_eq!(
            IntersectionChoice::Stright.next(),
            IntersectionChoice::Right
        );
        assert_eq!(IntersectionChoice::Right.next(), IntersectionChoice::Left);
    }

    #[test]
    fn test_intersection_choice_cycle() {
        let mut choice = IntersectionChoice::default();
        assert_eq!(choice, IntersectionChoice::Left);

        choice = choice.next();
        assert_eq!(choice, IntersectionChoice::Stright);

        choice = choice.next();
        assert_eq!(choice, IntersectionChoice::Right);

        choice = choice.next();
        assert_eq!(choice, IntersectionChoice::Left);
    }

    #[test]
    fn test_position_add_cart_direction() {
        let pos = Position { x: 5, y: 3 };

        assert_eq!(pos + CartDirection::Up, Position { x: 5, y: 2 });
        assert_eq!(pos + CartDirection::Left, Position { x: 4, y: 3 });
        assert_eq!(pos + CartDirection::Down, Position { x: 5, y: 4 });
        assert_eq!(pos + CartDirection::Right, Position { x: 6, y: 3 });
    }

    #[test]
    fn test_position_from_tuple() {
        let pos: Position = (10, 20).into();
        assert_eq!(pos, Position { x: 10, y: 20 });
    }

    #[test]
    fn test_cart_direction_turning() {
        // Test left turns
        assert_eq!(CartDirection::Up.left(), CartDirection::Left);
        assert_eq!(CartDirection::Left.left(), CartDirection::Down);
        assert_eq!(CartDirection::Down.left(), CartDirection::Right);
        assert_eq!(CartDirection::Right.left(), CartDirection::Up);

        // Test straight
        assert_eq!(CartDirection::Up.straight(), CartDirection::Up);
        assert_eq!(CartDirection::Left.straight(), CartDirection::Left);
        assert_eq!(CartDirection::Down.straight(), CartDirection::Down);
        assert_eq!(CartDirection::Right.straight(), CartDirection::Right);

        // Test right turns
        assert_eq!(CartDirection::Up.right(), CartDirection::Right);
        assert_eq!(CartDirection::Left.right(), CartDirection::Up);
        assert_eq!(CartDirection::Down.right(), CartDirection::Left);
        assert_eq!(CartDirection::Right.right(), CartDirection::Down);
    }

    #[test]
    fn test_cart_direction_add_intersection_choice() {
        let dir = CartDirection::Up;
        assert_eq!(dir + IntersectionChoice::Left, CartDirection::Left);
        assert_eq!(dir + IntersectionChoice::Stright, CartDirection::Up);
        assert_eq!(dir + IntersectionChoice::Right, CartDirection::Right);
    }

    #[test]
    fn test_cart_advance() {
        let mut cart = Cart::from((1, (5, 3), CartDirection::Up));
        assert_eq!(cart.position, Position { x: 5, y: 3 });
        assert_eq!(cart.tick, 0);

        cart.advance();
        assert_eq!(cart.position, Position { x: 5, y: 2 });
        assert_eq!(cart.tick, 1);

        cart.advance();
        assert_eq!(cart.position, Position { x: 5, y: 1 });
        assert_eq!(cart.tick, 2);
    }

    #[test]
    fn test_position_add_assign() {
        let mut pos = Position { x: 5, y: 3 };

        pos += CartDirection::Up;
        assert_eq!(pos, Position { x: 5, y: 2 });

        pos += CartDirection::Right;
        assert_eq!(pos, Position { x: 6, y: 2 });

        pos += CartDirection::Down;
        assert_eq!(pos, Position { x: 6, y: 3 });

        pos += CartDirection::Left;
        assert_eq!(pos, Position { x: 5, y: 3 });
    }

    #[test]
    fn test_position_display() {
        let pos = Position { x: 42, y: 13 };
        assert_eq!(format!("{}", pos), "42,13");
    }

    #[test]
    fn test_cart_turn_on_curves() {
        let mut cart = Cart::from((1, (0, 0), CartDirection::Up));

        // Test turning on LeftToRightRising curve (/)
        cart.turn(&TrackSegmentShape::LeftToRightRising);
        assert_eq!(cart.direction, CartDirection::Right);

        // Reset and test from different directions
        cart.direction = CartDirection::Left;
        cart.turn(&TrackSegmentShape::LeftToRightRising);
        assert_eq!(cart.direction, CartDirection::Down);

        cart.direction = CartDirection::Down;
        cart.turn(&TrackSegmentShape::LeftToRightRising);
        assert_eq!(cart.direction, CartDirection::Left);

        cart.direction = CartDirection::Right;
        cart.turn(&TrackSegmentShape::LeftToRightRising);
        assert_eq!(cart.direction, CartDirection::Up);
    }

    #[test]
    fn test_cart_turn_on_falling_curve() {
        let mut cart = Cart::from((1, (0, 0), CartDirection::Up));

        // Test turning on LeftToRightFalling curve (\)
        cart.turn(&TrackSegmentShape::LeftToRightFalling);
        assert_eq!(cart.direction, CartDirection::Left);

        cart.direction = CartDirection::Left;
        cart.turn(&TrackSegmentShape::LeftToRightFalling);
        assert_eq!(cart.direction, CartDirection::Up);

        cart.direction = CartDirection::Down;
        cart.turn(&TrackSegmentShape::LeftToRightFalling);
        assert_eq!(cart.direction, CartDirection::Right);

        cart.direction = CartDirection::Right;
        cart.turn(&TrackSegmentShape::LeftToRightFalling);
        assert_eq!(cart.direction, CartDirection::Down);
    }

    #[test]
    fn test_cart_turn_at_intersection() {
        let mut cart = Cart::from((1, (0, 0), CartDirection::Up));
        assert_eq!(cart.intersection_choice, IntersectionChoice::Left);

        // First intersection: turn left
        cart.turn(&TrackSegmentShape::Intersection);
        assert_eq!(cart.direction, CartDirection::Left);
        assert_eq!(cart.intersection_choice, IntersectionChoice::Stright);

        // Second intersection: go straight
        cart.turn(&TrackSegmentShape::Intersection);
        assert_eq!(cart.direction, CartDirection::Left);
        assert_eq!(cart.intersection_choice, IntersectionChoice::Right);

        // Third intersection: turn right
        cart.turn(&TrackSegmentShape::Intersection);
        assert_eq!(cart.direction, CartDirection::Up);
        assert_eq!(cart.intersection_choice, IntersectionChoice::Left);
    }

    #[test]
    fn test_cart_turn_straight_tracks() {
        let mut cart = Cart::from((1, (0, 0), CartDirection::Up));
        let original_direction = cart.direction;

        // Straight tracks shouldn't change direction
        cart.turn(&TrackSegmentShape::Vertical);
        assert_eq!(cart.direction, original_direction);

        cart.turn(&TrackSegmentShape::Horizontal);
        assert_eq!(cart.direction, original_direction);
    }

    #[test]
    fn test_track_parsing_simple() {
        let input = "|\nv\n|";
        let track = Track::from_str(input).unwrap();

        // Should have 3 segments and 1 cart
        assert_eq!(track.segments.len(), 3);
        assert_eq!(track.carts.len(), 1);

        // Check cart was parsed correctly
        let cart = &track.carts[0];
        assert_eq!(cart.position, Position { x: 0, y: 1 });
        assert_eq!(cart.direction, CartDirection::Down);

        // Check segments were created correctly
        assert!(track.segments.contains_key(&Position { x: 0, y: 0 }));
        assert!(track.segments.contains_key(&Position { x: 0, y: 1 }));
        assert!(track.segments.contains_key(&Position { x: 0, y: 2 }));
    }

    #[test]
    fn test_track_parsing_with_curves() {
        let input = r#"/\
\/"#;
        let track = Track::from_str(input).unwrap();

        assert_eq!(track.segments.len(), 4);
        assert_eq!(track.carts.len(), 0);

        let seg1 = track.segments.get(&Position { x: 0, y: 0 }).unwrap();
        assert_eq!(seg1.shape, TrackSegmentShape::LeftToRightRising);

        let seg2 = track.segments.get(&Position { x: 1, y: 0 }).unwrap();
        assert_eq!(seg2.shape, TrackSegmentShape::LeftToRightFalling);
    }

    #[test]
    fn test_track_find_collision() {
        let track = Track::new(
            vec![
                (
                    Position { x: 0, y: 0 },
                    TrackSegment::from((Position { x: 0, y: 0 }, TrackSegmentShape::Horizontal)),
                ),
                (
                    Position { x: 1, y: 0 },
                    TrackSegment::from((Position { x: 1, y: 0 }, TrackSegmentShape::Horizontal)),
                ),
            ],
            vec![
                Cart::from((1, Position { x: 1, y: 0 }, CartDirection::Right)),
                Cart::from((2, Position { x: 1, y: 0 }, CartDirection::Left)),
            ],
        );

        // Carts are at same position - should detect collision
        let collision = track.find_collision();
        assert_eq!(collision, Some((Position { x: 1, y: 0 }, 0)));
    }

    #[test]
    fn test_track_no_collision() {
        let track = Track::new(
            vec![
                (
                    Position { x: 0, y: 0 },
                    TrackSegment::from((Position { x: 0, y: 0 }, TrackSegmentShape::Horizontal)),
                ),
                (
                    Position { x: 2, y: 0 },
                    TrackSegment::from((Position { x: 2, y: 0 }, TrackSegmentShape::Horizontal)),
                ),
            ],
            vec![
                Cart::from((1, Position { x: 0, y: 0 }, CartDirection::Right)),
                Cart::from((2, Position { x: 2, y: 0 }, CartDirection::Left)),
            ],
        );

        // Carts are at different positions - no collision
        let collision = track.find_collision();
        assert_eq!(collision, None);
    }

    #[test]
    fn test_track_advance() {
        let mut track = Track::new(
            vec![
                (
                    Position { x: 0, y: 0 },
                    TrackSegment::from((Position { x: 0, y: 0 }, TrackSegmentShape::Horizontal)),
                ),
                (
                    Position { x: 1, y: 0 },
                    TrackSegment::from((
                        Position { x: 1, y: 0 },
                        TrackSegmentShape::LeftToRightRising,
                    )),
                ),
                (
                    Position { x: 1, y: 1 },
                    TrackSegment::from((Position { x: 1, y: 1 }, TrackSegmentShape::Vertical)),
                ),
            ],
            vec![Cart::from((
                1,
                Position { x: 0, y: 0 },
                CartDirection::Right,
            ))],
        );

        // Initial state
        assert_eq!(track.carts[0].position, Position { x: 0, y: 0 });
        assert_eq!(track.carts[0].direction, CartDirection::Right);
        assert_eq!(track.carts[0].tick, 0);

        // Advance once: move right to (1,0) and turn on LeftToRightRising curve
        track.advance();
        assert_eq!(track.carts[0].position, Position { x: 1, y: 0 });
        assert_eq!(track.carts[0].direction, CartDirection::Up); // Right + LeftToRightRising = Up
        assert_eq!(track.carts[0].tick, 1);

        // Advance again: move up to (1,-1) - but we need a segment there
        // Let's just verify the position changed correctly for up movement
        let old_y = track.carts[0].position.y;
        track.advance();
        assert_eq!(track.carts[0].position, Position { x: 1, y: old_y - 1 }); // Moved up
        assert_eq!(track.carts[0].tick, 2);
    }

    #[test]
    fn test_track_advance_at_intersection() {
        let mut track = Track::new(
            vec![
                (
                    Position { x: 0, y: 0 },
                    TrackSegment::from((Position { x: 0, y: 0 }, TrackSegmentShape::Horizontal)),
                ),
                (
                    Position { x: 1, y: 0 },
                    TrackSegment::from((Position { x: 1, y: 0 }, TrackSegmentShape::Intersection)),
                ),
                (
                    Position { x: 2, y: 0 },
                    TrackSegment::from((Position { x: 2, y: 0 }, TrackSegmentShape::Horizontal)),
                ),
            ],
            vec![Cart::from((
                1,
                Position { x: 0, y: 0 },
                CartDirection::Right,
            ))],
        );

        // Initial state
        assert_eq!(track.carts[0].intersection_choice, IntersectionChoice::Left);

        // Advance: move to intersection and turn left
        track.advance();
        assert_eq!(track.carts[0].position, Position { x: 1, y: 0 });
        assert_eq!(track.carts[0].direction, CartDirection::Up); // Right + Left = Up
        assert_eq!(
            track.carts[0].intersection_choice,
            IntersectionChoice::Stright
        );
    }

    #[test]
    fn test_track_remove_carts() {
        let mut track = Track::new(
            vec![
                (
                    Position { x: 0, y: 0 },
                    TrackSegment::from((Position { x: 0, y: 0 }, TrackSegmentShape::Horizontal)),
                ),
                (
                    Position { x: 1, y: 0 },
                    TrackSegment::from((Position { x: 1, y: 0 }, TrackSegmentShape::Horizontal)),
                ),
                (
                    Position { x: 2, y: 0 },
                    TrackSegment::from((Position { x: 2, y: 0 }, TrackSegmentShape::Horizontal)),
                ),
            ],
            vec![
                Cart::from((1, Position { x: 0, y: 0 }, CartDirection::Right)),
                Cart::from((2, Position { x: 1, y: 0 }, CartDirection::Left)),
                Cart::from((3, Position { x: 2, y: 0 }, CartDirection::Left)),
            ],
        );

        // Initially have 3 carts
        assert_eq!(track.carts.len(), 3);

        // Remove carts at position (1,0) - should remove cart 2
        track.remove_carts(&Position { x: 1, y: 0 });
        assert_eq!(track.carts.len(), 2);

        // Check remaining carts are the correct ones
        let remaining_ids: Vec<usize> = track.carts.iter().map(|c| c.id).collect();
        assert!(remaining_ids.contains(&1));
        assert!(!remaining_ids.contains(&2));
        assert!(remaining_ids.contains(&3));
    }

    #[test]
    fn test_track_remove_multiple_carts_at_position() {
        let mut track = Track::new(
            vec![
                (
                    Position { x: 0, y: 0 },
                    TrackSegment::from((Position { x: 0, y: 0 }, TrackSegmentShape::Horizontal)),
                ),
                (
                    Position { x: 1, y: 0 },
                    TrackSegment::from((Position { x: 1, y: 0 }, TrackSegmentShape::Horizontal)),
                ),
            ],
            vec![
                Cart::from((1, Position { x: 1, y: 0 }, CartDirection::Right)),
                Cart::from((2, Position { x: 1, y: 0 }, CartDirection::Left)),
                Cart::from((3, Position { x: 0, y: 0 }, CartDirection::Right)),
            ],
        );

        // Initially have 3 carts, 2 at same position
        assert_eq!(track.carts.len(), 3);

        // Remove carts at position (1,0) - should remove both carts 1 and 2
        track.remove_carts(&Position { x: 1, y: 0 });
        assert_eq!(track.carts.len(), 1);

        // Check only cart 3 remains
        assert_eq!(track.carts[0].id, 3);
        assert_eq!(track.carts[0].position, Position { x: 0, y: 0 });
    }

    #[test]
    fn test_track_remove_carts_no_match() {
        let mut track = Track::new(
            vec![
                (
                    Position { x: 0, y: 0 },
                    TrackSegment::from((Position { x: 0, y: 0 }, TrackSegmentShape::Horizontal)),
                ),
                (
                    Position { x: 1, y: 0 },
                    TrackSegment::from((Position { x: 1, y: 0 }, TrackSegmentShape::Horizontal)),
                ),
            ],
            vec![
                Cart::from((1, Position { x: 0, y: 0 }, CartDirection::Right)),
                Cart::from((2, Position { x: 1, y: 0 }, CartDirection::Left)),
            ],
        );

        // Initially have 2 carts
        assert_eq!(track.carts.len(), 2);

        // Try to remove carts at position where none exist
        track.remove_carts(&Position { x: 5, y: 5 });

        // Should still have 2 carts
        assert_eq!(track.carts.len(), 2);
    }
}
