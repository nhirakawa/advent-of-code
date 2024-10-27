use anyhow::bail;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let expenses = parse_expenses(input)?;

    for (outer_index, outer) in expenses.iter().enumerate() {
        for (inner_index, inner) in expenses.iter().enumerate() {
            if inner_index != outer_index && outer + inner == 2020 {
                return Ok((outer * inner).to_string());
            }
        }
    }

    bail!("No answer found")
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let expenses = parse_expenses(input)?;
    for (first_index, first) in expenses.iter().enumerate() {
        for (second_index, second) in expenses.iter().enumerate() {
            for (third_index, third) in expenses.iter().enumerate() {
                if first_index != second_index
                    && second_index != third_index
                    && first + second + third == 2020
                {
                    return Ok((first * second * third).to_string());
                }
            }
        }
    }

    bail!("No answer found")
}

fn parse_expenses(input: &str) -> anyhow::Result<Vec<u32>> {
    input
        .split('\n')
        .filter(|s| s != &"")
        .map(|s| s.parse::<u32>().map_err(anyhow::Error::from))
        .collect::<anyhow::Result<Vec<u32>>>()
}
