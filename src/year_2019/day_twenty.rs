use crate::common::parse::griderator;
use anyhow::bail;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let maze = Maze::from_str(input)?;
    bfs(&maze, Recursion::None)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let maze = Maze::from_str(input)?;
    bfs(&maze, Recursion::Level(0))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Recursion {
    None,
    Level(usize),
}

impl Recursion {
    fn inner(&self) -> Self {
        match self {
            Recursion::None => Recursion::None,
            Recursion::Level(level) => Recursion::Level(level + 1),
        }
    }

    /// Returns `None` if already at the outermost level (0), since there is
    /// no level "outside" it to warp to.
    fn outer(&self) -> Option<Self> {
        match self {
            Recursion::None => Some(Recursion::None),
            Recursion::Level(0) => None,
            Recursion::Level(level) => Some(Recursion::Level(level - 1)),
        }
    }

    fn can_exit(&self) -> bool {
        match self {
            Recursion::None => true,
            Recursion::Level(0) => true,
            Recursion::Level(_) => false,
        }
    }
}

fn bfs(maze: &Maze, recursion: Recursion) -> anyhow::Result<usize> {
    let mut queue = VecDeque::new();
    queue.push_back((maze.start, 0, recursion));

    let mut seen = HashSet::new();

    while let Some((position, distance, recursion)) = queue.pop_front() {
        if position == maze.end && recursion.can_exit() {
            return Ok(distance);
        }

        if !seen.insert((position, recursion)) {
            continue;
        }

        for next in position.next() {
            if maze.spaces.contains(&next) {
                queue.push_back((next, distance + 1, recursion));
            }
        }

        if let Some(portal) = maze.portals.get(&position) {
            let (warp_to, exit_edge) = portal.warp(&position)?;

            // Since we know the edge of the position that we're warping too,
            // we invert the logic to match the edge of the position that we're warping from
            let next_recursion_state = match exit_edge {
                Edge::Inner => recursion.outer(),
                Edge::Outer => Some(recursion.inner()),
            };
            if let Some(next_recursion_state) = next_recursion_state {
                queue.push_back((warp_to, distance + 1, next_recursion_state));
            }
        }
    }

    bail!("No solution found")
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
enum Edge {
    Inner,
    Outer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Portal {
    name: PortalName,
    positions: [(Position, Edge); 2],
}

impl Portal {
    fn new(name: PortalName, positions: [(Position, Edge); 2]) -> Self {
        Self { name, positions }
    }

    fn warp(&self, position: &Position) -> anyhow::Result<(Position, Edge)> {
        if position == &self.positions[0].0 {
            Ok(self.positions[1])
        } else if position == &self.positions[1].0 {
            Ok(self.positions[0])
        } else {
            bail!(
                "Position {position:?} must be either {:?} or {:?}",
                self.positions[0].0,
                self.positions[1].0
            )
        }
    }

    #[cfg(test)]
    fn edge_at(&self, position: &Position) -> anyhow::Result<Edge> {
        if position == &self.positions[0].0 {
            Ok(self.positions[0].1)
        } else if position == &self.positions[1].0 {
            Ok(self.positions[1].1)
        } else {
            bail!(
                "Position {position:?} must be either {:?} or {:?}",
                self.positions[0].0,
                self.positions[1].0
            )
        }
    }
}

#[derive(Debug, Clone)]
struct Maze {
    start: Position,
    end: Position,
    spaces: HashSet<Position>,
    portals: HashMap<Position, Portal>,
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Find the spaces and the portal fragments
        let mut partial_portals: HashMap<Position, char> = HashMap::new();
        let mut spaces = HashSet::new();

        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;

        for (position, c) in griderator(s) {
            let position: Position = position.into();

            if c == ' ' || c == '#' {
                continue;
            } else if c == '.' {
                spaces.insert(position);

                min_x = min_x.min(position.0.0);
                max_x = max_x.max(position.0.0);
                min_y = min_y.min(position.0.1);
                max_y = max_y.max(position.0.1);
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
            let first_edge = if first.0.0 == min_x
                || first.0.0 == max_x
                || first.0.1 == min_y
                || first.0.1 == max_y
            {
                Edge::Outer
            } else {
                Edge::Inner
            };

            let second = positions[1];
            let second_edge = if second.0.0 == min_x
                || second.0.0 == max_x
                || second.0.1 == min_y
                || second.0.1 == max_y
            {
                Edge::Outer
            } else {
                Edge::Inner
            };

            if first_edge == second_edge {
                bail!("Portal edges are incompatible");
            }

            let portal = Portal::new(portal_name, [(first, first_edge), (second, second_edge)]);

            portals.insert(first, portal);
            portals.insert(second, portal);
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

    fn sample_maze() -> Maze {
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

        Maze::from_str(maze).unwrap()
    }

    #[test]
    fn test_maze_from_str() {
        let maze = sample_maze();

        assert_eq!(maze.start, Position((9, 2)));
        assert_eq!(maze.end, Position((13, 16)));
    }

    #[test]
    fn test_maze_from_str_edge_detection() {
        let maze = sample_maze();

        // BC: outer instance in the top-left, inner instance in the middle
        assert_eq!(
            maze.portals[&Position((2, 8))]
                .edge_at(&Position((2, 8)))
                .unwrap(),
            Edge::Outer,
            "BC at (2, 8) should be on the outer edge"
        );
        assert_eq!(
            maze.portals[&Position((9, 6))]
                .edge_at(&Position((9, 6)))
                .unwrap(),
            Edge::Inner,
            "BC at (9, 6) should be on an inner edge"
        );

        // DE: outer instance on the left, inner instance in the middle
        assert_eq!(
            maze.portals[&Position((2, 13))]
                .edge_at(&Position((2, 13)))
                .unwrap(),
            Edge::Outer,
            "DE at (2, 13) should be on the outer edge"
        );
        assert_eq!(
            maze.portals[&Position((6, 10))]
                .edge_at(&Position((6, 10)))
                .unwrap(),
            Edge::Inner,
            "DE at (6, 10) should be on an inner edge"
        );

        // FG: outer instance on the left, inner instance in the middle
        assert_eq!(
            maze.portals[&Position((2, 15))]
                .edge_at(&Position((2, 15)))
                .unwrap(),
            Edge::Outer,
            "FG at (2, 15) should be on the outer edge"
        );
        assert_eq!(
            maze.portals[&Position((11, 12))]
                .edge_at(&Position((11, 12)))
                .unwrap(),
            Edge::Inner,
            "FG at (11, 12) should be on an inner edge"
        );

        // AA/ZZ are the maze entrance/exit, not portals
        assert!(!maze.portals.contains_key(&maze.start));
        assert!(!maze.portals.contains_key(&maze.end));
    }
}
