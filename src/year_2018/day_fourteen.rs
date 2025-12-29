use std::str::FromStr;

use anyhow::{anyhow, bail};
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let number_of_recipes = input.parse::<usize>()?;
    let mut recipes = Recipes::new();
    Ok(recipes
        .make_recipes(number_of_recipes)
        .iter()
        .map(|u| u.to_string())
        .collect::<String>())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let suffix = input
        .split("")
        .filter_map(|s| s.parse::<u8>().ok())
        .collect_vec();
    if suffix.len() != input.len() {
        bail!("Could not parse {input} as digits");
    }
    let mut recipes = Recipes::new();
    let number_of_recipes = recipes.make_recipes_until_suffix(&suffix);
    Ok(number_of_recipes)
}

struct Recipes {
    // Index of the recipe for the first elf
    first_elf: usize,
    // Index of the recipe for the second elf
    second_elf: usize,
    scoreboard: Vec<u8>,
}

impl Recipes {
    fn new() -> Recipes {
        // NOTE: An earlier version of code pre-allocated a large capacity for the scoreboard
        // since we anticipated needing to grow the arary overtime. In practice,
        // this didn't meaningfully impact the timing of the solution.
        Recipes {
            first_elf: 0,
            second_elf: 1,
            scoreboard: vec![3, 7],
        }
    }

    fn make_recipes(&mut self, number_of_recipes: usize) -> Vec<u8> {
        while self.scoreboard.len() < 10 + number_of_recipes {
            self.make_recipe();
        }

        self.scoreboard
            .iter()
            .skip(number_of_recipes)
            .take(10)
            .copied()
            .collect()
    }

    fn make_recipes_until_suffix(&mut self, suffix: &[u8]) -> usize {
        while !self.contains_near_end(suffix) {
            self.make_recipe();
        }

        if self.scoreboard.ends_with(suffix) {
            self.scoreboard.len() - suffix.len()
        } else {
            self.scoreboard.len() - suffix.len() - 1
        }
    }

    fn make_recipe(&mut self) {
        let first_elf_recipe = self.scoreboard[self.first_elf];
        let second_elf_recipe = self.scoreboard[self.second_elf];

        match Self::make_new_recipe(first_elf_recipe, second_elf_recipe) {
            (first_new_recipe, Some(second_new_recipe)) => {
                self.scoreboard.push(first_new_recipe);
                self.scoreboard.push(second_new_recipe);
            }
            (first_new_recipe, None) => {
                self.scoreboard.push(first_new_recipe);
            }
        };

        let first_elf = (self.first_elf + 1 + first_elf_recipe as usize) % self.scoreboard.len();
        let second_elf = (self.second_elf + 1 + second_elf_recipe as usize) % self.scoreboard.len();

        self.first_elf = first_elf;
        self.second_elf = second_elf;
    }

    fn make_new_recipe(first: u8, second: u8) -> (u8, Option<u8>) {
        let result = first + second;

        if result < 10 {
            (result, None)
        } else {
            (1, Some(result - 10))
        }
    }

    // We expect bytes near the end (or else we would have seen it in a previous iteration)
    // but we're not guaranteed to see it as a strict suffix.
    fn contains_near_end(&self, bytes: &[u8]) -> bool {
        // TODO: this searches a wider window than strictly necessary (to avoid a potential off-by-one)
        let search_window = bytes.len() + 1;

        // Handle a potential underflow by defaulting to the start of the list
        let search_range_start = self.scoreboard.len().saturating_sub(search_window);

        self.scoreboard[search_range_start..]
            .windows(bytes.len())
            .any(|window| window == bytes)
    }
}

