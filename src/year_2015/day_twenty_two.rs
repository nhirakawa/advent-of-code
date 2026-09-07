use anyhow::{anyhow, bail};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let boss = Boss::from_str(input)?;

    let mut seen: HashSet<SeenState> = HashSet::new();

    let mut heap = BinaryHeap::new();
    let initial_game_state = GameState::init(Player::default(), boss, Mode::Easy);
    heap.push(Reverse(initial_game_state));

    let mut minimum_spent_mana = usize::MAX;

    while let Some(Reverse(state)) = heap.pop() {
        if state.spent_mana >= minimum_spent_mana {
            continue;
        }

        if !seen.insert(state.clone().into()) {
            continue;
        }

        match state.turn {
            Turn::Player => {
                for spell in Spell::values() {
                    let state = state.clone();
                    let player_turn_result = state.do_player_turn(spell);
                    match player_turn_result {
                        PlayerTurnResult::PlayerWin(spent_mana) => {
                            minimum_spent_mana = usize::min(minimum_spent_mana, spent_mana);
                        }
                        PlayerTurnResult::Continue(next_state) => {
                            heap.push(Reverse(next_state));
                        }
                        PlayerTurnResult::InvalidSpellCast | PlayerTurnResult::BossWin => {
                            continue;
                        }
                    }
                }
            }
            Turn::Boss => {
                let boss_turn_result = state.do_boss_turn();
                match boss_turn_result {
                    BossTurnResult::PlayerWin(spent_mana) => {
                        minimum_spent_mana = usize::min(minimum_spent_mana, spent_mana);
                    }
                    BossTurnResult::BossWin => {
                        continue;
                    }
                    BossTurnResult::Continue(next_state) => {
                        heap.push(Reverse(next_state));
                    }
                }
            }
        }
    }

    if minimum_spent_mana == usize::MAX {
        bail!("no solution found");
    }

    Ok(minimum_spent_mana)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let boss = Boss::from_str(input)?;

    let mut seen: HashSet<SeenState> = HashSet::new();

    let mut heap = BinaryHeap::new();
    let initial_game_state = GameState::init(Player::default(), boss, Mode::Hard);
    heap.push(Reverse(initial_game_state));

    let mut minimum_spent_mana = usize::MAX;

    while let Some(Reverse(state)) = heap.pop() {
        if state.spent_mana >= minimum_spent_mana {
            continue;
        }

        if !seen.insert(state.clone().into()) {
            continue;
        }

        match state.turn {
            Turn::Player => {
                for spell in Spell::values() {
                    let state = state.clone();
                    let player_turn_result = state.do_player_turn(spell);
                    match player_turn_result {
                        PlayerTurnResult::PlayerWin(spent_mana) => {
                            minimum_spent_mana = usize::min(minimum_spent_mana, spent_mana);
                        }
                        PlayerTurnResult::Continue(next_state) => {
                            heap.push(Reverse(next_state));
                        }
                        PlayerTurnResult::InvalidSpellCast | PlayerTurnResult::BossWin => {
                            continue;
                        }
                    }
                }
            }
            Turn::Boss => {
                let boss_turn_result = state.do_boss_turn();
                match boss_turn_result {
                    BossTurnResult::PlayerWin(spent_mana) => {
                        minimum_spent_mana = usize::min(minimum_spent_mana, spent_mana);
                    }
                    BossTurnResult::BossWin => {
                        continue;
                    }
                    BossTurnResult::Continue(next_state) => {
                        heap.push(Reverse(next_state));
                    }
                }
            }
        }
    }

    if minimum_spent_mana == usize::MAX {
        bail!("no solution found");
    }

    Ok(minimum_spent_mana)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct SeenState {
    player: Player,
    boss: Boss,
    turn: Turn,
    shield_turns_remaining: usize,
    poison_turns_remaining: usize,
    recharge_turns_remaining: usize,
}

impl From<GameState> for SeenState {
    fn from(state: GameState) -> Self {
        Self {
            player: state.player,
            boss: state.boss,
            turn: state.turn,
            shield_turns_remaining: state.shield_turns_remaining,
            poison_turns_remaining: state.poison_turns_remaining,
            recharge_turns_remaining: state.recharge_turns_remaining,
        }
    }
}

enum PlayerTurnResult {
    PlayerWin(usize),
    BossWin,
    Continue(GameState),
    InvalidSpellCast,
}

enum BossTurnResult {
    PlayerWin(usize),
    BossWin,
    Continue(GameState),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GameState {
    player: Player,
    boss: Boss,
    mode: Mode,
    turn: Turn,
    shield_turns_remaining: usize,
    poison_turns_remaining: usize,
    recharge_turns_remaining: usize,
    spent_mana: usize,
}

impl GameState {
    fn init(player: Player, boss: Boss, mode: Mode) -> Self {
        Self {
            player,
            boss,
            mode,
            turn: Turn::Player,
            shield_turns_remaining: 0,
            poison_turns_remaining: 0,
            recharge_turns_remaining: 0,
            spent_mana: 0,
        }
    }

