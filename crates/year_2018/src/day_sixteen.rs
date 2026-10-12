use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
    str::FromStr,
};

use anyhow::{anyhow, bail};
use itertools::Itertools;
use log::{debug, info};
use strum::{Display, EnumIter, IntoEnumIterator};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (samples, _) = parse(input)?;

    let mut count = 0;

    for sample in samples {
        let mut valid_op_codes = 0;
        for op_code in OpCode::iter() {
            if sample.is_op_code_valid(op_code) {
                valid_op_codes += 1;
            }
        }
        if valid_op_codes >= 3 {
            count += 1;
        }
    }

    Ok(count)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let (samples, instructions) = parse(input)?;

    let mut deduction_inputs = Vec::new();

    for sample in samples {
        let mut valid_op_codes = HashSet::new();

        for op_code in OpCode::iter() {
            if sample.is_op_code_valid(op_code) {
                valid_op_codes.insert(op_code);
            }
        }

        if valid_op_codes.is_empty() {
            bail!("{sample:?} is not valid");
        }

        deduction_inputs.push((sample, valid_op_codes));
    }

    // A mapping of integer to OpCode
    let mut op_code_map = HashMap::new();
    // The set of OpCode_s that are known
    let mut known_op_codes = HashSet::new();

    while op_code_map.len() != 16 {
        let mut modification_made = false;
        for (sample, valid_op_codes) in deduction_inputs.iter() {
            let op_code = sample.get_op_code();

            let valid_op_codes: HashSet<_> = valid_op_codes
                .difference(&known_op_codes)
                .copied()
                .collect();

            if valid_op_codes.len() == 1 {
                let valid_op_code = valid_op_codes
                    .iter()
                    .next()
                    .copied()
                    .ok_or_else(|| anyhow!("Expected exactly one valid op code for {op_code}"))?;

                info!("{op_code} must be {valid_op_code}");
                match op_code_map.entry(op_code) {
                    Entry::Vacant(entry) => {
                        entry.insert(valid_op_code);
                        known_op_codes.insert(valid_op_code);
                        modification_made = true;
                    }
                    Entry::Occupied(entry) => {
                        let existing_value = *entry.get();

                        if existing_value != valid_op_code {
                            bail!(
                                "Found logical inconsistency - {op_code} must be {valid_op_code} but {op_code} is already mapped to {existing_value}"
                            );
                        }
                    }
                }
            } else {
                debug!("{op_code} can be one of {valid_op_codes:?}");
            }
        }
        if !modification_made {
            bail!("Progress has stalled");
        }
    }

    let mut registers = Registers([0; 4]);

    for [op_code, input_a, input_b, output_c] in instructions {
        let op_code = op_code_map
            .get(&op_code)
            .copied()
            .ok_or_else(|| anyhow!("No op code mapped to {op_code}"))?;

        let instruction = Instruction {
            op_code,
            input_a,
            input_b,
            output_c,
        };

        evaluate(instruction, &mut registers);
    }

    Ok(registers.0[0])
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Registers([usize; 4]);

#[derive(Debug, Copy, Clone)]
struct Sample {
    before: Registers,
    instruction: [usize; 4],
    after: Registers,
}

impl Sample {
    fn get_op_code(&self) -> usize {
        self.instruction[0]
    }

    fn is_op_code_valid(&self, op_code: OpCode) -> bool {
        let instruction = Instruction {
            op_code,
            input_a: self.instruction[1],
            input_b: self.instruction[2],
            output_c: self.instruction[3],
        };
        let mut registers = self.before;

        evaluate(instruction, &mut registers);

        registers == self.after
    }
}

impl FromStr for Sample {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").collect_vec();

        if lines.len() != 3 {
            bail!("Expected 3 lines, found {} - {lines:?}", lines.len());
        }

