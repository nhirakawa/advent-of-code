use crate::common::parse::{finish, unsigned_number};
use nom::{
    bytes::complete::tag, character::complete::space1, multi::separated_list1, IResult, Parser,
};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let reports = parse(input)?;

    let safe_report_count = reports.into_iter().filter(is_safe_part_one).count();

    Ok(safe_report_count.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let reports = parse(input)?;

    let safe_report_count = reports.into_iter().filter(is_safe_part_two).count();

    Ok(safe_report_count.to_string())
}

fn is_safe_part_one(report: &Report) -> bool {
    let all_increasing = report.windows(2).all(|w| w[0] < w[1]);
    if all_increasing {
        return report.windows(2).all(|w| w[1] - w[0] <= 3);
    }

    let all_decreasing = report.windows(2).all(|w| w[0] > w[1]);
    if all_decreasing {
        return report.windows(2).all(|w| w[0] - w[1] <= 3);
    }

    false
}

fn is_safe_part_two(report: &Report) -> bool {
    if is_safe_part_one(report) {
        return true;
    }

    for idx in 0..report.len() {
        let mut report = report.clone();
        report.remove(idx);

        if is_safe_part_one(&report) {
            return true;
        }
    }

    false
}

type Level = u32;
type Report = Vec<Level>;
type Reports = Vec<Report>;

fn parse(input: &str) -> anyhow::Result<Reports> {
    finish(reports, input)
}

fn reports(i: &str) -> IResult<&str, Reports> {
    separated_list1(tag("\n"), report).parse(i)
}

fn report(i: &str) -> IResult<&str, Report> {
    separated_list1(space1, level).parse(i)
}

fn level(i: &str) -> IResult<&str, Level> {
    unsigned_number.parse(i)
}
