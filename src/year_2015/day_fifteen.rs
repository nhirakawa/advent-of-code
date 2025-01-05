use std::collections::HashSet;

use log::{debug, trace};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let ingredients = parse::parse(input)?;

    debug!("Ingredients: {:?}", ingredients);

    let max_score = mix_ingredients(&ingredients, &CalorieConstraint::None);

    Ok(max_score.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let ingredients = parse::parse(input)?;

    debug!("Ingredients: {:?}", ingredients);

    let max_score = mix_ingredients(&ingredients, &CalorieConstraint::MealReplacement);

    Ok(max_score.to_string())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum CalorieConstraint {
    None,
    MealReplacement,
}

fn mix_ingredients(ingredients: &[Ingredient], calorire_constraint: &CalorieConstraint) -> i32 {
    let mut max_score = 0;

    let mut stack = vec![DfsState {
        counts: vec![0; ingredients.len()],
        remaining_teaspoons: 100,
    }];

    let mut seen = HashSet::new();

    while let Some(state) = stack.pop() {
        if !seen.insert(state.clone()) {
            continue;
        }

        trace!("Evaluating {:?}", state);

        if state.remaining_teaspoons == 0 {
            let score = calculate_score(state.counts.clone(), ingredients, calorire_constraint);
            max_score = i32::max(max_score, score);
            continue;
        }

        for ingredient_index in 0..ingredients.len() {
            let mut new_counts = state.counts.clone();
            new_counts[ingredient_index] += 1;

            let new_state = DfsState {
                counts: new_counts,
                remaining_teaspoons: state.remaining_teaspoons - 1,
            };

            if !seen.contains(&new_state) {
                stack.push(new_state);
            }
        }
    }

    max_score
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct DfsState {
    counts: Vec<usize>,
    remaining_teaspoons: usize,
}

fn calculate_score(
    counts: Vec<usize>,
    ingredients: &[Ingredient],
    calorire_constraint: &CalorieConstraint,
) -> i32 {
    let total_count = counts.iter().sum::<usize>();

    if total_count != 100 {
        panic!("Expected 100, got {}", total_count);
    }

    trace!("Counts: {:?}", counts);

    let mut total_capacity = 0;
    let mut total_durability = 0;
    let mut total_flavor = 0;
    let mut total_texture = 0;
    let mut total_calories = 0;

    for (index, count) in counts.iter().enumerate() {
        let ingredient = &ingredients[index];

        total_capacity += *count as i32 * ingredient.capacity;
        total_durability += *count as i32 * ingredient.durability;
        total_flavor += *count as i32 * ingredient.flavor;
        total_texture += *count as i32 * ingredient.texture;
        total_calories += *count as i32 * ingredient.calories;
    }

    if let CalorieConstraint::MealReplacement = calorire_constraint {
        if total_calories != 500 {
            return 0;
        }
    }

    let total_capacity = i32::max(total_capacity, 0);
    let total_durability = i32::max(total_durability, 0);
    let total_flavor = i32::max(total_flavor, 0);
    let total_texture = i32::max(total_texture, 0);

    trace!("Capacity: {}", total_capacity);
    trace!("Durability: {}", total_durability);
    trace!("Flavor: {}", total_flavor);
    trace!("Texture: {}", total_texture);

    total_capacity * total_durability * total_flavor * total_texture
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Ingredient<'a> {
    name: &'a str,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl<'a> Ingredient<'a> {
    pub fn new(
        name: &'a str,
        capacity: i32,
        durability: i32,
        flavor: i32,
        texture: i32,
        calories: i32,
    ) -> Self {
        Self {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

mod parse {
    use anyhow::{anyhow, bail};

    use super::Ingredient;

    pub fn parse(input: &str) -> anyhow::Result<Vec<Ingredient>> {
        let mut ingredients = Vec::new();

        for line in input.lines() {
            let (name, rest) = line
                .split_once(": ")
                .ok_or(anyhow!("Could not extract name from {}", line))?;

            let properties = rest.split(", ").collect::<Vec<_>>();

            if properties.len() != 5 {
                bail!(
                    "Expected 5 properties, got {} - {:?}",
                    properties.len(),
                    properties
                );
            }

            let capacity = properties[0]
                .split_once(" ")
                .ok_or(anyhow!("Could not extract capacity from {}", properties[0]))?
                .1
                .parse::<i32>()?;

            let durability = properties[1]
                .split_once(" ")
                .ok_or(anyhow!(
                    "Could not extract durability from {}",
                    properties[1]
                ))?
                .1
                .parse::<i32>()?;

            let flavor = properties[2]
                .split_once(" ")
                .ok_or(anyhow!("Could not extract flavor from {}", properties[2]))?
                .1
                .parse::<i32>()?;

            let texture = properties[3]
                .split_once(" ")
                .ok_or(anyhow!("Could not extract texture from {}", properties[3]))?
                .1
                .parse::<i32>()?;

            let calories = properties[4]
                .split_once(" ")
                .ok_or(anyhow!("Could not extract calories from {}", properties[4]))?
                .1
                .parse::<i32>()?;

            let ingredient = Ingredient::new(name, capacity, durability, flavor, texture, calories);
            ingredients.push(ingredient);
        }

        Ok(ingredients)
    }
}
