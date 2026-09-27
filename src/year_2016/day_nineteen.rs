use anyhow::anyhow;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let number_of_elves = input.parse::<usize>()?;
    let leading_one_index = number_of_elves
        .highest_one()
        .ok_or(anyhow!("{number_of_elves:0b} does not have a leading one"))?;
    let remainder = number_of_elves & !(1 << leading_one_index);
    Ok((remainder << 1) | 1)
}
pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}
