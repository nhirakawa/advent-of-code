use anyhow::bail;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let aunts_sue = parse::parse(input)?;

    for aunt_sue in aunts_sue {
        if did_aunt_sue_send_package(
            &aunt_sue,
            CatAndTreeOp::Equal,
            PomeranianAndGoldfishOp::Equal,
        ) {
            return Ok(aunt_sue.id);
        }
    }

    bail!("Could not find Aunt Sue")
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let aunts_sue = parse::parse(input)?;

    for aunt_sue in aunts_sue {
        if did_aunt_sue_send_package(
            &aunt_sue,
            CatAndTreeOp::GreaterThan,
            PomeranianAndGoldfishOp::LessThan,
        ) {
            return Ok(aunt_sue.id);
        }
    }

    bail!("Could not find Aunt Sue")
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum CatAndTreeOp {
    GreaterThan,
    Equal,
}

impl CatAndTreeOp {
    fn is_satisfied(&self, value: usize, other: usize) -> bool {
        match self {
            Self::GreaterThan => value > other,
            Self::Equal => value == other,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum PomeranianAndGoldfishOp {
    LessThan,
    Equal,
}

impl PomeranianAndGoldfishOp {
    fn is_satisfied(&self, value: usize, other: usize) -> bool {
        match self {
            Self::LessThan => value < other,
            Self::Equal => value == other,
        }
    }
}

fn did_aunt_sue_send_package(
    aunt_sue: &AuntSue,
    cat_and_tree_op: CatAndTreeOp,
    pomeranian_and_goldfish_op: PomeranianAndGoldfishOp,
) -> bool {
    if let Some(children) = aunt_sue.children
        && children != 3
    {
        return false;
    }

    if let Some(cats) = aunt_sue.cats
        && !cat_and_tree_op.is_satisfied(cats, 7)
    {
        return false;
    }

    if let Some(samoyeds) = aunt_sue.samoyeds
        && samoyeds != 2
    {
        return false;
    }

    if let Some(pomeranians) = aunt_sue.pomeranians
        && !pomeranian_and_goldfish_op.is_satisfied(pomeranians, 3)
    {
        return false;
    }

    if let Some(akitas) = aunt_sue.akitas
        && akitas != 0
    {
        return false;
    }

    if let Some(vizslas) = aunt_sue.vizslas
        && vizslas != 0
    {
        return false;
    }

    if let Some(goldfish) = aunt_sue.goldfish
        && !pomeranian_and_goldfish_op.is_satisfied(goldfish, 5)
    {
        return false;
    }

    if let Some(trees) = aunt_sue.trees
        && !cat_and_tree_op.is_satisfied(trees, 3)
    {
        return false;
    }

    if let Some(cars) = aunt_sue.cars
        && cars != 2
    {
        return false;
    }

    if let Some(perfumes) = aunt_sue.perfumes
        && perfumes != 1
    {
        return false;
    }

    true
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct AuntSue {
    id: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl
    From<(
        usize,
        Option<usize>,
        Option<usize>,
        Option<usize>,
        Option<usize>,
        Option<usize>,
        Option<usize>,
        Option<usize>,
        Option<usize>,
        Option<usize>,
        Option<usize>,
    )> for AuntSue
{
    fn from(
        (
            id,
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes,
        ): (
            usize,
            Option<usize>,
            Option<usize>,
            Option<usize>,
            Option<usize>,
            Option<usize>,
            Option<usize>,
            Option<usize>,
            Option<usize>,
            Option<usize>,
            Option<usize>,
        ),
    ) -> Self {
        Self {
            id,
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes,
        }
    }
}

mod parse {
    use std::collections::HashMap;

    use super::AuntSue;
    use anyhow::anyhow;
    use itertools::Itertools;

    pub fn parse(input: &str) -> anyhow::Result<Vec<AuntSue>> {
        input.lines().map(parse_line).collect()
    }

    fn parse_line(line: &str) -> anyhow::Result<AuntSue> {
        let (sue, attributes) = line
            .split_once(": ")
            .ok_or(anyhow!("Invalid input: '{}'", line))?;

        let id = sue
            .split_once(" ")
            .ok_or(anyhow!("Invalid input: '{}'", line))?
            .1;

        let id = id.parse::<usize>()?;

        let attributes = attributes.split(", ").collect_vec();

        let attributes = attributes
            .into_iter()
            .map(|attribute| {
                let (key, value) = attribute
                    .split_once(": ")
                    .ok_or(anyhow!("Invalid input: '{}'", line))?;

                let value = value.parse::<usize>()?;

                Ok((key, value))
            })
            .collect::<anyhow::Result<HashMap<_, _>>>()?;

        let children = attributes.get("children").copied();
        let cats = attributes.get("cats").copied();
        let samoyeds = attributes.get("samoyeds").copied();
        let pomeranians = attributes.get("pomeranians").copied();
        let akitas = attributes.get("akitas").copied();
        let vizslas = attributes.get("vizslas").copied();
        let goldfish = attributes.get("goldfish").copied();
        let trees = attributes.get("trees").copied();
        let cars = attributes.get("cars").copied();
        let perfumes = attributes.get("perfumes").copied();

        Ok(AuntSue::from((
            id,
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes,
        )))
    }
}
