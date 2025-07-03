use anyhow::bail;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let number_of_presents = input.trim().parse::<usize>()?;

    let mut houses = vec![0; number_of_presents / 10];

    for i in 1..(number_of_presents / 10) {
        for j in (i..(number_of_presents / 10)).step_by(i) {
            houses[j] += i * 10;
        }
    }

    for (i, presents) in houses.iter().enumerate() {
        if *presents >= number_of_presents {
            return Ok(i);
        }
    }

    bail!("No solution found")
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let number_of_presents = input.trim().parse::<usize>()?;

    let mut houses = vec![0; number_of_presents / 11];

    for i in 1..(number_of_presents / 11) {
        for j in (i..(number_of_presents / 11)).step_by(i).take(50) {
            houses[j] += i * 11;
        }
    }

    for (i, presents) in houses.iter().enumerate() {
        if *presents >= number_of_presents {
            return Ok(i);
        }
    }

    bail!("No solution found")
}