        let before = lines[0]
            .strip_prefix("Before:")
            .map(str::trim_start)
            .and_then(|s| s.strip_circumfix("[", "]"))
            .ok_or_else(|| anyhow!("Invalid before registers - '{}'", lines[0]))?;
        let before = before.replace(",", "");
        let before = parse_integer_array(&before)?;
        let before = Registers(before);

        let instruction = parse_integer_array(lines[1])?;

        let after = lines[2]
            .strip_prefix("After:")
            .map(str::trim_start)
            .and_then(|s| s.strip_circumfix("[", "]"))
            .ok_or_else(|| anyhow!("Invalid after registers - '{}'", lines[2]))?;
        let after = after.replace(",", "");
        let after = parse_integer_array(&after)?;
        let after = Registers(after);

        Ok(Self {
            before,
            instruction,
            after,
        })
    }
}

struct Instruction {
    op_code: OpCode,
    input_a: usize,
    input_b: usize,
    output_c: usize,
}

fn evaluate(
    Instruction {
        op_code,
        input_a,
        input_b,
        output_c,
    }: Instruction,
    registers: &mut Registers,
) {
    let output = match op_code {
        OpCode::AddRegister => {
            let a = registers.0[input_a];
            let b = registers.0[input_b];
            a + b
        }
        OpCode::AddImmediate => {
            let a = registers.0[input_a];
            a + input_b
        }
        OpCode::MultiplyRegister => {
            let a = registers.0[input_a];
            let b = registers.0[input_b];
            a * b
        }
        OpCode::MultiplyImmediate => {
            let a = registers.0[input_a];
            a * input_b
        }
        OpCode::BitwiseAndRegister => {
            let a = registers.0[input_a];
            let b = registers.0[input_b];
            a & b
        }
        OpCode::BitwiseAndImmediate => {
            let a = registers.0[input_a];
            a & input_b
        }
        OpCode::BitwiseOrRegister => {
            let a = registers.0[input_a];
            let b = registers.0[input_b];
            a | b
        }
        OpCode::BitwiseOrImmediate => {
            let a = registers.0[input_a];
            a | input_b
        }
        OpCode::SetRegister => registers.0[input_a],
        OpCode::SetImmediate => input_a,
        OpCode::GreaterThanImmediateRegister => {
            let b = registers.0[input_b];
            if input_a > b { 1 } else { 0 }
        }
        OpCode::GreaterThanRegisterImmediate => {
            let a = registers.0[input_a];
            if a > input_b { 1 } else { 0 }
        }
        OpCode::GreaterThanRegisterRegister => {
            let a = registers.0[input_a];
            let b = registers.0[input_b];
            if a > b { 1 } else { 0 }
        }
        OpCode::EqualImmediateRegister => {
            let b = registers.0[input_b];
            if input_a == b { 1 } else { 0 }
        }
        OpCode::EqualRegisterImmediate => {
            let a = registers.0[input_a];
            if a == input_b { 1 } else { 0 }
        }
        OpCode::EqualRegisterRegister => {
            let a = registers.0[input_a];
            let b = registers.0[input_b];
            if a == b { 1 } else { 0 }
        }
    };

    registers.0[output_c] = output;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, Display)]
enum OpCode {
    AddRegister,
    AddImmediate,
    MultiplyRegister,
    MultiplyImmediate,
    BitwiseAndRegister,
    BitwiseAndImmediate,
    BitwiseOrRegister,
    BitwiseOrImmediate,
    SetRegister,
    SetImmediate,
    GreaterThanImmediateRegister,
    GreaterThanRegisterImmediate,
    GreaterThanRegisterRegister,
    EqualImmediateRegister,
    EqualRegisterImmediate,
    EqualRegisterRegister,
}

