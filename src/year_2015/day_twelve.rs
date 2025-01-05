use anyhow::bail;
use serde_json::Value;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let value: Value = serde_json::from_str(input)?;
    sum_value(&value, IgnoreKey::None).map(|sum| sum.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let value: Value = serde_json::from_str(input)?;
    sum_value(&value, IgnoreKey::Red).map(|sum| sum.to_string())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum IgnoreKey {
    None,
    Red,
}

fn sum_value(value: &Value, ignore_key: IgnoreKey) -> anyhow::Result<i64> {
    match value {
        Value::Null => Ok(0),
        Value::Bool(_) => Ok(0),
        Value::Number(number) => {
            if let Some(number) = number.as_i64() {
                Ok(number)
            } else {
                bail!("Invalid number {}", number)
            }
        }
        Value::String(_) => Ok(0),
        Value::Array(vec) => {
            let mut sum = 0;
            for value in vec {
                sum += sum_value(value, ignore_key)?;
            }
            Ok(sum)
        }
        Value::Object(map) => {
            if ignore_key == IgnoreKey::Red {
                for (_, value) in map.iter() {
                    if let Value::String(string) = value {
                        if string == "red" {
                            return Ok(0);
                        }
                    }
                }
            }

            let mut sum = 0;
            for (_, value) in map.iter() {
                sum += sum_value(value, ignore_key)?;
            }
            Ok(sum)
        }
    }
}
