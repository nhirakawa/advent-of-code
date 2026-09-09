use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::bail;

use crate::year_2016::day_ten::model::Output;
use crate::year_2016::day_ten::model::Target;
use crate::year_2016::day_ten::model::{Bot, Input};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (inputs, bots) = parse::parse_context(input)?;

    let mut bot_system = BotSystem::new(bots)?;

    for Input { value, output } in inputs {
        bot_system.push_value(output, value)?;
    }

    bot_system
        .find_bot_with_values(61, 17)
        .ok_or(anyhow!("No answer found"))
}
pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let (inputs, bots) = parse::parse_context(input)?;

    let mut bot_system = BotSystem::new(bots)?;

    for Input { value, output } in inputs {
        bot_system.push_value(output, value)?;
    }

    let output_zero = bot_system
        .get_output(0)
        .ok_or(anyhow!("No output value for 0"))? as usize;
    let output_one = bot_system
        .get_output(1)
        .ok_or(anyhow!("No output value for 1"))? as usize;
    let output_two = bot_system
        .get_output(2)
        .ok_or(anyhow!("No output value for 2"))? as usize;

    let solution = output_zero * output_one * output_two;

    Ok(solution)
}

struct BotSystem {
    ids: Vec<u8>,
    bots: HashMap<u8, Bot>,
    bot_values: HashMap<u8, BotValues>,
    outputs: HashMap<u8, u8>,
}

impl BotSystem {
    fn new(input: Vec<Bot>) -> anyhow::Result<BotSystem> {
        let mut ids = Vec::new();
        let mut bots = HashMap::new();
        let mut bot_values = HashMap::new();
        let outputs = HashMap::new();

        for bot in input {
            let id = bot.id;
            if ids.contains(&id) {
                bail!("Found duplicate id - {id}");
            }

            ids.push(id);
            bots.insert(id, bot);
            bot_values.insert(id, BotValues::new(id));
        }

        Ok(BotSystem {
            ids,
            bots,
            bot_values,
            outputs,
        })
    }

    fn push_value(&mut self, id: u8, value: u8) -> anyhow::Result<()> {
        let bot_values = self
            .bot_values
            .get_mut(&id)
            .ok_or(anyhow!("No BotValues found for id {id}"))?;

        bot_values.push_value(value)?;

        if let Some((first, second)) = bot_values.values() {
            let low = first.min(second);
            let high = first.max(second);

            let bot = self
                .bots
                .get(&id)
                .copied()
                .ok_or(anyhow!("No Bot found for id {id}"))?;

            match bot.low {
                Target::Bot(low_id) => {
                    self.push_value(low_id, low)?;
                }
                Target::Output(Output { id: low_id }) => {
                    self.outputs.insert(low_id, low);
                }
            };

            match bot.high {
                Target::Bot(high_id) => {
                    self.push_value(high_id, high)?;
                }
                Target::Output(Output { id: low_id }) => {
                    self.outputs.insert(low_id, high);
                }
            };
        }

        Ok(())
    }

    fn find_bot_with_values(&self, first: u8, second: u8) -> Option<u8> {
        for id in &self.ids {
            if let Some(bot_values) = self.bot_values.get(id)
                && bot_values.has_values(first, second)
            {
                return Some(*id);
            }
        }

        None
    }

    fn get_output(&self, output: u8) -> Option<u8> {
        self.outputs.get(&output).copied()
    }
}

#[allow(unused)]
struct BotValues {
    id: u8,
    first: Option<u8>,
    second: Option<u8>,
}

impl BotValues {
    fn new(id: u8) -> BotValues {
        BotValues {
            id,
            first: None,
            second: None,
        }
    }

    fn push_value(&mut self, value: u8) -> anyhow::Result<()> {
        match (self.first, self.second) {
            (None, _) => {
                self.first = Some(value);
                Ok(())
            }
            (Some(first), _) if first == value => Ok(()),
            (Some(_), None) => {
                self.second = Some(value);
                Ok(())
            }
            (Some(_), Some(second)) if second == value => Ok(()),
            (Some(first), Some(second)) => {
                bail!("self.first({first}) and self.second({second}) already have values");
            }
        }
    }

    fn values(&self) -> Option<(u8, u8)> {
        match (self.first, self.second) {
            (Some(first), Some(second)) => Some((first, second)),
            _ => None,
        }
    }

    fn has_values(&self, first: u8, second: u8) -> bool {
        match (self.first, self.second) {
            (Some(self_first), Some(self_second)) => {
                (self_first == first && self_second == second)
                    || (self_first == second && self_second == first)
            }
            _ => false,
        }
    }
}

mod model {
    #[derive(Debug)]
    pub struct Input {
        pub value: u8,
        pub output: u8,
    }

    impl From<(u8, u8)> for Input {
        fn from((value, output): (u8, u8)) -> Self {
            Self { value, output }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Bot {
        pub id: u8,
        pub low: Target,
        pub high: Target,
    }

    impl From<(u8, Target, Target)> for Bot {
        fn from((id, low, high): (u8, Target, Target)) -> Self {
            Self { id, low, high }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Output {
        pub id: u8,
    }

    impl From<u8> for Output {
        fn from(id: u8) -> Self {
            Self { id }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Target {
        Bot(u8),
        Output(Output),
    }

    pub enum Instruction {
        Input(Input),
        Bot(Bot),
    }
}

mod parse {
    use nom::{
        IResult, Parser,
        branch::alt,
        bytes::complete::tag,
        character::complete::u8,
        combinator::{into, map},
        multi::separated_list1,
        sequence::{preceded, separated_pair},
    };

    use crate::{
        common::parse::finish,
        year_2016::day_ten::model::{Bot, Input, Instruction, Output, Target},
    };

    pub fn parse_context(i: &str) -> anyhow::Result<(Vec<Input>, Vec<Bot>)> {
        let instructions = finish(instructions, i)?;

        let mut inputs = Vec::new();
        let mut bots = Vec::new();

        for instruction in instructions {
            match instruction {
                Instruction::Input(input) => inputs.push(input),
                Instruction::Bot(bot) => bots.push(bot),
            };
        }

        Ok((inputs, bots))
    }

    fn instructions(i: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(tag("\n"), instruction).parse(i)
    }

    fn instruction(i: &str) -> IResult<&str, Instruction> {
        alt((
            map(input_instruction, Instruction::Input),
            map(bot_instruction, Instruction::Bot),
        ))
        .parse(i)
    }

    fn input_instruction(i: &str) -> IResult<&str, Input> {
        into(separated_pair(input_value, tag(" goes to "), bot_id)).parse(i)
    }

    fn bot_instruction(i: &str) -> IResult<&str, Bot> {
        into(map(
            (
                bot_id,
                tag(" gives low to "),
                target,
                tag(" and high to "),
                target,
            ),
            |(id, _, low, _, high)| (id, low, high),
        ))
        .parse(i)
    }

    fn target(i: &str) -> IResult<&str, Target> {
        alt((map(bot_id, Target::Bot), map(output, Target::Output))).parse(i)
    }

    fn bot_id(i: &str) -> IResult<&str, u8> {
        preceded(tag("bot "), u8).parse(i)
    }

    fn input_value(i: &str) -> IResult<&str, u8> {
        preceded(tag("value "), u8).parse(i)
    }

    fn output(i: &str) -> IResult<&str, Output> {
        map(preceded(tag("output "), u8), Output::from).parse(i)
    }
}
