use std::collections::HashSet;

use crate::common::parse::griderator;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let paper_rolls = parse_paper_rolls(input);
    Ok(paper_rolls.get_removable().len())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let mut paper_rolls = parse_paper_rolls(input);
    let original_count = paper_rolls.len();

    loop {
        let removed = paper_rolls.without_removable();
        if removed.len() == paper_rolls.len() {
            break;
        }
        paper_rolls = removed;
    }

    Ok(original_count - paper_rolls.len())
}

struct PaperRolls(HashSet<(isize, isize)>);

impl PaperRolls {
    fn iter(&self) -> impl Iterator<Item = (isize, isize)> {
        self.0.iter().copied()
    }

    fn contains(&self, other: &(isize, isize)) -> bool {
        self.0.contains(other)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn get_removable(&self) -> HashSet<(isize, isize)> {
        let mut removable = HashSet::new();

        for paper_roll in self.iter() {
            let neighbors = neighbors(paper_roll);
            let real_neighbor_count = neighbors
                .iter()
                .filter(|coord| self.contains(*coord))
                .count();
            if real_neighbor_count < 4 {
                removable.insert(paper_roll);
            }
        }

        removable
    }

    fn without_removable(&self) -> PaperRolls {
        let removable = self.get_removable();
        PaperRolls(self.0.clone().difference(&removable).copied().collect())
    }
}

fn neighbors((x, y): (isize, isize)) -> [(isize, isize); 8] {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

fn parse_paper_rolls(input: &str) -> PaperRolls {
    let paper_rolls = griderator(input)
        .filter_map(
            |(coordinates, c)| {
                if c == '@' { Some(coordinates) } else { None }
            },
        )
        .collect();

    PaperRolls(paper_rolls)
}
