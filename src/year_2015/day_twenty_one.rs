#![allow(clippy::type_complexity)]

use anyhow::bail;
use model::*;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let boss = input.parse::<Boss>()?;

    let mut min_cost = usize::MAX;

    for (weapon, armor, rings) in all_item_combinations() {
        let player = create_player(weapon, armor, rings);
        let winner = fight(player, boss);

        if winner == Winner::Player {
            let cost = weapon.cost()
                + armor.map_or(0, |armor| armor.cost())
                + rings.0.map_or(0, |ring| ring.cost())
                + rings.1.map_or(0, |ring| ring.cost());

            if cost < min_cost {
                min_cost = cost;
            }
        }
    }

    if min_cost != usize::MAX {
        return Ok(min_cost);
    }

    bail!("No winning combination found")
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let boss = input.parse::<Boss>()?;

    let mut max_cost = 0;

    for (weapon, armor, rings) in all_item_combinations() {
        let player = create_player(weapon, armor, rings);
        let winner = fight(player, boss);

        if winner == Winner::Boss {
            let cost = weapon.cost()
                + armor.map_or(0, |armor| armor.cost())
                + rings.0.map_or(0, |ring| ring.cost())
                + rings.1.map_or(0, |ring| ring.cost());

            if cost > max_cost {
                max_cost = cost;
            }
        }
    }

    if max_cost != 0 {
        return Ok(max_cost);
    }

    bail!("Not implemented")
}

fn all_item_combinations() -> Vec<(Weapon, Option<Armor>, (Option<Ring>, Option<Ring>))> {
    let weapons = Weapon::values();
    let armors = Armor::values();
    let rings = Ring::values();

    let mut combinations = Vec::new();

    for weapon in weapons {
        for armor in armors
            .iter()
            .copied()
            .map(Some)
            .chain(std::iter::once(None))
        {
            for ring_1 in rings.iter().copied().map(Some).chain(std::iter::once(None)) {
                for ring_2 in rings.iter().copied().map(Some).chain(std::iter::once(None)) {
                    if ring_1.is_some() && ring_2.is_some() && ring_1 == ring_2 {
                        continue;
                    }

                    combinations.push((weapon, armor, (ring_1, ring_2)));
                }
            }
        }
    }

    combinations
}

fn create_player(
    weapon: Weapon,
    armor: Option<Armor>,
    rings: (Option<Ring>, Option<Ring>),
) -> Player {
    let damage = weapon.damage()
        + rings.0.map_or(0, |ring| ring.damage())
        + rings.1.map_or(0, |ring| ring.damage());

    let armor = armor.map_or(0, |armor| armor.armor())
        + rings.0.map_or(0, |ring| ring.armor())
        + rings.1.map_or(0, |ring| ring.armor());

    Player {
        hit_points: 100,
        damage: damage as i32,
        armor: armor as i32,
    }
}

fn fight(player: Player, boss: Boss) -> Winner {
    let mut player = player;
    let mut boss = boss;

    while player.hit_points > 0 && boss.hit_points > 0 {
        let player_damage = (player.damage - boss.armor).max(1);
        let boss_damage = (boss.damage - player.armor).max(1);

        boss.hit_points -= player_damage;
        if boss.hit_points <= 0 {
            return Winner::Player;
        }

        player.hit_points -= boss_damage;
        if player.hit_points <= 0 {
            return Winner::Boss;
        }
    }

    unreachable!()
}

mod model {
    use std::str::FromStr;

