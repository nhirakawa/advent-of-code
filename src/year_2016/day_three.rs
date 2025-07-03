use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let integers = into_integers(input);

    let triangles = integers
        .tuples()
        .filter(|(a, b, c)| *a + *b > *c && *a + *c > *b && *b + *c > *a)
        .count();

    Ok(triangles)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let integers = into_integers(input);

    let triangles = integers
        .tuples()
        .flat_map(|(a, b, c, d, e, f, g, h, i)| [(a, d, g), (b, e, h), (c, f, i)])
        .filter(|(a, b, c)| *a + *b > *c && *a + *c > *b && *b + *c > *a)
        .count();

    Ok(triangles)
}

fn into_integers(input: &str) -> impl Iterator<Item = u32> + '_ {
    input
        .trim()
        .lines()
        .flat_map(|line| line.split_whitespace().filter_map(|s| s.parse().ok()))
}
