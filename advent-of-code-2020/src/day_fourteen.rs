use common::prelude::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, value},
    multi::{count, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::collections::{HashMap, HashSet};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-14.txt");
    let parse_start = SystemTime::now();
    let instructions = parse_instructions(input)?;
    let parse_duration = parse_start.elapsed().unwrap();

    let part_one = part_one(&instructions, parse_duration);
    let part_two = part_two(&instructions, parse_duration);

    Ok((part_one, part_two))
}

fn part_one(instructions: &[Instruction], parse_duration: Duration) -> PartAnswer {
    let start = SystemTime::now();

    let mut current_bitmask = Vec::new();
    let mut memory = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::SetMask(bitmask) => {
                current_bitmask = bitmask.clone();
            }
            Instruction::SetMemory(memory_value) => {
                memory.insert(
                    memory_value.address,
                    apply_mask_to_value(&current_bitmask, memory_value.value),
                );
            }
        }
    }

    let solution: u64 = memory.values().into_iter().sum();

    let elapsed = start.elapsed().unwrap();

    (solution, elapsed + parse_duration).into()
}

fn apply_mask_to_value(mask: &[MaskValue], value: u64) -> u64 {
    let binary_string = integer_to_binary_string(value);

    let zipped = mask.iter().zip(binary_string.chars());

    let mut bits = Vec::new();

    for (mask_value, bit) in zipped {
        match mask_value {
            MaskValue::NoValue => bits.push(bit),
            MaskValue::One => bits.push('1'),
            MaskValue::Zero => bits.push('0'),
        }
    }

    let bit_string = bits.into_iter().collect::<String>();

    u64::from_str_radix(&bit_string, 2).unwrap()
}

fn part_two(instructions: &[Instruction], parse_duration: Duration) -> PartAnswer {
    let start = SystemTime::now();

    let mut current_mask = vec![];
    let mut memory = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::SetMask(mask) => current_mask = mask.clone(),
            Instruction::SetMemory(memory_value) => {
                let addresses = apply_mask_to_address(&current_mask, memory_value.address);

                for address in addresses {
                    memory.insert(address, memory_value.value);
                }
            }
        }
    }

    let solution: u64 = memory.values().into_iter().sum();

    let elapsed = start.elapsed().unwrap();

    (solution, elapsed + parse_duration).into()
}

fn apply_mask_to_address(mask: &[MaskValue], address: u64) -> HashSet<u64> {
    let bitstring_address = integer_to_binary_string(address);

    let mut mask_without_floating_bits = Vec::new();
    let mut floating_bit_locations = Vec::new();

    let zipped = mask.iter().zip(bitstring_address.chars());

    for (index, (mask_value, address_char)) in zipped.enumerate() {
        match mask_value {
            MaskValue::NoValue => {
                floating_bit_locations.push(mask.len() - 1 - index);
                mask_without_floating_bits.push('0');
            }
            MaskValue::Zero => {
                mask_without_floating_bits.push(address_char);
            }
            MaskValue::One => {
                mask_without_floating_bits.push('1');
            }
        }
    }

    let base: String = mask_without_floating_bits.into_iter().collect();
    let base = u64::from_str_radix(&base, 2).unwrap();

    let modifiers = powerset_modifiers(&floating_bit_locations);

    modifiers.into_iter().map(|m| m + base).collect()
}

fn powerset_modifiers(floating_bits: &[usize]) -> HashSet<u64> {
    let powerset_size = 2usize.pow(floating_bits.len() as u32);

    let mut numbers = HashSet::new();

    for i in 0..powerset_size {
        let mut current = 0;

        for (j, floating_bit) in floating_bits.iter().enumerate() {
            if i & (1 << j) != 0 {
                let value = 2u64.pow(*floating_bit as u32);
                current += value;
            }
        }

        numbers.insert(current);
    }

    numbers
}