impl FromStr for Recipes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut scoreboard = Vec::new();
        let mut first_elf = None;
        let mut second_elf = None;

        for (index, token) in s.split_whitespace().enumerate() {
            if token.starts_with('(') && token.ends_with(')') {
                let value = token[1..token.len() - 1].parse::<u8>()?;
                scoreboard.push(value);
                first_elf = Some(index);
            } else if token.starts_with('[') && token.ends_with(']') {
                let value = token[1..token.len() - 1].parse::<u8>()?;
                scoreboard.push(value);
                second_elf = Some(index);
            } else {
                let value = token.parse::<u8>()?;
                scoreboard.push(value);
            }
        }

        let first_elf = first_elf
            .ok_or_else(|| anyhow!("No first elf position found (missing parentheses)"))?;
        let second_elf = second_elf
            .ok_or_else(|| anyhow!("No second elf position found (missing square brackets)"))?;

        Ok(Recipes {
            first_elf,
            second_elf,
            scoreboard,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_new_recipe() {
        assert_eq!(Recipes::make_new_recipe(3, 4), (7, None));
        assert_eq!(Recipes::make_new_recipe(3, 7), (1, Some(0)));
    }

    #[test]
    fn test_make_recipe() {
        let mut recipes = Recipes::new();

        recipes.make_recipe();

        assert_eq!(recipes.first_elf, 0);
        assert_eq!(recipes.second_elf, 1);
        assert_eq!(recipes.scoreboard, vec![3, 7, 1, 0]);

        recipes.make_recipe();

        assert_eq!(recipes.first_elf, 4);
        assert_eq!(recipes.second_elf, 3);
        assert_eq!(recipes.scoreboard, vec![3, 7, 1, 0, 1, 0]);

        recipes.make_recipe();

        assert_eq!(recipes.first_elf, 6);
        assert_eq!(recipes.second_elf, 4);
        assert_eq!(recipes.scoreboard, vec![3, 7, 1, 0, 1, 0, 1]);

        recipes.make_recipe();

        assert_eq!(recipes.first_elf, 0);
        assert_eq!(recipes.second_elf, 6);
        assert_eq!(recipes.scoreboard, vec![3, 7, 1, 0, 1, 0, 1, 2]);
    }

    #[test]
    fn test_make_recipes_example_one() {
        let mut recipes = Recipes::new();

        let score: String = recipes
            .make_recipes(9)
            .iter()
            .map(|u| u.to_string())
            .collect();

        assert_eq!(score, "5158916779");
    }

    #[test]
    fn test_make_recipes_example_two() {
        let mut recipes = Recipes::new();

        let score: String = recipes
            .make_recipes(5)
            .iter()
            .map(|u| u.to_string())
            .collect();

        assert_eq!(score, "0124515891");
    }

    #[test]
    fn test_make_recipes_example_three() {
        let mut recipes = Recipes::new();

        let score: String = recipes
            .make_recipes(18)
            .iter()
            .map(|u| u.to_string())
            .collect();

        assert_eq!(score, "9251071085");
    }

    #[test]
    fn test_make_recipes_example_four() {
        let mut recipes = Recipes::new();

        let score: String = recipes
            .make_recipes(2018)
            .iter()
            .map(|u| u.to_string())
            .collect();

        assert_eq!(score, "5941429882");
    }

    #[test]
    fn test_make_recipes_until_suffix_example_one() {
        let mut recipes = Recipes::new();
        assert_eq!(recipes.make_recipes_until_suffix(&[5, 1, 5, 8, 9]), 9);
    }

    #[test]
    fn test_make_recipes_until_suffix_example_two() {
        let mut recipes = Recipes::new();
        assert_eq!(recipes.make_recipes_until_suffix(&[0, 1, 2, 4, 5]), 5);
    }

    #[test]
    fn test_make_recipes_until_suffix_example_three() {
        let mut recipes = Recipes::new();
        assert_eq!(recipes.make_recipes_until_suffix(&[9, 2, 5, 1, 0]), 18);
    }

    #[test]
    fn test_make_recipes_until_suffix_example_four() {
        let mut recipes = Recipes::new();
        assert_eq!(recipes.make_recipes_until_suffix(&[5, 9, 4, 1, 4]), 2018);
    }

    #[test]
    fn test_contains_near_end() {
        let recipes = Recipes {
            first_elf: 0,
            second_elf: 1,
            scoreboard: vec![1, 0, 1, 4, 7, 1, 9, 2, 6, 8, 6, 6, 3],
        };

        assert!(recipes.contains_near_end(&[8, 6, 6, 3]));
        assert!(recipes.contains_near_end(&[8, 6, 6]));
        assert!(!recipes.contains_near_end(&[1, 0, 1]));
        assert!(!recipes.contains_near_end(&[0, 0, 0, 0, 0]));
    }
}
