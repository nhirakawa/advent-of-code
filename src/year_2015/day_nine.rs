use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::alpha1, combinator::map, multi::separated_list1,
    sequence::separated_pair, IResult, Parser,
};

use crate::common::parse::{finish, unsigned_number};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let distances = parse(input)?;

    get_all_hamiltonian_distances(distances)
        .min()
        .map(|u| u.to_string())
        .ok_or(anyhow::anyhow!("No shortest path found"))
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let distances = parse(input)?;

    get_all_hamiltonian_distances(distances)
        .max()
        .map(|u| u.to_string())
        .ok_or(anyhow::anyhow!("No longest path found"))
}

fn get_all_hamiltonian_distances(
    distances: HashMap<LocationPair, u32>,
) -> impl Iterator<Item = u32> {
    let unique_locations = unique_locations(distances.keys().cloned());

    let number_of_unique_locations = unique_locations.len();

    unique_locations
        .into_iter()
        .permutations(number_of_unique_locations)
        .map(move |locations| {
            locations
                .into_iter()
                .tuple_windows()
                .filter_map(|(first, second)| distances.get(&(first, second)))
                .copied()
                .sum::<u32>()
        })
}

fn unique_locations<I: IntoIterator<Item = LocationPair>>(location_pairs: I) -> HashSet<Location> {
    location_pairs
        .into_iter()
        .flat_map(|(from, to)| vec![from, to])
        .collect()
}

type Location = String;
type LocationPair = (Location, Location);
type Distance = ((Location, Location), u32);
type Distances = Vec<Distance>;

fn parse(i: &str) -> anyhow::Result<HashMap<LocationPair, u32>> {
    let distances = finish(distances, i)?;

    let mut map = HashMap::new();

    for ((from, to), distance) in distances {
        map.insert((from.clone(), to.clone()), distance);
        map.insert((to, from), distance);
    }

    Ok(map)
}

fn distances(i: &str) -> IResult<&str, Distances> {
    separated_list1(tag("\n"), distance).parse(i)
}

fn distance(i: &str) -> IResult<&str, Distance> {
    separated_pair(distance_pair, tag(" = "), unsigned_number).parse(i)
}

fn distance_pair(i: &str) -> IResult<&str, (Location, Location)> {
    separated_pair(location, tag(" to "), location).parse(i)
}

fn location(i: &str) -> IResult<&str, Location> {
    map(alpha1, |s: &str| s.to_string()).parse(i)
}
