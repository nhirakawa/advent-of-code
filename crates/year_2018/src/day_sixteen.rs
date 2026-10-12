use anyhow::anyhow;
use strum::EnumIter;

pub fn part_one(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Registers([usize; 4]);

struct Sample {
    before: Registers,
    instruction: [usize; 4],
    after: Registers,
}

impl Sample {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
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
}