    fn do_player_turn(self, spell: Spell) -> PlayerTurnResult {
        let GameState {
            mut player,
            mut boss,
            mode,
            turn: _turn,
            mut shield_turns_remaining,
            mut poison_turns_remaining,
            mut recharge_turns_remaining,
            mut spent_mana,
        } = self;

        if mode == Mode::Hard {
            if player.hit_points <= 1 {
                return PlayerTurnResult::BossWin;
            }
            player.hit_points -= 1;
        }

        if poison_turns_remaining > 0 {
            poison_turns_remaining -= 1;
            if boss.hit_points <= 3 {
                return PlayerTurnResult::PlayerWin(spent_mana);
            }

            boss.hit_points -= 3;
        }
        if shield_turns_remaining > 0 {
            shield_turns_remaining -= 1;
        }
        player.armor = if shield_turns_remaining > 0 { 7 } else { 0 };
        if recharge_turns_remaining > 0 {
            recharge_turns_remaining -= 1;
            player.mana += 101;
        }

        let cost = spell.cost();

        if cost > player.mana {
            return PlayerTurnResult::InvalidSpellCast;
        }
        if spell == Spell::Poison && poison_turns_remaining > 0 {
            return PlayerTurnResult::InvalidSpellCast;
        }
        if spell == Spell::Shield && shield_turns_remaining > 0 {
            return PlayerTurnResult::InvalidSpellCast;
        }
        if spell == Spell::Recharge && recharge_turns_remaining > 0 {
            return PlayerTurnResult::InvalidSpellCast;
        }

        spent_mana += cost;
        player.mana -= cost;

        match spell {
            Spell::MagicMissile => {
                if boss.hit_points <= 4 {
                    return PlayerTurnResult::PlayerWin(spent_mana);
                }
                boss.hit_points -= 4;
            }
            Spell::Drain => {
                if boss.hit_points <= 2 {
                    return PlayerTurnResult::PlayerWin(spent_mana);
                }
                boss.hit_points -= 2;
                player.hit_points += 2;
            }
            Spell::Shield => {
                shield_turns_remaining = 6;
            }
            Spell::Poison => {
                poison_turns_remaining = 6;
            }
            Spell::Recharge => {
                recharge_turns_remaining = 5;
            }
        }

        PlayerTurnResult::Continue(GameState {
            player,
            boss,
            mode,
            turn: Turn::Boss,
            shield_turns_remaining,
            poison_turns_remaining,
            recharge_turns_remaining,
            spent_mana,
        })
    }

    fn do_boss_turn(self) -> BossTurnResult {
        let GameState {
            mut player,
            mut boss,
            mode,
            turn: _turn,
            mut shield_turns_remaining,
            mut poison_turns_remaining,
            mut recharge_turns_remaining,
            spent_mana,
        } = self;

        if poison_turns_remaining > 0 {
            poison_turns_remaining -= 1;
            if boss.hit_points <= 3 {
                return BossTurnResult::PlayerWin(spent_mana);
            }

            boss.hit_points -= 3;
        }
        if shield_turns_remaining > 0 {
            shield_turns_remaining -= 1;
        }
        player.armor = if shield_turns_remaining > 0 { 7 } else { 0 };
        if recharge_turns_remaining > 0 {
            recharge_turns_remaining -= 1;
            player.mana += 101;
        }

        let damage = if player.armor >= boss.damage {
            1
        } else {
            boss.damage - player.armor
        };

        if damage >= player.hit_points {
            return BossTurnResult::BossWin;
        }

        player.hit_points -= damage;

        BossTurnResult::Continue(GameState {
            player,
            boss,
            mode,
            turn: Turn::Player,
            shield_turns_remaining,
            poison_turns_remaining,
            recharge_turns_remaining,
            spent_mana,
        })
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.spent_mana.cmp(&other.spent_mana)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Mode {
    Easy,
    Hard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Turn {
    Player,
    Boss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    hit_points: usize,
    armor: usize,
    mana: usize,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            hit_points: 50,
            armor: 0,
            mana: 500,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Boss {
    hit_points: usize,
    damage: usize,
}

impl Boss {
    fn new(hit_points: usize, damage: usize) -> Self {
        Self { hit_points, damage }
    }
}

impl FromStr for Boss {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s
            .trim()
            .split_once('\n')
            .ok_or(anyhow!("Could not split on newline"))?;

        let hit_points = first
            .strip_prefix("Hit Points: ")
            .ok_or(anyhow!("No 'Hit Points: ' prefix"))
            .and_then(|s| s.parse().map_err(anyhow::Error::from))?;

        let damage = second
            .strip_prefix("Damage: ")
            .ok_or(anyhow!("No 'Damage: ' prefix"))
            .and_then(|s| s.parse().map_err(anyhow::Error::from))?;

        Ok(Boss::new(hit_points, damage))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn values() -> [Spell; 5] {
        [
            Spell::MagicMissile,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge,
        ]
    }

    fn cost(&self) -> usize {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }
}
