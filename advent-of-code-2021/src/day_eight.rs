use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Add,
};

use common::prelude::*;
use log::warn;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, into, map, value},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-8.txt");
    let all_segments = parse_segments(input);

    let part_one = part_one(&all_segments);
    let part_two = part_two(&all_segments);

    Ok((part_one, part_two))
}

fn part_one(segments: &[SegmentOutput]) -> PartAnswer {
    let start = SystemTime::now();
    let mut count = 0;

    for segment_output in segments {
        for output in &segment_output.output {
            count += match output.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            };
        }
    }

    PartAnswer::new(count, start.elapsed().unwrap())
}

fn part_two(segments: &[SegmentOutput]) -> PartAnswer {
    let start = SystemTime::now();

    let mut sum = 0;
    for segment in segments {
        sum += decode(segment);
    }

    PartAnswer::new(sum, start.elapsed().unwrap())
}

fn decode(segment_value: &SegmentOutput) -> usize {
    // figure out B,F,E first

    let mut counts_by_signal = HashMap::new();

    for segment_value in &segment_value.unique_signals {
        for signal in &segment_value.signals {
            if !counts_by_signal.contains_key(signal) {
                counts_by_signal.insert(*signal, 0);
            }

            if let Some(count) = counts_by_signal.get_mut(signal) {
                *count += 1;
            }
        }
    }

    let mut signal_to_real_signals = HashMap::new();

    for (signal, count) in counts_by_signal.iter() {
        if let Some(real_signal) = match *count {
            6 => Some(Signal::B),
            4 => Some(Signal::E),
            9 => Some(Signal::F),
            _ => None,
        } {
            signal_to_real_signals.insert(*signal, real_signal);
        }
    }

    // since we have F, then figure out C

    for segment_value in &segment_value.unique_signals {
        if segment_value.len() == 2 {
            for signal in &segment_value.signals {
                if !signal_to_real_signals.contains_key(&signal) {
                    signal_to_real_signals.insert(*signal, Signal::C);
                }
            }
        }
    }

    // when we have C and F, then we have A

    for segment_value in &segment_value.unique_signals {
        if segment_value.len() == 3 {
            for signal in &segment_value.signals {
                if !signal_to_real_signals.contains_key(&signal) {
                    signal_to_real_signals.insert(*signal, Signal::A);
                }
            }
        }
    }

    for segment_value in &segment_value.unique_signals {
        if segment_value.len() == 4 {
            for signal in &segment_value.signals {
                if !signal_to_real_signals.contains_key(&signal) {
                    signal_to_real_signals.insert(*signal, Signal::D);
                }
            }
        }
    }

    for segment_value in &segment_value.unique_signals {
        if segment_value.len() == 7 {
            for signal in &segment_value.signals {
                if !signal_to_real_signals.contains_key(&signal) {
                    signal_to_real_signals.insert(*signal, Signal::G);
                }
            }
        }
    }

    let numbers_by_segments: HashMap<String, usize> = vec![
        ("abcefg".into(), 0),
        ("cf".into(), 1),
        ("acdeg".into(), 2),
        ("acdfg".into(), 3),
        ("bcdf".into(), 4),
        ("abdfg".into(), 5),
        ("abdefg".into(), 6),
        ("acf".into(), 7),
        ("abcdefg".into(), 8),
        ("abcdfg".into(), 9),
    ]
    .into_iter()
    .collect();

    let mut decoded_number = 0;

    for (index, segment_value) in segment_value.output.iter().enumerate() {
        let mut new_signal = segment_value
            .signals
            .iter()
            .map(|signal| signal_to_real_signals[signal].clone().to_string())
            .collect::<Vec<String>>();
        new_signal.sort();
        let new_signal = new_signal.join("");

        if let Some(value) = numbers_by_segments.get(&new_signal) {
            let multiplier = 10usize.pow((4 - index - 1) as u32);
            let value = value * multiplier;
            decoded_number += value;
        } else {
            warn!("no value found for {}", new_signal);
        }
    }

    decoded_number
}

#[derive(Debug, PartialEq, Clone)]
struct SegmentOutput {
    unique_signals: Vec<SegmentValue>,
    output: Vec<SegmentValue>,
}

impl SegmentOutput {
    fn new(unique_signals: Vec<SegmentValue>, output: Vec<SegmentValue>) -> Self {
        Self {
            unique_signals,
            output,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct SegmentValue {
    signals: HashSet<Signal>,
}

impl SegmentValue {
    fn len(&self) -> usize {
        self.signals.len()
    }
}

impl Add<&SegmentValue> for SegmentValue {
    type Output = SegmentValue;

    fn add(self, rhs: &SegmentValue) -> Self::Output {
        let combined_signals: Vec<Signal> = self
            .signals
            .union(&rhs.signals)
            .map(|signal| signal.clone())
            .collect();

        SegmentValue::from(combined_signals)
    }
}

impl Display for SegmentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sorted: Vec<Signal> = self.signals.iter().map(|c| c.clone()).collect();
        sorted.sort();
        for signal in sorted {
            write!(f, "{}", signal.to_string())?;
        }

        Ok(())
    }
}

impl From<Vec<Signal>> for SegmentValue {
    fn from(signals: Vec<Signal>) -> Self {
        let signals = signals.into_iter().collect();
        Self { signals }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
enum Signal {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl ToString for Signal {
    fn to_string(&self) -> String {
        match &self {
            Signal::A => "a",
            Signal::B => "b",
            Signal::C => "c",
            Signal::D => "d",
            Signal::E => "e",
            Signal::F => "f",
            Signal::G => "g",
        }
        .to_string()
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let first = self.to_string();
        let second = other.to_string();

        first.cmp(&second)
    }
}

// impl Display for Signal {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let string = match &self {
//             Signal::A => "a",
//             Signal::B => "b",
//             Signal::C => "c",
//             Signal::D => "d",
//             Signal::E => "e",
//             Signal::F => "f",
//             Signal::G => "g",
//         };

//         write!(f, "{}", string)
//     }
// }

fn parse_segments(i: &str) -> Vec<SegmentOutput> {
    all_consuming(terminated(all_segments, tag("\n")))(i)
        .unwrap()
        .1
}

fn all_segments(i: &str) -> IResult<&str, Vec<SegmentOutput>> {
    separated_list1(tag("\n"), combined_segments)(i)
}

fn combined_segments(i: &str) -> IResult<&str, SegmentOutput> {
    map(
        separated_pair(segments, tag(" | "), segments),
        |(unique_signals, output)| SegmentOutput::new(unique_signals, output),
    )(i)
}

fn segments(i: &str) -> IResult<&str, Vec<SegmentValue>> {
    separated_list1(tag(" "), segment)(i)
}

fn segment(i: &str) -> IResult<&str, SegmentValue> {
    into(many1(signal))(i)
}

fn signal(i: &str) -> IResult<&str, Signal> {
    alt((
        value(Signal::A, tag("a")),
        value(Signal::B, tag("b")),
        value(Signal::C, tag("c")),
        value(Signal::D, tag("d")),
        value(Signal::E, tag("e")),
        value(Signal::F, tag("f")),
        value(Signal::G, tag("g")),
    ))(i)
}
