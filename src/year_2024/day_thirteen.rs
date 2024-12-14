use anyhow::bail;
use log::debug;
use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

use crate::common::parse::{finish, unsigned_number};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let claw_machines = parse_claw_machines(input)?;

    let mut total_tokens = 0;

    for claw_machine in claw_machines {
        let ButtonA { x: x1, y: x2 } = claw_machine.button_a;
        let ButtonB { x: y1, y: y2 } = claw_machine.button_b;
        let (z1, z2) = claw_machine.prize;

        if let Ok((a, b)) = evaluate((x1, y1, z1), (x2, y2, z2)) {
            total_tokens += (3 * a) + b;
        }
    }

    Ok(total_tokens.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let claw_machines = parse_claw_machines(input)?;

    let mut total_tokens = 0;

    for claw_machine in claw_machines {
        let ButtonA { x: x1, y: x2 } = claw_machine.button_a;
        let ButtonB { x: y1, y: y2 } = claw_machine.button_b;
        let (z1, z2) = claw_machine.prize;

        let (z1, z2) = (z1 + 10000000000000, z2 + 10000000000000);

        if let Ok((a, b)) = evaluate((x1, y1, z1), (x2, y2, z2)) {
            total_tokens += (3 * a) + b;
        }
    }

    Ok(total_tokens.to_string())
}

fn evaluate(
    (x1, y1, z1): (Int, Int, Int),
    (x2, y2, z2): (Int, Int, Int),
) -> anyhow::Result<(Int, Int)> {
    debug!("Variables: x1={x1}, y1={y1}, z1={z1}, x2={x2}, y2={y2}, z2={z2}");

    let b_numerator = (x1 * z2) - (x2 * z1);
    let b_denominator = (x1 * y2) - (x2 * y1);

    debug!("Evaluating {b_numerator}/{b_denominator}");

    let b_has_integer_solution = ((x1 * z2) - (x2 * z1)) % ((x1 * y2) - (x2 * y1)) == 0;
    if !b_has_integer_solution {
        bail!("Could not find integer solution for b");
    }

    let b = ((x1 * z2) - (x2 * z1)) / ((x1 * y2) - (x2 * y1));

    let a_has_integer_solution = (z1 - (y1 * b)) % x1 == 0;

    if !a_has_integer_solution {
        bail!("Could not find integer solution for a");
    }

    let a = (z1 - (y1 * b)) / x1;

    Ok((a, b))
}

type Int = i128;

struct ClawMachine {
    button_a: ButtonA,
    button_b: ButtonB,
    prize: (Int, Int),
}

struct ButtonA {
    x: Int,
    y: Int,
}

struct ButtonB {
    x: Int,
    y: Int,
}

type ClawMachines = Vec<ClawMachine>;

fn parse_claw_machines(i: &str) -> anyhow::Result<ClawMachines> {
    finish(claw_machines)(i)
        .map(|(_, claw_machines)| claw_machines)
        .map_err(|e| e.to_owned().into())
}

fn claw_machines(i: &str) -> IResult<&str, ClawMachines> {
    separated_list1(tag("\n\n"), claw_machine)(i)
}

fn claw_machine(i: &str) -> IResult<&str, ClawMachine> {
    map(
        tuple((button_a, tag("\n"), button_b, tag("\n"), prize)),
        |(button_a, _, button_b, _, prize)| ClawMachine {
            button_a,
            button_b,
            prize,
        },
    )(i)
}

fn button_a(i: &str) -> IResult<&str, ButtonA> {
    let x = preceded(tag("X+"), unsigned_number);
    let y = preceded(tag("Y+"), unsigned_number);
    preceded(
        tag("Button A: "),
        map(separated_pair(x, tag(", "), y), |(x, y)| ButtonA { x, y }),
    )(i)
}

fn button_b(i: &str) -> IResult<&str, ButtonB> {
    let x = preceded(tag("X+"), unsigned_number);
    let y = preceded(tag("Y+"), unsigned_number);
    preceded(
        tag("Button B: "),
        map(separated_pair(x, tag(", "), y), |(x, y)| ButtonB { x, y }),
    )(i)
}

fn prize(i: &str) -> IResult<&str, (Int, Int)> {
    let x = preceded(tag("X="), unsigned_number);
    let y = preceded(tag("Y="), unsigned_number);
    preceded(tag("Prize: "), separated_pair(x, tag(", "), y))(i)
}