fn parse(s: &str) -> anyhow::Result<(Vec<Sample>, Vec<[usize; 4]>)> {
    let mut samples = Vec::new();
    let mut instructions = Vec::new();

    let (raw_samples, raw_instructions) = s
        .split_once("\n\n\n\n")
        .ok_or_else(|| anyhow!("Expected samples and instructions separated by blank lines"))?;

    for raw_sample in raw_samples.split("\n\n") {
        let sample = Sample::from_str(raw_sample)?;
        debug!(
            "Parsed sample {}: before={:?} instruction={:?} after={:?}",
            samples.len(),
            sample.before.0,
            sample.instruction,
            sample.after.0
        );
        samples.push(sample);
    }

    for raw_instruction in raw_instructions.lines() {
        let instruction = parse_integer_array(raw_instruction)?;
        instructions.push(instruction);
    }

    debug!(
        "Parsed {} samples and {} instructions",
        samples.len(),
        instructions.len()
    );

    Ok((samples, instructions))
}

fn parse_integer_array(s: &str) -> anyhow::Result<[usize; 4]> {
    let tokens = s.split_ascii_whitespace().collect_vec();

    if tokens.len() != 4 {
        bail!("Expected 4 tokens, found {} - '{tokens:?}'", tokens.len());
    }

    let first = tokens[0].parse()?;
    let second = tokens[1].parse()?;
    let third = tokens[2].parse()?;
    let fourth = tokens[3].parse()?;

    Ok([first, second, third, fourth])
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    const REGISTERS: [usize; 4] = [12, 10, 5, 7];

    fn run(op_code: OpCode, input_a: usize, input_b: usize, output_c: usize) -> [usize; 4] {
        let mut registers = Registers(REGISTERS);
        let instruction = Instruction {
            op_code,
            input_a,
            input_b,
            output_c,
        };
        evaluate(instruction, &mut registers);
        registers.0
    }

    #[test]
    fn test_add_register() {
        assert_eq!(run(OpCode::AddRegister, 1, 2, 3), [12, 10, 5, 15]);
    }

    #[test]
    fn test_add_immediate() {
        assert_eq!(run(OpCode::AddImmediate, 1, 2, 3), [12, 10, 5, 12]);
    }

    #[test]
    fn test_multiply_register() {
        assert_eq!(run(OpCode::MultiplyRegister, 1, 2, 3), [12, 10, 5, 50]);
    }

    #[test]
    fn test_multiply_immediate() {
        assert_eq!(run(OpCode::MultiplyImmediate, 1, 2, 3), [12, 10, 5, 20]);
    }

    #[test]
    fn test_bitwise_and_register() {
        assert_eq!(run(OpCode::BitwiseAndRegister, 0, 1, 3), [12, 10, 5, 8]);
    }

    #[test]
    fn test_bitwise_and_immediate() {
        assert_eq!(run(OpCode::BitwiseAndImmediate, 1, 2, 3), [12, 10, 5, 2]);
    }

    #[test]
    fn test_bitwise_or_register() {
        assert_eq!(run(OpCode::BitwiseOrRegister, 1, 2, 3), [12, 10, 5, 15]);
    }

    #[test]
    fn test_bitwise_or_immediate() {
        assert_eq!(run(OpCode::BitwiseOrImmediate, 1, 1, 3), [12, 10, 5, 11]);
    }

    #[test]
    fn test_set_register() {
        assert_eq!(run(OpCode::SetRegister, 1, 2, 3), [12, 10, 5, 10]);
    }

    #[test]
    fn test_set_immediate() {
        assert_eq!(run(OpCode::SetImmediate, 1, 2, 3), [12, 10, 5, 1]);
    }

    #[test]
    fn test_greater_than_immediate_register() {
        assert_eq!(
            run(OpCode::GreaterThanImmediateRegister, 6, 2, 3),
            [12, 10, 5, 1]
        );
        assert_eq!(
            run(OpCode::GreaterThanImmediateRegister, 5, 2, 3),
            [12, 10, 5, 0]
        );
    }

    #[test]
    fn test_greater_than_register_immediate() {
        assert_eq!(
            run(OpCode::GreaterThanRegisterImmediate, 1, 9, 3),
            [12, 10, 5, 1]
        );
        assert_eq!(
            run(OpCode::GreaterThanRegisterImmediate, 1, 10, 3),
            [12, 10, 5, 0]
        );
    }

    #[test]
    fn test_greater_than_register_register() {
        assert_eq!(
            run(OpCode::GreaterThanRegisterRegister, 1, 2, 3),
            [12, 10, 5, 1]
        );
        assert_eq!(
            run(OpCode::GreaterThanRegisterRegister, 2, 1, 3),
            [12, 10, 5, 0]
        );
    }

    #[test]
    fn test_equal_immediate_register() {
        assert_eq!(run(OpCode::EqualImmediateRegister, 5, 2, 3), [12, 10, 5, 1]);
        assert_eq!(run(OpCode::EqualImmediateRegister, 4, 2, 3), [12, 10, 5, 0]);
    }

    #[test]
    fn test_equal_register_immediate() {
        assert_eq!(run(OpCode::EqualRegisterImmediate, 2, 5, 3), [12, 10, 5, 1]);
        assert_eq!(run(OpCode::EqualRegisterImmediate, 2, 4, 3), [12, 10, 5, 0]);
    }

    #[test]
    fn test_equal_register_register() {
        assert_eq!(run(OpCode::EqualRegisterRegister, 1, 1, 3), [12, 10, 5, 1]);
        assert_eq!(run(OpCode::EqualRegisterRegister, 1, 2, 3), [12, 10, 5, 0]);
    }

    #[test]
    fn test_is_op_code_valid() {
        let execution = Sample {
            before: Registers([3, 2, 1, 1]),
            instruction: [9, 2, 1, 2],
            after: Registers([3, 2, 2, 1]),
        };

        let valid: Vec<OpCode> = OpCode::iter()
            .filter(|&op_code| execution.is_op_code_valid(op_code))
            .collect();

        assert_eq!(
            valid,
            vec![
                OpCode::AddImmediate,
                OpCode::MultiplyRegister,
                OpCode::SetImmediate
            ]
        );
    }

    #[test]
    fn test_parse_integer_array() -> anyhow::Result<()> {
        assert_eq!(parse_integer_array("9 2 1 2")?, [9, 2, 1, 2]);
        assert_eq!(parse_integer_array("10 0 15 3")?, [10, 0, 15, 3]);
        Ok(())
    }

    #[test]
    fn test_parse_integer_array_invalid() {
        assert!(parse_integer_array("9 2 1").is_err());
        assert!(parse_integer_array("9 2 1 2 5").is_err());
        assert!(parse_integer_array("9 2 x 2").is_err());
    }

    #[test]
    fn test_sample_from_str() -> anyhow::Result<()> {
        let sample = Sample::from_str("Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]")?;

        assert_eq!(sample.before, Registers([3, 2, 1, 1]));
        assert_eq!(sample.instruction, [9, 2, 1, 2]);
        assert_eq!(sample.after, Registers([3, 2, 2, 1]));
        Ok(())
    }

    #[test]
    fn test_sample_from_str_invalid() {
        assert!(Sample::from_str("Before: [3, 2, 1, 1]\n9 2 1 2").is_err());
        assert!(Sample::from_str("[3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]").is_err());
        assert!(Sample::from_str("Before: [3, 2, 1, 1]\n9 2 1 2\n[3, 2, 2, 1]").is_err());
    }

    #[test]
    fn test_parse_trailing_newline() -> anyhow::Result<()> {
        let input = "Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]\n\n\n\n9 2 1 2\n1 0 3 3\n";
        let (samples, instructions) = parse(input)?;

        assert_eq!(samples.len(), 1);
        assert_eq!(instructions, vec![[9, 2, 1, 2], [1, 0, 3, 3]]);
        Ok(())
    }
}
