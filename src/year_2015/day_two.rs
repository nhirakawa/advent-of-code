use nom::{bytes::complete::tag, combinator::map, multi::separated_list1, IResult, Parser};

use crate::common::parse::{finish, unsigned_number};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let dimensions = parse(input)?;

    let mut sum = 0;

    for (length, width, height) in dimensions {
        let lw = length * width;
        let wh = width * height;
        let hl = height * length;

        let smallest_side = lw.min(wh).min(hl);

        sum += 2 * lw + 2 * wh + 2 * hl + smallest_side;
    }

    Ok(sum.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let dimensions = parse(input)?;

    let mut sum = 0;

    for (length, width, height) in dimensions {
        let lw_perimeter = 2 * length + 2 * width;
        let wh_perimeter = 2 * width + 2 * height;
        let hl_perimeter = 2 * height + 2 * length;

        let smallest_perimeter = lw_perimeter.min(wh_perimeter).min(hl_perimeter);

        let volume = length * width * height;

        sum += smallest_perimeter + volume;
    }

    Ok(sum.to_string())
}

type Dimension = (u32, u32, u32);
type Dimensions = Vec<Dimension>;

fn parse(i: &str) -> anyhow::Result<Dimensions> {
    finish(dimensions, i)
}

fn dimensions(i: &str) -> IResult<&str, Dimensions> {
    separated_list1(tag("\n"), dimension).parse(i)
}

fn dimension(i: &str) -> IResult<&str, Dimension> {
    map(
        (
            unsigned_number,
            tag("x"),
            unsigned_number,
            tag("x"),
            unsigned_number,
        ),
        |(length, _, width, _, height)| (length, width, height),
    )
    .parse(i)
}
