use std::collections::{HashMap, HashSet, VecDeque};

use common::{math::triangular_number, prelude::*};
use log::debug;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::{map, value},
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-19.txt");

    let part_one = part_one(input);
    let part_two = part_two(input);

    Ok((part_one, part_two))
}

fn part_one(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let blueprints = parse(input);

    let mut sum = 0;

    for blueprint in blueprints {
        let blueprint_id = blueprint.id;
        let geode_count = search_state_space(&blueprint);

        println!("Blueprint {blueprint_id} produced {geode_count} geodes");

        let quality_level = blueprint_id * geode_count;
        sum += quality_level;
    }

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(sum, elapsed)
}

fn part_two(_input: &str) -> PartAnswer {
    let start = SystemTime::now();
    let _elapsed = start.elapsed().unwrap();
    PartAnswer::default()
}

/**
 * Return the most geodes that can be produced with this blueprint
 */
fn search_state_space(blueprint: &Blueprint) -> usize {
    let mut max_geode_count = 0;

    let mut queue = VecDeque::new();

    let starting_state = SearchState::new(vec![], 24, vec![(Resource::Ore, 1)]);
    queue.push_back(starting_state);

    let mut iterations = 0;

    while let Some(current_state) = queue.pop_front() {
        iterations += 1;

        if iterations % 1_000 == 0 {
            println!("Iterations {iterations}, queue size {}", queue.len());
        }

        // if we're out of time, we've reached the final state
        if current_state.time_remaining == 0 {
            max_geode_count =
                max_geode_count.max(current_state.get_amount_of_resource(&Resource::Geode));
            continue;
        }

        // https://www.reddit.com/r/adventofcode/comments/zpihwi/2022_day_19_solutions/j0tls7a/
        let most_optimistic_geode_count = current_state.get_amount_of_resource(&Resource::Geode)
            + (current_state.number_of_resource_robots(&Resource::Geode)
                * current_state.time_remaining)
            + triangular_number(current_state.time_remaining) as usize;

        if most_optimistic_geode_count < max_geode_count {
            continue;
        }

        for resource in Resource::all() {
            if let Some(next_state) = current_state.next_search_state(&resource, blueprint) {
                debug!("found search state for {resource:?}");
                queue.push_back(next_state);
            } else {
                debug!("no search state for {:?}", resource);
            }
        }
    }

    max_geode_count
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct SearchState {
    resources: Vec<(Resource, usize)>,
    time_remaining: usize,
    robots: Vec<(Resource, usize)>,
}

impl SearchState {
    fn new(
        resources: Vec<(Resource, usize)>,
        time_remaining: usize,
        robots: Vec<(Resource, usize)>,
    ) -> SearchState {
        SearchState {
            resources,
            time_remaining,
            robots,
        }
    }

    // build the next search state with the assumption that we're building a robot that outputs `resource`
    // we'll also assume that when we check the next search state, we've accumulated the resources between now and then
    fn next_search_state(
        &self,
        robot_type: &Resource,
        blueprint: &Blueprint,
    ) -> Option<SearchState> {
        // across all recipes, what is the max consumption of this resource
        let max_consumption = blueprint.get_max_resource_consumption(&robot_type);

        /*
         * if we're already producing the maximum required amount of this resource (excluding geode),
         * we don't need any more of this kind of robot
         */
        if *robot_type != Resource::Geode
            && self.number_of_resource_robots(&robot_type) >= max_consumption
        {
            debug!(
                "max consumption for {robot_type:?} has already been reached ({max_consumption})"
            );
            return None;
        }

        let recipe = blueprint.get_recipe_for_resource(robot_type);

        let mut time_necessary = 0;

        for cost in &recipe.costs {
            let current_resource_count = self.get_amount_of_resource(&cost.resource);

            if current_resource_count >= cost.amount {
                // if we already have the resources we need, we don't need to check robots
                debug!(
                    "{robot_type:?} costs {} {:?}, which we already have ({current_resource_count})",
                    cost.amount, cost.resource
                );
                continue;
            }

            let resource_count_needed = cost.amount - current_resource_count;

            debug!(
                "need {resource_count_needed} {:?} to produce 1 {robot_type:?} robot",
                cost.resource
            );

            // count the number of robots
            // if we have no robots, we can't actually produce
            let number_of_robots = self.robots.iter().find_map(|(resource_type, count)| {
                if resource_type == &cost.resource {
                    Some(*count)
                } else {
                    None
                }
            })?;

            debug!(
                "we have {number_of_robots} robots to produce {:?}",
                cost.resource
            );

            let time_needed = if resource_count_needed % number_of_robots == 0 {
                resource_count_needed / number_of_robots
            } else {
                (resource_count_needed / number_of_robots) + 1
            };

            time_necessary = time_necessary.max(time_needed);
        }

        if time_necessary + 1 > self.time_remaining {
            /*
             * If we don't have enough time to build another resource robot, just skip to the end
             */

            let mut resources = vec![];

            for resource in Resource::all() {
                let existing_resources = self.get_amount_of_resource(&resource);
                let resources_added =
                    self.number_of_resource_robots(&resource) * self.time_remaining;

                resources.push((resource, existing_resources + resources_added));
            }

            return Some(SearchState::new(resources, 0, self.robots.clone()));
            // panic!(
            //     "Need {} minutes to make {:?} robot but only {} minutes remain",
            //     time_necessary, resource, self.time_remaining
            // );
        }

        /*
         * We need `time_necessary` minutes to make all of the resources
         * We also need 1 more cycle before the robot is made
         */
        let time_remaining = self.time_remaining - time_necessary - 1;

        /*
         * Add 1 more robot
         * Either increment the count or add a new value to the vector
         */
        let has_existing_robot = self
            .robots
            .iter()
            .any(|(resource_type, _)| resource_type == robot_type);

        let robots: Vec<(Resource, usize)> = if has_existing_robot {
            self.robots
                .iter()
                .map(|(resource_type, count)| {
                    if resource_type == robot_type {
                        (*resource_type, count + 1)
                    } else {
                        (*resource_type, *count)
                    }
                })
                .collect()
        } else {
            let mut robots = self.robots.clone();
            robots.push((*robot_type, 1));
            robots
        };

        /*
         * Add resources from time advance, then subtract the cost of the robot that we'll have
         */
        let mut resources = vec![];
        for resource in Resource::all() {
            let amount_available = (self.number_of_resource_robots(&resource) * time_necessary)
                + self.get_amount_of_resource(&resource);
            let amount_required = recipe.get_required_amount(&resource);

            let amount_remaining = amount_available - amount_required;

            resources.push((resource, amount_remaining))
        }

        Some(SearchState::new(resources, time_remaining, robots))
    }

    fn get_amount_of_resource(&self, resource: &Resource) -> usize {
        self.resources
            .iter()
            .find_map(|(resource_type, amount)| {
                if resource_type == resource {
                    Some(*amount)
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }

    /**
     * Count the number of robots that can produce `resource`
     */
    fn number_of_resource_robots(&self, resource: &Resource) -> usize {
        self.robots
            .iter()
            .find_map(|(produces_resource, count)| {
                if produces_resource == resource {
                    Some(*count)
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    fn all() -> Vec<Resource> {
        vec![
            Resource::Ore,
            Resource::Clay,
            Resource::Obsidian,
            Resource::Geode,
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Cost {
    resource: Resource,
    amount: usize,
}

impl Cost {
    fn new(resource: Resource, amount: usize) -> Cost {
        Cost { resource, amount }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Recipe {
    resource: Resource,
    costs: Vec<Cost>,
}

impl Recipe {
    fn new(resource: Resource, costs: Vec<Cost>) -> Recipe {
        Recipe { resource, costs }
    }
    /**
     * For any recipe, what's the most `resource` they can consume in a single minute?
     * There's no point in having more `resource` than this robot can consume, since we can only build 1 robot in a given minute
     */
    fn get_max_resource_consumption(&self, resource: &Resource) -> usize {
        self.costs
            .iter()
            .filter_map(|cost| {
                if cost.resource == *resource {
                    Some(cost.amount)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0)
    }

    /**
     * return the amount of `resource` required to build this robot
     */
    fn get_required_amount(&self, resource: &Resource) -> usize {
        self.costs
            .iter()
            .find_map(|cost| {
                if cost.resource == *resource {
                    Some(cost.amount)
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Blueprint {
    id: usize,
    recipes: Vec<Recipe>,
}

impl Blueprint {
    fn new(id: usize, recipes: Vec<Recipe>) -> Blueprint {
        Blueprint { id, recipes }
    }
    /**
     * Across all recipes, what's the most `resource` they can consume in a single minute?
     * There's no point in having more `resource` than this robot can consume, since we can only build 1 robot in a given minute
     */
    fn get_max_resource_consumption(&self, resource: &Resource) -> usize {
        self.recipes
            .iter()
            .map(|recipe| recipe.get_max_resource_consumption(resource))
            .max()
            .unwrap_or(0)
    }

    fn get_recipe_for_resource(&self, resource: &Resource) -> Recipe {
        self.recipes
            .iter()
            .find(|recipe| recipe.resource == *resource)
            .cloned()
            .unwrap()
    }
}

fn parse(i: &str) -> Vec<Blueprint> {
    finish(blueprints)(i).unwrap().1
}

fn blueprints(i: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(multispace1, blueprint)(i)
}

fn blueprint(i: &str) -> IResult<&str, Blueprint> {
    map(
        tuple((
            tag("Blueprint "),
            unsigned_number,
            tag(":"),
            multispace1,
            recipes,
        )),
        |(_, id, _, _, recipes)| Blueprint { id, recipes },
    )(i)
}

fn recipes(i: &str) -> IResult<&str, Vec<Recipe>> {
    separated_list1(multispace1, recipe)(i)
}

fn recipe(i: &str) -> IResult<&str, Recipe> {
    map(
        terminated(
            tuple((tag("Each "), resource, tag(" robot costs "), costs)),
            tag("."),
        ),
        |(_, resource, _, costs)| Recipe { resource, costs },
    )(i)
}

fn costs(i: &str) -> IResult<&str, Vec<Cost>> {
    alt((
        map(
            separated_pair(cost, tag(" and "), cost),
            |(first, second)| vec![first, second],
        ),
        map(cost, |cost| vec![cost]),
    ))(i)
}

fn cost(i: &str) -> IResult<&str, Cost> {
    map(
        separated_pair(unsigned_number, tag(" "), resource),
        |(amount, resource)| Cost { amount, resource },
    )(i)
}

fn resource(i: &str) -> IResult<&str, Resource> {
    alt((ore, clay, obsidian, geode))(i)
}

fn ore(i: &str) -> IResult<&str, Resource> {
    value(Resource::Ore, tag("ore"))(i)
}

fn clay(i: &str) -> IResult<&str, Resource> {
    value(Resource::Clay, tag("clay"))(i)
}

fn obsidian(i: &str) -> IResult<&str, Resource> {
    value(Resource::Obsidian, tag("obsidian"))(i)
}

fn geode(i: &str) -> IResult<&str, Resource> {
    value(Resource::Geode, tag("geode"))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_costs_parser() {
        assert_eq!(
            costs("4 ore and 14 clay"),
            Ok((
                "",
                vec![Cost::new(Resource::Ore, 4), Cost::new(Resource::Clay, 14)]
            ))
        );
    }

    #[test]
    fn test_next_search_state() {
        let search_state = SearchState::new(vec![], 24, vec![(Resource::Ore, 2)]);
        let blueprint = Blueprint::new(
            1,
            vec![
                Recipe::new(Resource::Clay, vec![Cost::new(Resource::Ore, 2)]),
                Recipe::new(Resource::Ore, vec![Cost::new(Resource::Ore, 3)]),
            ],
        );

        let next_search_state = search_state.next_search_state(&Resource::Ore, &blueprint);
        assert!(next_search_state.is_some());

        let next_search_state = next_search_state.unwrap();
        assert_eq!(next_search_state.time_remaining, 21);
        assert_eq!(next_search_state.robots, vec![(Resource::Ore, 3)]);
        assert_eq!(
            next_search_state.resources,
            vec![
                (Resource::Ore, 1),
                (Resource::Clay, 0),
                (Resource::Obsidian, 0),
                (Resource::Geode, 0)
            ]
        );
    }

    #[test]
    fn test_next_search_state_not_enough_time_remaining() {
        let search_state =
            SearchState::new(vec![], 2, vec![(Resource::Ore, 1), (Resource::Clay, 1)]);

        let blueprint = Blueprint::new(
            1,
            vec![
                Recipe::new(Resource::Clay, vec![Cost::new(Resource::Ore, 5)]),
                Recipe::new(Resource::Geode, vec![Cost::new(Resource::Clay, 10)]),
            ],
        );

        let next_search_state = search_state.next_search_state(&Resource::Geode, &blueprint);
        assert!(next_search_state.is_some());

        let next_search_state = next_search_state.unwrap();

        assert_eq!(
            next_search_state.resources,
            vec![
                (Resource::Ore, 2),
                (Resource::Clay, 2),
                (Resource::Obsidian, 0),
                (Resource::Geode, 0)
            ]
        );
        assert_eq!(next_search_state.time_remaining, 0);
        assert_eq!(next_search_state.robots, search_state.robots);
    }
}
