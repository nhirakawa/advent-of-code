use crate::assembunny::{ArgumentMode, Register, from_program};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut interpreter = from_program(input, ArgumentMode::Lenient)?;
    interpreter.write_to_register(7, Register::A);
    interpreter.run()?;
    Ok(interpreter.read_from_register(Register::A))
}
pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let mut interpreter = from_program(input, ArgumentMode::Lenient)?;
    interpreter.write_to_register(12, Register::A);
    interpreter.run()?;
    Ok(interpreter.read_from_register(Register::A))
}
