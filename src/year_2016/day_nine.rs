use anyhow::bail;
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let chars = input.chars().collect_vec();
    get_decompressed_length(&chars, Version::One)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let chars = input.chars().collect_vec();
    get_decompressed_length(&chars, Version::Two)
}

fn get_decompressed_length(chars: &[char], version: Version) -> anyhow::Result<usize> {
    let mut size = 0;

    let mut index = 0;

    while index < chars.len() {
        let c = chars[index];

        if c == '(' {
            let closing_paren_index = index_of(&chars, index, ')')?;

            let (length, count) = extract_length_and_count(&chars[index..=closing_paren_index])?;

            index = closing_paren_index + 1;

            let decompressed_length = match version {
                Version::One => length * count,
                Version::Two => {
                    let single_eval_length =
                        get_decompressed_length(&chars[index..index + length], version)?;
                    single_eval_length * count
                }
            };
            size += decompressed_length;
            index += length;
        } else {
            size += 1;
            index += 1;
        }
    }

    Ok(size)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Version {
    One,
    Two,
}

fn extract_length_and_count(chars: &[char]) -> anyhow::Result<(usize, usize)> {
    if chars.is_empty() {
        bail!("Invalid chars - empty");
    }

    if chars[0] != '(' {
        bail!("Invalid chars - no '(' prefix");
    }

    if chars[chars.len() - 1] != ')' {
        bail!("Invalid chars - no ')' suffix");
    }

    let index_of_x = index_of(chars, 0, 'x')?;

    let length = chars[1..index_of_x]
        .into_iter()
        .collect::<String>()
        .parse::<usize>()?;

    let count = chars[(index_of_x + 1)..(chars.len() - 1)]
        .into_iter()
        .collect::<String>()
        .parse::<usize>()?;

    Ok((length, count))
}

fn index_of(chars: &[char], from: usize, find: char) -> anyhow::Result<usize> {
    for i in from..chars.len() {
        if chars[i] == find {
            return Ok(i);
        }
    }

    bail!("Could not find '{find}'")
}
