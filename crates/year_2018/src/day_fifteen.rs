use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use anyhow::{anyhow, bail};
use common::parse::griderator;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut grid = Grid::from_str(input)?;

    let mut count = 0;

    while grid.take_turns() {
        count += 1;
    }

    Ok(count * grid.total_health())
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position(i64, i64);

impl From<(i64, i64)> for Position {
    fn from((x, y): (i64, i64)) -> Self {
        Self(x, y)
    }
}

impl Position {
    /// Adjacent positions, in reading order.
    fn neighbors(self) -> [Position; 4] {
        let Self(x, y) = self;
        [
            Self(x, y - 1),
            Self(x - 1, y),
            Self(x + 1, y),
            Self(x, y + 1),
        ]
    }
}

/// Reading order: top-to-bottom, then left-to-right.
impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.1, self.0).cmp(&(other.1, other.0))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum UnitType {
    Elf,
    Goblin,
}

impl UnitType {
    fn enemy(self) -> Self {
        match self {
            Self::Elf => Self::Goblin,
            Self::Goblin => Self::Elf,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Unit {
    /// Starting position, which stays unique for the unit's lifetime
    id: Position,
    unit_type: UnitType,
    position: Position,
    attack: u32,
    health: u32,
}

impl Unit {
    fn new(unit_type: UnitType, position: Position) -> Self {
        Self {
            id: position,
            unit_type,
            position,
            attack: 3,
            health: 200,
        }
    }

    fn new_goblin(position: Position) -> Self {
        Self::new(UnitType::Goblin, position)
    }

    fn new_elf(position: Position) -> Self {
        Self::new(UnitType::Elf, position)
    }
}

struct Grid {
    spaces: HashSet<Position>,
    goblins: Vec<Unit>,
    elves: Vec<Unit>,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spaces = HashSet::new();
        let mut goblins = Vec::new();
        let mut elves = Vec::new();

        for (position, c) in griderator(s) {
            let position = position.into();

            match c {
                '.' => {
                    spaces.insert(position);
                }
                '#' => {
                    continue;
                }
                'G' => {
                    spaces.insert(position);
                    let goblin = Unit::new_goblin(position);
                    goblins.push(goblin);
                }
                'E' => {
                    spaces.insert(position);
                    let elf = Unit::new_elf(position);
                    elves.push(elf);
                }
                _ => {
                    bail!("Unrecognized character: '{c}'");
                }
            }
        }

        Ok(Grid {
            spaces,
            goblins,
            elves,
        })
    }
}

impl Grid {
    fn units(&self) -> impl Iterator<Item = &Unit> {
        self.goblins.iter().chain(self.elves.iter())
    }

    fn units_of(&self, unit_type: UnitType) -> &[Unit] {
        match unit_type {
            UnitType::Elf => &self.elves,
            UnitType::Goblin => &self.goblins,
        }
    }

    fn units_of_mut(&mut self, unit_type: UnitType) -> &mut Vec<Unit> {
        match unit_type {
            UnitType::Elf => &mut self.elves,
            UnitType::Goblin => &mut self.goblins,
        }
    }

    fn total_health(&self) -> u32 {
        self.units().map(|unit| unit.health).sum()
    }

    /// Runs one round. Returns `false` if combat ended before every unit took its turn.
    fn take_turns(&mut self) -> bool {
        let mut order: Vec<_> = self
            .units()
            .map(|unit| (unit.position, unit.unit_type, unit.id))
            .collect();
        order.sort_by_key(|&(position, _, _)| position);

        for (_, unit_type, id) in order {
            // Units killed earlier in the round don't get a turn
            let Some(index) = self
                .units_of(unit_type)
                .iter()
                .position(|unit| unit.id == id)
            else {
                continue;
            };

            if self.units_of(unit_type.enemy()).is_empty() {
                return false;
            }

            let mut unit = self.units_of(unit_type)[index];
            if let Some(step) = self.next_step(&unit) {
                unit.position = step;
                self.units_of_mut(unit_type)[index].position = step;
            }

            self.attack(unit);
        }

        true
    }

    /// Distance from `from` to every open position reachable from it.
    fn bfs(&self, from: Position, occupied: &HashSet<Position>) -> HashMap<Position, usize> {
        let mut distances = HashMap::from([(from, 0)]);
        let mut queue = VecDeque::from([from]);

        while let Some(position) = queue.pop_front() {
            let distance = distances[&position];

            for neighbor in position.neighbors() {
                if self.spaces.contains(&neighbor)
                    && !occupied.contains(&neighbor)
                    && !distances.contains_key(&neighbor)
                {
                    distances.insert(neighbor, distance + 1);
                    queue.push_back(neighbor);
                }
            }
        }

        distances
    }

    /// The position `unit` should move to this turn, or `None` if it shouldn't move.
    fn next_step(&self, unit: &Unit) -> Option<Position> {
        let enemies = self.units_of(unit.unit_type.enemy());

        let in_contact = unit
            .position
            .neighbors()
            .iter()
            .any(|&neighbor| enemies.iter().any(|enemy| enemy.position == neighbor));
        if in_contact {
            return None;
        }

        let occupied: HashSet<_> = self.units().map(|unit| unit.position).collect();

        // Nearest reachable square in range of an enemy, ties broken by reading order
        let from_unit = self.bfs(unit.position, &occupied);
        let (_, target) = enemies
            .iter()
            .flat_map(|enemy| enemy.position.neighbors())
            .filter_map(|position| from_unit.get(&position).map(|&d| (d, position)))
            .min()?;

        // First step along a shortest path to the target, ties broken by reading order
        let from_target = self.bfs(target, &occupied);
        unit.position
            .neighbors()
            .into_iter()
            .filter_map(|position| from_target.get(&position).map(|&d| (d, position)))
            .min()
            .map(|(_, position)| position)
    }

    /// Index of the enemy `unit` should attack, or `None` if no enemy is adjacent.
    fn attack_target(&self, unit: &Unit) -> Option<usize> {
        let adjacent = unit.position.neighbors();

        // Fewest hit points, ties broken by reading order
        self.units_of(unit.unit_type.enemy())
            .iter()
            .enumerate()
            .filter(|(_, enemy)| adjacent.contains(&enemy.position))
            .min_by_key(|(_, enemy)| (enemy.health, enemy.position))
            .map(|(index, _)| index)
    }

    /// `attacker` attacks its chosen target. Returns the target if the attack killed it.
    fn attack(&mut self, attacker: Unit) -> Option<Unit> {
        let index = self.attack_target(&attacker)?;
        let enemies = self.units_of_mut(attacker.unit_type.enemy());

        let target = &mut enemies[index];
        target.health = target.health.saturating_sub(attacker.attack);

        (target.health == 0).then(|| enemies.remove(index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Moves every unit once, in reading order, without attacking.
    fn move_round(grid: &mut Grid) {
        let mut order: Vec<_> = grid.units().map(|unit| unit.position).collect();
        order.sort();

        for position in order {
            let unit = grid.units().find(|unit| unit.position == position).unwrap();
            if let Some(step) = grid.next_step(unit) {
                grid.goblins
                    .iter_mut()
                    .chain(grid.elves.iter_mut())
                    .find(|unit| unit.position == position)
                    .unwrap()
                    .position = step;
            }
        }
    }

    fn positions(units: &[Unit]) -> Vec<Position> {
        let mut positions: Vec<_> = units.iter().map(|unit| unit.position).collect();
        positions.sort();
        positions
    }

    fn assert_units_match(grid: &Grid, expected: &str) {
        let expected: Grid = expected.parse().unwrap();
        assert_eq!(positions(&grid.goblins), positions(&expected.goblins));
        assert_eq!(positions(&grid.elves), positions(&expected.elves));
    }

    #[test]
    fn test_take_turns() {
        let cases = [
            (
                [
                    "#######", "#.G...#", "#...EG#", "#.#.#G#", "#..G#E#", "#.....#", "#######",
                ],
                27730,
            ),
            (
                [
                    "#######", "#G..#E#", "#E#E.E#", "#G.##.#", "#...#E#", "#...E.#", "#######",
                ],
                36334,
            ),
            (
                [
                    "#######", "#E..EG#", "#.#G.E#", "#E.##E#", "#G..#.#", "#..E#.#", "#######",
                ],
                39514,
            ),
            (
                [
                    "#######", "#E.G#.#", "#.#G..#", "#G.#.G#", "#G..#.#", "#...E.#", "#######",
                ],
                27755,
            ),
            (
                [
                    "#######", "#.E...#", "#.#..G#", "#.###.#", "#E#G#G#", "#...#G#", "#######",
                ],
                28944,
            ),
        ];

        for (input, outcome) in cases {
            let mut grid: Grid = input.join("\n").parse().unwrap();
            let mut rounds = 0;
            while grid.take_turns() {
                rounds += 1;
            }
            let health: u32 = grid.units().map(|unit| unit.health).sum();

            assert_eq!(rounds * health, outcome);
        }
    }

    #[test]
    fn test_attack() {
        let mut grid: Grid = "G....\n..G..\n..EG.\n..G..\n...G.".parse().unwrap();
        for (goblin, health) in grid.goblins.iter_mut().zip([9, 4, 2, 2, 1]) {
            goblin.health = health;
        }
        let elf = grid.elves[0];

        // Two adjacent goblins tie on 2 HP; the one first in reading order dies
        let killed = grid.attack(elf).unwrap();
        assert_eq!(killed.position, Position(3, 2));
        assert_units_match(&grid, "G....\n..G..\n..E..\n..G..\n...G.");

        // The remaining goblins at (2, 1) and (2, 3) are attacked in health order
        assert_eq!(
            grid.attack(elf).map(|unit| unit.position),
            Some(Position(2, 3))
        );
        assert_eq!(grid.attack(elf), None);
        assert_eq!(grid.goblins[1].health, 1);
    }

    #[test]
    fn test_next_step() {
        let grid: Grid = "#######\n#E..G.#\n#...#.#\n#.G.#G#\n#######"
            .parse()
            .unwrap();

        assert_eq!(grid.next_step(&grid.elves[0]), Some(Position(2, 1)));
    }

    #[test]
    fn test_movement() {
        let mut grid: Grid = [
            "#########",
            "#G..G..G#",
            "#.......#",
            "#.......#",
            "#G..E..G#",
            "#.......#",
            "#.......#",
            "#G..G..G#",
            "#########",
        ]
        .join("\n")
        .parse()
        .unwrap();

        let expected = [
            [
                "#########",
                "#.G...G.#",
                "#...G...#",
                "#...E..G#",
                "#.G.....#",
                "#.......#",
                "#G..G..G#",
                "#.......#",
                "#########",
            ],
            [
                "#########",
                "#..G.G..#",
                "#...G...#",
                "#.G.E.G.#",
                "#.......#",
                "#G..G..G#",
                "#.......#",
                "#.......#",
                "#########",
            ],
            [
                "#########",
                "#.......#",
                "#..GGG..#",
                "#..GEG..#",
                "#G..G...#",
                "#......G#",
                "#.......#",
                "#.......#",
                "#########",
            ],
        ];

        for expected in expected {
            move_round(&mut grid);
            assert_units_match(&grid, &expected.join("\n"));
        }
    }
}
