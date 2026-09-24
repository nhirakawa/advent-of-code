use anyhow::anyhow;

pub fn part_one(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("not implemented"))
}
pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("not implemented"))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Component {
    Microchip(String),
    Generator(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TestingFacility {
    elevator_floor: usize,
    floors: [Vec<Component>; 4],
}

