use std::iter::once;

use anyhow::{anyhow, Ok};
use itertools::Itertools;
use model::Preference;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let preferences = parse::parse(input)?;

    let attendees: Vec<&str> = preferences
        .iter()
        .map(|preference| preference.person())
        .unique()
        .collect();

    let mut max_happiness = isize::MIN;

    for permutation in attendees.iter().permutations(attendees.len()) {
        let happiness = change_in_happiness(&permutation, &preferences)?;
        max_happiness = max_happiness.max(happiness);
    }

    Ok(max_happiness.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let preferences = parse::parse(input)?;

    let attendees: Vec<&str> = preferences
        .iter()
        .map(|preference| preference.person())
        .unique()
        .chain(once("ME"))
        .collect();

    let mut max_happiness = isize::MIN;

    for permutation in attendees.iter().permutations(attendees.len()) {
        let happiness = change_in_happiness(&permutation, &preferences)?;
        max_happiness = max_happiness.max(happiness);
    }

    Ok(max_happiness.to_string())
}

fn change_in_happiness(attendees: &[&&str], preferences: &[Preference]) -> anyhow::Result<isize> {
    let mut happiness = 0;

    for (index, attendee) in attendees.iter().enumerate() {
        if **attendee == "ME" {
            continue;
        }

        let neighbor_before = if index == 0 {
            attendees[attendees.len() - 1]
        } else {
            attendees[index - 1]
        };

        let happiness_before = if *neighbor_before == "ME" {
            0
        } else {
            preferences
                .iter()
                .find(|preference| {
                    &&preference.person() == attendee && &preference.neighbor() == neighbor_before
                })
                .map(|preference| preference.happiness())
                .ok_or(anyhow!(
                    "Preference not found for attendee={}, neighbor={}",
                    attendee,
                    neighbor_before
                ))?
        };

        let neighbor_after = if index == attendees.len() - 1 {
            attendees[0]
        } else {
            attendees[index + 1]
        };

        let happiness_after = if *neighbor_after == "ME" {
            0_isize
        } else {
            preferences
                .iter()
                .find(|preference| {
                    &&preference.person() == attendee && &preference.neighbor() == neighbor_after
                })
                .map(|preference| preference.happiness())
                .ok_or(anyhow!(
                    "Preference not found for attendee={}, neighbor={}",
                    attendee,
                    neighbor_after
                ))?
        };

        happiness += happiness_before + happiness_after;
    }

    Ok(happiness)
}

mod model {
    #[derive(Debug, PartialEq, Eq, Clone, Hash)]
    pub struct Preference {
        person: String,
        neighbor: String,
        happiness: i32,
    }

    impl Preference {
        pub fn new(person: String, neighbor: String, happiness: i32) -> Self {
            Self {
                person,
                neighbor,
                happiness,
            }
        }

        pub fn person(&self) -> &str {
            &self.person
        }

        pub fn neighbor(&self) -> &str {
            &self.neighbor
        }

        pub fn happiness(&self) -> isize {
            self.happiness as isize
        }
    }
}

mod parse {
    use super::model::Preference;

    pub fn parse(input: &str) -> anyhow::Result<Vec<Preference>> {
        let mut preferences = Vec::new();

        for line in input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let person = parts[0];
            let neighbor = parts[10].trim_end_matches('.');
            let happiness = match parts[2] {
                "gain" => parts[3].parse::<i32>()?,
                "lose" => -parts[3].parse::<i32>()?,
                _ => unreachable!(),
            };

            preferences.push(Preference::new(
                person.to_string(),
                neighbor.to_string(),
                happiness,
            ));
        }

        Ok(preferences)
    }
}
