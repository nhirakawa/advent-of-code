use std::collections::HashSet;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (mut lights, max_x, max_y) = parse(input)?;

    for _ in 0..100 {
        lights = flip_lights(lights, max_x, max_y, CornerStrategy::NotSpecial);
    }

    Ok(lights.len())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let (mut lights, max_x, max_y) = parse(input)?;

    for _ in 0..100 {
        lights = flip_lights(lights, max_x, max_y, CornerStrategy::AlwaysOn);
    }

    Ok(lights.len())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum CornerStrategy {
    NotSpecial,
    AlwaysOn,
}

fn flip_lights(
    lights: HashSet<Coordinate>,
    max_x: isize,
    max_y: isize,
    corner_strategy: CornerStrategy,
) -> HashSet<Coordinate> {
    let mut new_lights = HashSet::new();

    for x in 0..=max_x {
        for y in 0..=max_y {
            if corner_strategy == CornerStrategy::AlwaysOn
                && (x == 0 || x == max_x)
                && (y == 0 || y == max_y)
            {
                new_lights.insert((x, y));
                continue;
            }

            let on_neighbors = [
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x - 1, y),
                (x + 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ]
            .iter()
            .filter(|n| lights.contains(n))
            .count();

            if lights.contains(&(x, y)) {
                if on_neighbors == 2 || on_neighbors == 3 {
                    new_lights.insert((x, y));
                }
            } else if on_neighbors == 3 {
                new_lights.insert((x, y));
            }
        }
    }

    new_lights
}

type Coordinate = (isize, isize);

fn parse(input: &str) -> anyhow::Result<(HashSet<Coordinate>, isize, isize)> {
    let mut lights = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            max_x = max_x.max(x);
            max_y = max_y.max(y);

            if c == '#' {
                lights.insert((x as isize, y as isize));
            }
        }
    }
    Ok((lights, max_x as isize, max_y as isize))
}
