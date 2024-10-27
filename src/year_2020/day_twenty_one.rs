use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let foods = parse_foods(input);

    let ingredient_to_identified_allergen = identify_allergen_containing_ingredients(&foods);

    let mut counter = 0;

    for food in foods {
        for ingredient in &food.ingredients {
            if !ingredient_to_identified_allergen.contains_key(ingredient) {
                counter += 1;
            }
        }
    }

    Ok(counter.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let foods = parse_foods(input);

    let ingredient_to_identified_allergen = identify_allergen_containing_ingredients(&foods);

    let mut sorted_ingredients: Vec<(String, String)> =
        ingredient_to_identified_allergen.into_iter().collect();

    sorted_ingredients.sort_by_key(|(_, allergen)| allergen.to_string());

    let sorted_ingredients: Vec<String> = sorted_ingredients
        .into_iter()
        .map(|(ingredient, _)| ingredient)
        .collect();

    Ok(sorted_ingredients.join(","))
}

fn identify_allergen_containing_ingredients(foods: &[Food]) -> HashMap<String, String> {
    let mut foods_by_allergen = HashMap::new();

    for food in foods {
        for allergen in &food.allergens {
            if !foods_by_allergen.contains_key(allergen) {
                foods_by_allergen.insert(allergen.clone(), vec![]);
            }

            foods_by_allergen.get_mut(allergen).unwrap().push(food);
        }
    }

    let mut allergens_to_identified_ingredient = HashMap::new();

    while allergens_to_identified_ingredient.len() < foods_by_allergen.len() {
        for (allergen, foods) in &foods_by_allergen {
            let all_ingredients: HashSet<String> = foods
                .clone()
                .into_iter()
                .flat_map(|food| food.ingredients.clone())
                .collect();

            let identified_ingredients: HashSet<String> = allergens_to_identified_ingredient
                .values()
                .map(|s: &String| s.into())
                .collect();

            let mut common_ingredients = &all_ingredients - &identified_ingredients;

            for food in foods {
                let these_ingredients: HashSet<String> =
                    food.ingredients.clone().into_iter().collect();
                common_ingredients = &common_ingredients & &these_ingredients;
            }

            if common_ingredients.len() == 1 {
                let ingredient = common_ingredients.into_iter().next().unwrap();

                allergens_to_identified_ingredient.insert(allergen.clone(), ingredient);
            }
        }
    }

    let ingredient_to_identified_allergen: HashMap<String, String> =
        allergens_to_identified_ingredient
            .into_iter()
            .map(|(allergen, ingredient)| (ingredient, allergen))
            .collect();

    ingredient_to_identified_allergen
}

type Allergen = String;
type Ingredient = String;

#[derive(Debug, PartialEq, Clone)]
struct Food {
    ingredients: Vec<Ingredient>,
    allergens: Vec<Allergen>,
}

impl Food {
    fn new<S: Into<String>>(ingredients: Vec<S>, allergens: Vec<S>) -> Food {
        let ingredients = ingredients.into_iter().map(|s| s.into()).collect();
        let allergens = allergens.into_iter().map(|s| s.into()).collect();

        Food {
            ingredients,
            allergens,
        }
    }
}

fn parse_foods(input: &str) -> Vec<Food> {
    foods(input).unwrap().1
}

fn foods(i: &str) -> IResult<&str, Vec<Food>> {
    separated_list1(tag("\n"), food)(i)
}

fn food(i: &str) -> IResult<&str, Food> {
    map(
        separated_pair(ingredient_list, tag(" "), allergen_list),
        |(ingredients, allergens)| Food::new(ingredients, allergens),
    )(i)
}

fn ingredient_list(i: &str) -> IResult<&str, Vec<Ingredient>> {
    separated_list1(tag(" "), ingredient)(i)
}

fn ingredient(i: &str) -> IResult<&str, Ingredient> {
    map(alpha1, |s: &str| s.into())(i)
}

fn allergen_list(i: &str) -> IResult<&str, Vec<Allergen>> {
    delimited(
        tag("(contains "),
        separated_list0(tag(", "), allergen),
        tag(")"),
    )(i)
}

fn allergen(i: &str) -> IResult<&str, Allergen> {
    map(alpha1, |s: &str| s.into())(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allergen() {
        assert_eq!(allergen("eggs"), Ok(("", "eggs".into())))
    }

    #[test]
    fn test_allergens() {
        assert_eq!(
            allergen_list("(contains dairy, fish)"),
            Ok(("", vec!["dairy".into(), "fish".into()]))
        )
    }

    #[test]
    fn test_food() {
        assert_eq!(
            food("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
            Ok((
                "",
                Food::new(
                    vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"],
                    vec!["dairy", "fish"]
                )
            ))
        );
    }
}
