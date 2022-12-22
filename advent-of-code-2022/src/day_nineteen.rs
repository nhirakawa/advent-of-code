use common::prelude::*;
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

    let _elapsed = start.elapsed().unwrap();
    PartAnswer::default()
}

fn part_two(_input: &str) -> PartAnswer {
    let start = SystemTime::now();
    let _elapsed = start.elapsed().unwrap();
    PartAnswer::default()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Cost {
    resource: Resource,
    amount: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Recipe {
    resource: Resource,
    costs: Vec<Cost>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Blueprint {
    id: usize,
    recipes: Vec<Recipe>,
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
        map(cost, |cost| vec![cost]),
        separated_list1(tag(" and "), cost),
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
