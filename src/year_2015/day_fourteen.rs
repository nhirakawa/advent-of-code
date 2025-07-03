use anyhow::anyhow;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let reindeer_list = parse::parse(input)?;

    let reindeer_list = race(reindeer_list, 2503)?;

    let max_distance = reindeer_list
        .iter()
        .map(|reindeer| reindeer.distance_travelled)
        .max()
        .ok_or_else(|| anyhow!("No reindeer found"))?;

    Ok(max_distance)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let reindeer_list = parse::parse(input)?;

    let reindeer_list = race(reindeer_list, 2503)?;

    let max_score = reindeer_list
        .iter()
        .map(|reindeer| reindeer.score)
        .max()
        .ok_or_else(|| anyhow!("No reindeer found"))?;

    Ok(max_score)
}

fn race(mut reindeer_list: Vec<Reindeer>, time: usize) -> anyhow::Result<Vec<Reindeer>> {
    for _ in 0..time {
        for reindeer in reindeer_list.iter_mut() {
            if reindeer.moving_time_remaining > 0 {
                reindeer.moving_time_remaining -= 1;
                reindeer.distance_travelled += reindeer.speed;

                if reindeer.moving_time_remaining == 0 {
                    reindeer.resting_time_remaining = reindeer.resting_time;
                }
            } else if reindeer.resting_time_remaining > 0 {
                reindeer.resting_time_remaining -= 1;

                if reindeer.resting_time_remaining == 0 {
                    reindeer.moving_time_remaining = reindeer.moving_time;
                }
            }
        }

        let max_distance = reindeer_list
            .iter()
            .map(|reindeer| reindeer.distance_travelled)
            .max()
            .ok_or_else(|| anyhow!("No reindeer found"))?;

        for reindeer in reindeer_list.iter_mut() {
            if reindeer.distance_travelled == max_distance {
                reindeer.score += 1;
            }
        }
    }

    Ok(reindeer_list)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Reindeer<'a> {
    name: &'a str,
    speed: usize,
    distance_travelled: usize,
    score: usize,
    moving_time: usize,
    resting_time: usize,
    moving_time_remaining: usize,
    resting_time_remaining: usize,
}

impl<'a> Reindeer<'a> {
    pub fn new(name: &'a str, speed: usize, moving_time: usize, resting_time: usize) -> Self {
        Self {
            name,
            speed,
            distance_travelled: 0,
            score: 0,
            moving_time,
            resting_time,
            moving_time_remaining: moving_time,
            resting_time_remaining: 0,
        }
    }
}

mod parse {
    use anyhow::bail;
    use itertools::Itertools;

    use super::Reindeer;

    pub fn parse(input: &str) -> anyhow::Result<Vec<Reindeer>> {
        let mut reindeer_list = Vec::new();

        for line in input.lines() {
            let line = line.split_whitespace().collect_vec();

            if line.len() != 15 {
                bail!("Line {} is not valid", line.join(" "));
            }

            let name = line[0];
            let speed = line[3].parse()?;
            let moving_time = line[6].parse()?;
            let resting_time = line[13].parse()?;

            let reindeer = Reindeer::new(name, speed, moving_time, resting_time);
            reindeer_list.push(reindeer);
        }

        Ok(reindeer_list)
    }
}