    use anyhow::anyhow;
    use anyhow::bail;

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum Winner {
        Player,
        Boss,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Player {
        pub hit_points: i32,
        pub damage: i32,
        pub armor: i32,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum Weapon {
        Dagger,
        Shortsword,
        Warhammer,
        Longsword,
        Greataxe,
    }

    impl Weapon {
        pub fn values() -> Vec<Weapon> {
            vec![
                Weapon::Dagger,
                Weapon::Shortsword,
                Weapon::Warhammer,
                Weapon::Longsword,
                Weapon::Greataxe,
            ]
        }

        pub fn cost(&self) -> usize {
            match self {
                Weapon::Dagger => 8,
                Weapon::Shortsword => 10,
                Weapon::Warhammer => 25,
                Weapon::Longsword => 40,
                Weapon::Greataxe => 74,
            }
        }

        pub fn damage(&self) -> usize {
            match self {
                Weapon::Dagger => 4,
                Weapon::Shortsword => 5,
                Weapon::Warhammer => 6,
                Weapon::Longsword => 7,
                Weapon::Greataxe => 8,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum Armor {
        Leather,
        Chainmail,
        Splintmail,
        Bandedmail,
        Platemail,
    }

    impl Armor {
        pub fn values() -> Vec<Armor> {
            vec![
                Armor::Leather,
                Armor::Chainmail,
                Armor::Splintmail,
                Armor::Bandedmail,
                Armor::Platemail,
            ]
        }

        pub fn cost(&self) -> usize {
            match self {
                Armor::Leather => 13,
                Armor::Chainmail => 31,
                Armor::Splintmail => 53,
                Armor::Bandedmail => 75,
                Armor::Platemail => 102,
            }
        }

        pub fn armor(&self) -> usize {
            match self {
                Armor::Leather => 1,
                Armor::Chainmail => 2,
                Armor::Splintmail => 3,
                Armor::Bandedmail => 4,
                Armor::Platemail => 5,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum Ring {
        DamageAdd1,
        DamageAdd2,
        DamageAdd3,
        DefenseAdd1,
        DefenseAdd2,
        DefenseAdd3,
    }

    impl Ring {
        pub fn values() -> Vec<Ring> {
            vec![
                Ring::DamageAdd1,
                Ring::DamageAdd2,
                Ring::DamageAdd3,
                Ring::DefenseAdd1,
                Ring::DefenseAdd2,
                Ring::DefenseAdd3,
            ]
        }

        pub fn cost(&self) -> usize {
            match self {
                Ring::DamageAdd1 => 25,
                Ring::DamageAdd2 => 50,
                Ring::DamageAdd3 => 100,
                Ring::DefenseAdd1 => 20,
                Ring::DefenseAdd2 => 40,
                Ring::DefenseAdd3 => 80,
            }
        }

        pub fn damage(&self) -> usize {
            match self {
                Ring::DamageAdd1 => 1,
                Ring::DamageAdd2 => 2,
                Ring::DamageAdd3 => 3,
                Ring::DefenseAdd1 => 0,
                Ring::DefenseAdd2 => 0,
                Ring::DefenseAdd3 => 0,
            }
        }

        pub fn armor(&self) -> usize {
            match self {
                Ring::DamageAdd1 => 0,
                Ring::DamageAdd2 => 0,
                Ring::DamageAdd3 => 0,
                Ring::DefenseAdd1 => 1,
                Ring::DefenseAdd2 => 2,
                Ring::DefenseAdd3 => 3,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Boss {
        pub hit_points: i32,
        pub damage: i32,
        pub armor: i32,
    }

    impl FromStr for Boss {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s = s.trim();

            let lines = s.lines().collect::<Vec<_>>();

            if lines.len() != 3 {
                bail!("Invalid input - incorrect length");
            }

            let hit_points = {
                let line = lines[0];
                if !line.starts_with("Hit Points: ") {
                    bail!("Invalid input - missing Hit Points");
                }

                line.split_once("Hit Points: ")
                    .ok_or(anyhow!("Invalid input - could not extract hit points"))?
                    .1
                    .parse::<i32>()?
            };

            let damage = {
                let line = lines[1];
                if !line.starts_with("Damage: ") {
                    bail!("Invalid input - missing Damage");
                }

                line.split_once("Damage: ")
                    .ok_or(anyhow!("Invalid input - could not extract damage"))?
                    .1
                    .parse::<i32>()?
            };

            let armor = {
                let line = lines[2];
                if !line.starts_with("Armor: ") {
                    bail!("Invalid input - missing Armor");
                }

                line.split_once("Armor: ")
                    .ok_or(anyhow!("Invalid input - could not extract armor"))?
                    .1
                    .parse::<i32>()?
            };

            Ok(Boss {
                hit_points,
                damage,
                armor,
            })
        }
    }
}