fn integer_to_binary_string(integer: u64) -> String {
    format!("{:036b}", integer)
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum MaskValue {
    NoValue,
    One,
    Zero,
}

enum Instruction {
    SetMask(Vec<MaskValue>),
    SetMemory(MemoryValue),
}

#[derive(Debug, PartialEq)]
struct MemoryValue {
    address: u64,
    value: u64,
}

fn parse_instructions(i: &str) -> Result<Vec<Instruction>, AdventOfCodeError> {
    let result: IResult<&str, Vec<Instruction>> = instructions(i);

    result
        .map(|(_, instructions)| instructions)
        .map_err(|_| AdventOfCodeError::NomParseError)
}

fn instructions(i: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(tag("\n"), instruction)(i)
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    let bitmask = map(bitmask, Instruction::SetMask);
    let memory = map(memory, Instruction::SetMemory);

    alt((bitmask, memory))(i)
}

fn bitmask(i: &str) -> IResult<&str, Vec<MaskValue>> {
    preceded(tag("mask = "), count(mask_value, 36))(i)
}

fn mask_value(i: &str) -> IResult<&str, MaskValue> {
    alt((empty_mask_value, one_mask_value, zero_mask_value))(i)
}

fn empty_mask_value(i: &str) -> IResult<&str, MaskValue> {
    value(MaskValue::NoValue, tag("X"))(i)
}

fn one_mask_value(i: &str) -> IResult<&str, MaskValue> {
    value(MaskValue::One, tag("1"))(i)
}

fn zero_mask_value(i: &str) -> IResult<&str, MaskValue> {
    value(MaskValue::Zero, tag("0"))(i)
}

fn memory(i: &str) -> IResult<&str, MemoryValue> {
    map(
        tuple((tag("mem"), address, tag(" = "), integer)),
        |(_, address, _, value)| MemoryValue { address, value },
    )(i)
}

fn address(i: &str) -> IResult<&str, u64> {
    delimited(tag("["), integer, tag("]"))(i)
}

fn integer(i: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bitmask_string_to_mask_values(i: &str) -> Vec<MaskValue> {
        let (_, bitmask) = bitmask(i).unwrap();
        bitmask
    }

    #[test]
    fn test_bitmask() {
        assert_eq!(
            bitmask("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            Ok((
                "",
                vec![
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::One,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::Zero,
                    MaskValue::NoValue
                ]
            ))
        );
    }

    #[test]
    fn test_memory() {
        assert_eq!(
            memory("mem[8] = 11"),
            Ok((
                "",
                MemoryValue {
                    address: 8,
                    value: 11
                }
            ))
        );
    }

    #[test]
    fn test_integer_to_binary_string() {
        assert_eq!(
            integer_to_binary_string(11),
            "000000000000000000000000000000001011"
        );
        assert_eq!(
            integer_to_binary_string(73),
            "000000000000000000000000000001001001"
        );
    }

    #[test]
    fn test_apply_mask_to_value() {
        assert_eq!(
            apply_mask_to_value(
                &vec![
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::One,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::NoValue,
                    MaskValue::Zero,
                    MaskValue::NoValue
                ],
                11
            ),
            73
        );
    }

    #[test]
    fn test_apply_mask_to_address_recursive() {
        let mask = bitmask_string_to_mask_values("mask = 000000000000000000000000000000X1001X");

        let addresses = apply_mask_to_address(&mask, 42);

        assert_eq!(addresses, vec![26, 27, 58, 59].into_iter().collect());

        let mask = bitmask_string_to_mask_values("mask = 00000000000000000000000000000000X0XX");

        let addresses = apply_mask_to_address(&mask, 26);

        assert_eq!(
            addresses,
            vec![16, 17, 18, 19, 24, 25, 26, 27].into_iter().collect()
        );
    }

    #[test]
    fn test_powerset_modifiers() {
        let expected = vec![0, 1, 2, 3, 8, 9, 10, 11].into_iter().collect();
        assert_eq!(powerset_modifiers(&vec![0, 1, 3]), expected);
    }
}
