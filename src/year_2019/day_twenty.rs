use crate::common::parse::griderator;
use anyhow::{anyhow, bail};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub fn part_one(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position((isize, isize));

impl Position {
    fn next(&self) -> [Self; 4] {
        let (x, y) = self.0;
        [
            (x + 1, y).into(),
            (x - 1, y).into(),
            (x, y + 1).into(),
            (x, y - 1).into(),
        ]
    }

    fn up(&self) -> Self {
        let (x, y) = self.0;
        (x, y - 1).into()
    }

    fn left(&self) -> Self {
        let (x, y) = self.0;
        (x - 1, y).into()
    }
}

impl From<(isize, isize)> for Position {
    fn from(value: (isize, isize)) -> Self {
        Self(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct PortalName([char; 2]);

impl PortalName {
    fn new(a: char, b: char) -> Self {
        PortalName([a, b])
    }

    fn is_start(&self) -> bool {
        self.0[0] == 'A' && self.0[1] == 'A'
    }

    fn is_end(&self) -> bool {
        self.0[0] == 'Z' && self.0[1] == 'Z'
    }
}

impl Display for PortalName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Portal {
    name: PortalName,
    position: [Position; 2],
}

impl Portal {
    fn new(name: PortalName, position: [Position; 2]) -> Self {
        Self { name, position }
    }
}

#[derive(Debug, Clone)]
struct Maze {
    start: Position,
    end: Position,
    spaces: HashSet<Position>,
    portals: HashMap<Position, Position>,
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Find the spaces and the portal fragments
        let mut partial_portals: HashMap<Position, char> = HashMap::new();
        let mut spaces = HashSet::new();
        for (position, c) in griderator(s) {
            let position: Position = position.into();

            if c == ' ' || c == '#' {
                continue;
            } else if c == '.' {
                spaces.insert(position);
            } else if c.is_ascii_uppercase() {
                partial_portals.insert(position, c);
            }
        }

        // Combine the fragments
        let mut combined_portals: HashMap<PortalName, Vec<Position>> = HashMap::new();
        for (position, fragment) in &partial_portals {
            let above = partial_portals.get(&position.up());
            let left = partial_portals.get(&position.left());

            let (other_position, other_fragment) = match (above, left) {
                (Some(_), Some(_)) => {
                    bail!("Found portal fragment above AND left from {position:?}")
                }
                (None, None) => continue,
                (Some(other_fragment), None) => (position.up(), *other_fragment),
                (None, Some(other_fragment)) => (position.left(), *other_fragment),
            };

            // Portals are named top-to-bottom or left-to-right,
            // so prefer `other_fragment` since it is either above or to the left
            let portal_name = PortalName::new(other_fragment, *fragment);

            let portal_positions = [position.next(), other_position.next()]
                .into_iter()
                .flatten()
                .filter(|p| spaces.contains(p))
                .collect_vec();

            if portal_positions.len() != 1 {
                bail!("Expected exactly 1 space near {position:?}, found {portal_positions:?}");
            }

            combined_portals
                .entry(portal_name)
                .or_default()
                .push(portal_positions[0]);
        }

        // Combine both ends of each portal
        let mut portals = HashMap::new();
        let mut start = None;
        let mut end = None;
        for (portal_name, positions) in combined_portals {
            if portal_name.is_start() || portal_name.is_end() {
                if positions.len() != 1 {
                    bail!(
                        "Expected portal {portal_name} to contain exactly 1 position, found {positions:?}"
                    );
                }
                if portal_name.is_start() {
                    start = Some(positions[0]);
                }
                if portal_name.is_end() {
                    end = Some(positions[0]);
                }
                continue;
            }

            if positions.len() != 2 {
                bail!(
                    "Expected portal {portal_name} to contain exactly 2 positions, found {positions:?}"
                );
            }

            let first = positions[0];
            let second = positions[1];

            portals.insert(first, second);
            portals.insert(second, first);
        }

        let Some(start) = start else {
            bail!("No start found");
        };

        let Some(end) = end else {
            bail!("No end found");
        };

        Ok(Maze {
            start,
            end,
            spaces,
            portals,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_maze_from_str() {
        let maze = indoc!(
            "
                     A
                     A
              #######.#########
              #######.........#
              #######.#######.#
              #######.#######.#
              #######.#######.#
              #####  B    ###.#
            BC...##  C    ###.#
              ##.##       ###.#
              ##...DE  F  ###.#
              #####    G  ###.#
              #########.#####.#
            DE..#######...###.#
              #.#########.###.#
            FG..#########.....#
              ###########.#####
                         Z
                         Z
        "
        );

        let maze = Maze::from_str(maze).unwrap();

        assert_eq!(maze.start, Position((9, 2)));
        assert_eq!(maze.end, Position((13, 16)));
    }
}
