use anyhow::bail;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let mut result = input.to_string();

    for _ in 0..40 {
        result = say(result)?;
    }

    Ok(result.len().to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let mut result = input.to_string();

    for _ in 0..50 {
        result = say(result)?;
    }

    Ok(result.len().to_string())
}

fn say(s: String) -> anyhow::Result<String> {
    if s.is_empty() {
        bail!("Empty string")
    }

    let mut result = String::new();

    let mut current = s.chars().next().unwrap();
    let mut count = 1;

    for c in s.chars().skip(1) {
        if c == current {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(current);

            current = c;
            count = 1;
        }
    }

    result.push_str(&count.to_string());
    result.push(current);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say() {
        assert_eq!(say("1".to_string()).unwrap(), "11");
        assert_eq!(say("11".to_string()).unwrap(), "21");
        assert_eq!(say("21".to_string()).unwrap(), "1211");
        assert_eq!(say("1211".to_string()).unwrap(), "111221");
        assert_eq!(say("111221".to_string()).unwrap(), "312211");
    }
}
