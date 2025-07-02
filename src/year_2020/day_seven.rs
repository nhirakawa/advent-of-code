use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{all_consuming, into, map, map_res, value},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult, Parser,
};
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let graph = parse_graph(input)?;

    let starting_bag = "shiny gold".to_string();

    let mut queue = vec![starting_bag];
    let mut seen: HashSet<String> = HashSet::new();

    while let Some(current) = queue.pop() {
        let contained_by = graph.contained_by.get(&current);

        if contained_by.is_none() {
            continue;
        }

        let contained_by = contained_by.unwrap();

        for bag in contained_by {
            if seen.insert(bag.clone()) {
                queue.push(bag.clone());
            }
        }
    }

    Ok(seen.len().to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let graph = parse_graph(input)?;

    Ok(get_bag_count(&graph).to_string())
}

fn get_bag_count(graph: &BagGraph) -> u32 {
    let mut cost = 0;

    for (value, color) in graph.contains.get("shiny gold").unwrap() {
        let marginal_cost = value * get_bag_count_recursive(graph, color);

        cost += marginal_cost;
    }

    cost
}

fn get_bag_count_recursive(graph: &BagGraph, current: &str) -> u32 {
    let bags = graph.contains.get(current);

    if bags.is_none() {
        return 0;
    }

    let bags = bags.unwrap();

    if bags.is_empty() {
        return 1;
    }

    let mut cost = 0;

    for (value, color) in bags {
        let marginal_cost = value * get_bag_count_recursive(graph, color);

        cost += marginal_cost;
    }

    cost + 1
}

struct BagGraph {
    contains: HashMap<String, Vec<(u32, String)>>,
    contained_by: HashMap<String, Vec<String>>,
}

impl From<Vec<Bag>> for BagGraph {
    fn from(bags: Vec<Bag>) -> Self {
        let mut contains: HashMap<String, Vec<(u32, String)>> = HashMap::new();
        let mut contained_by: HashMap<String, Vec<String>> = HashMap::new();

        for bag in bags {
            if !contains.contains_key(&bag.color) {
                contains.insert(bag.color.clone(), vec![]);
            }

            for contained in bag.contains {
                contains
                    .get_mut(&bag.color)
                    .unwrap()
                    .push(contained.clone());

                if !contained_by.contains_key(&contained.1) {
                    contained_by.insert(contained.1.clone(), vec![]);
                }

                contained_by
                    .get_mut(&contained.1)
                    .unwrap()
                    .push(bag.color.clone());
            }
        }

        BagGraph {
            contains,
            contained_by,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Bag {
    color: String,
    contains: Vec<(u32, String)>,
}

fn parse_graph(i: &str) -> anyhow::Result<BagGraph> {
    let result: IResult<&str, BagGraph> = into(all_consuming(bags)).parse(i);

    result
        .map(|(_, graph)| graph)
        .map_err(|e| anyhow::Error::from(e.to_owned()))
}

fn bags(i: &str) -> IResult<&str, Vec<Bag>> {
    many1(terminated(bag, tag(".\n"))).parse(i)
}

fn bag(i: &str) -> IResult<&str, Bag> {
    map(
        (containing_bag_color, contain, contains_bags),
        |(containing_bag, _, bags)| Bag {
            color: containing_bag,
            contains: bags,
        },
    )
    .parse(i)
}

fn containing_bag_color(i: &str) -> IResult<&str, String> {
    map((color, space, tag("bags")), |(color, _, _)| color).parse(i)
}

fn contain(i: &str) -> IResult<&str, ()> {
    value((), tag(" contain ")).parse(i)
}

fn contains_bags(i: &str) -> IResult<&str, Vec<(u32, String)>> {
    alt((no_bags, at_least_one_bag)).parse(i)
}

fn no_bags(i: &str) -> IResult<&str, Vec<(u32, String)>> {
    value(Vec::new(), tag("no other bags")).parse(i)
}

fn at_least_one_bag(i: &str) -> IResult<&str, Vec<(u32, String)>> {
    let bag = (number_and_color, space, bag_or_bags);
    let bag = map(bag, |(number_and_color, _, _)| number_and_color);
    separated_list1(tag(", "), bag).parse(i)
}

fn number_and_color(i: &str) -> IResult<&str, (u32, String)> {
    let number_and_color = (number, space, color);
    let mut number_and_color = map(number_and_color, |(number, _, color)| (number, color));

    number_and_color.parse(i)
}

fn number(i: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>()).parse(i)
}

fn color(i: &str) -> IResult<&str, String> {
    let color = (alpha1, space, alpha1);
    let color = map(color, |(first, _, second)| (first, second));
    let mut color = map(color, |(first, second)| format!("{} {}", first, second));

    color.parse(i)
}

fn bag_or_bags(i: &str) -> IResult<&str, ()> {
    value((), alt((tag("bags"), tag("bag")))).parse(i)
}

fn space(i: &str) -> IResult<&str, ()> {
    value((), tag(" ")).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        assert_eq!(color("shiny gold"), Ok(("", "shiny gold".into())));
    }

    #[test]
    fn test_containing_bag_color() {
        assert_eq!(
            containing_bag_color("dark orange bags"),
            Ok(("", "dark orange".into()))
        );
    }

    #[test]
    fn test_number_and_color() {
        assert_eq!(
            number_and_color("2 dark red"),
            Ok(("", (2, "dark red".into())))
        );
    }

    #[test]
    fn test_at_least_one_bag() {
        assert_eq!(
            at_least_one_bag("1 shiny gold bag"),
            Ok(("", vec![(1, "shiny gold".into())]))
        );

        assert_eq!(
            at_least_one_bag("5 dotted black bags"),
            Ok(("", vec![(5, "dotted black".into())]))
        );

        assert_eq!(
            at_least_one_bag("5 faded blue bags, 6 dotted black bags"),
            Ok((
                "",
                vec![(5, "faded blue".into()), (6, "dotted black".into())]
            ))
        );
    }

    #[test]
    fn test_bag() {
        assert_eq!(
            bag("light red bags contain 1 bright white bag, 2 muted yellow bags"),
            Ok((
                "",
                Bag {
                    color: "light red".into(),
                    contains: vec![(1, "bright white".into()), (2, "muted yellow".into())]
                }
            ))
        );

        assert_eq!(
            bag("dotted black bags contain no other bags"),
            Ok((
                "",
                Bag {
                    color: "dotted black".into(),
                    contains: vec![]
                }
            ))
        );
    }

    #[test]
    fn test_bags() {
        assert_eq!(
            bags("light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\n"),
            Ok(("", vec![Bag{color: "light red".into(), contains: vec![(1, "bright white".into()), (2, "muted yellow".into())]}, Bag{color: "dark orange".into(), contains: vec![(3, "bright white".into()), (4, "muted yellow".into())]}]))
        );
    }
}
