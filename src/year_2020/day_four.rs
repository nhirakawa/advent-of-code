use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_till, take_while_m_n},
    character::complete::{digit1, hex_digit1},
    combinator::{all_consuming, map, success, value},
    multi::{many0, separated_list1},
    sequence::{preceded, separated_pair, terminated},
};
use std::collections::HashMap;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let passports = parse_passports(input)?;

    let answer = passports.len();

    Ok(answer)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let passports = parse_passports(input)?;

    let mut counter: u32 = 0;
    for passport in passports {
        if passport.is_valid() {
            counter += 1;
        }
    }

    Ok(counter)
}

fn parse_passports(input: &str) -> anyhow::Result<Vec<Passport>> {
    let (_, passports) = passports(input).map_err(|e| anyhow::Error::from(e.to_owned()))?;

    Ok(passports)
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum KeyType {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Value {
    Valid,
    Invalid,
}

#[derive(Debug, PartialEq)]
struct KeyValue {
    key_type: KeyType,
    value: Value,
}

impl From<(KeyType, Value)> for KeyValue {
    fn from(kv: (KeyType, Value)) -> KeyValue {
        let (key_type, value) = kv;
        KeyValue { key_type, value }
    }
}

#[derive(Debug, PartialEq)]
struct Passport {
    birth_year: Value,
    issue_year: Value,
    expiration_year: Value,
    height: Value,
    hair_color: Value,
    eye_color: Value,
    passport_id: Value,
}

impl Passport {
    #[allow(clippy::needless_bool, clippy::if_same_then_else)]
    pub fn is_valid(&self) -> bool {
        if self.birth_year == Value::Invalid {
            false
        } else if self.issue_year == Value::Invalid {
            false
        } else if self.expiration_year == Value::Invalid {
            false
        } else if self.height == Value::Invalid {
            false
        } else if self.hair_color == Value::Invalid {
            false
        } else if self.eye_color == Value::Invalid {
            false
        } else if self.passport_id == Value::Invalid {
            false
        } else {
            true
        }
    }
}

fn passports(i: &str) -> IResult<&str, Vec<Passport>> {
    let (remaining, passports) =
        terminated(separated_list1(tag("\n\n"), passport), many0(tag("\n"))).parse(i)?;

    let passports = passports.into_iter().flatten().collect();

    Ok((remaining, passports))
}

fn passport(i: &str) -> IResult<&str, Option<Passport>> {
    let key_values = separated_list1(space_or_newline, key_value);

    map(key_values, to_passport).parse(i)
}

fn to_passport(kvs: Vec<KeyValue>) -> Option<Passport> {
    let mut map = kvs
        .into_iter()
        .map(|kv| (kv.key_type, kv.value))
        .collect::<HashMap<KeyType, Value>>();

    let birth_year = map.remove(&KeyType::BirthYear)?;
    let issue_year = map.remove(&KeyType::IssueYear)?;
    let expiration_year = map.remove(&KeyType::ExpirationYear)?;
    let height = map.remove(&KeyType::Height)?;
    let hair_color = map.remove(&KeyType::HairColor)?;
    let eye_color = map.remove(&KeyType::EyeColor)?;
    let passport_id = map.remove(&KeyType::PassportId)?;

    Some(Passport {
        birth_year,
        issue_year,
        expiration_year,
        height,
        hair_color,
        eye_color,
        passport_id,
    })
}

fn key_value(i: &str) -> IResult<&str, KeyValue> {
    alt((
        birth_year,
        issue_year,
        expiration_year,
        height,
        hair_color,
        eye_color,
        passport_id,
        country_id,
    ))
    .parse(i)
}

fn birth_year(i: &str) -> IResult<&str, KeyValue> {
    let (remaining, value) = unvalidated_value("byr").parse(i)?;

    let value = value
        .parse::<u32>()
        .map(|year| {
            if !(1920..=2002).contains(&year) {
                Value::Invalid
            } else {
                Value::Valid
            }
        })
        .unwrap_or(Value::Invalid);

    Ok((
        remaining,
        KeyValue {
            key_type: KeyType::BirthYear,
            value,
        },
    ))
}

fn issue_year(i: &str) -> IResult<&str, KeyValue> {
    let (remaining, value) = unvalidated_value("iyr").parse(i)?;

    let value = value
        .parse::<u32>()
        .map(|year| {
            if !(2010..=2020).contains(&year) {
                Value::Invalid
            } else {
                Value::Valid
            }
        })
        .unwrap_or(Value::Invalid);

    let key_value = KeyValue {
        key_type: KeyType::IssueYear,
        value,
    };
    Ok((remaining, key_value))
}

fn expiration_year(i: &str) -> IResult<&str, KeyValue> {
    let (remaining, value) = unvalidated_value("eyr").parse(i)?;

    let value = value
        .parse::<u32>()
        .map(|year| {
            if !(2020..=2030).contains(&year) {
                Value::Invalid
            } else {
                Value::Valid
            }
        })
        .unwrap_or(Value::Invalid);

    let key_value = KeyValue {
        key_type: KeyType::ExpirationYear,
        value,
    };

    Ok((remaining, key_value))
}

fn height(i: &str) -> IResult<&str, KeyValue> {
    let (remaining, value) = unvalidated_value("hgt").parse(i)?;

    let centimeters = terminated(digit1, tag("cm"));
    let centimeters = map(centimeters, |s: &str| s.parse::<u32>());
    let centimeters = map(centimeters, |result| {
        result
            .map(|cm| {
                if !(150..=193).contains(&cm) {
                    Value::Invalid
                } else {
                    Value::Valid
                }
            })
            .unwrap_or(Value::Invalid)
    });
    let inches = terminated(digit1, tag("in"));
    let inches = map(inches, |s: &str| s.parse::<u32>());
    let inches = map(inches, |result| {
        result
            .map(|height| {
                if !(59..=76).contains(&height) {
                    Value::Invalid
                } else {
                    Value::Valid
                }
            })
            .unwrap_or(Value::Invalid)
    });

    let (_, kv) = map(
        alt((centimeters, inches, success(Value::Invalid))),
        |value| KeyValue {
            key_type: KeyType::Height,
            value,
        },
    )
    .parse(value)?;

    Ok((remaining, kv))
}

fn hair_color(i: &str) -> IResult<&str, KeyValue> {
    let (remaining, raw) = unvalidated_value("hcl").parse(i)?;

    let value: IResult<&str, Value> =
        all_consuming(value(Value::Valid, preceded(tag("#"), hex_digit1))).parse(raw);
    let value = value.map(|(_, v)| v);
    let value = value.unwrap_or(Value::Invalid);

    let key_value = KeyValue {
        key_type: KeyType::HairColor,
        value,
    };

    Ok((remaining, key_value))
}

fn eye_color(i: &str) -> IResult<&str, KeyValue> {
    let (remaining, raw) = unvalidated_value("ecl").parse(i)?;

    let amb = tag("amb");
    let blu = tag("blu");
    let brn = tag("brn");
    let gry = tag("gry");
    let grn = tag("grn");
    let hzl = tag("hzl");
    let oth = tag("oth");

    let color = alt((amb, blu, brn, gry, grn, hzl, oth));
    let color = value(Value::Valid, color);

    let result: IResult<&str, Value> = all_consuming(color).parse(raw);
    let result = result.map(|(_, v)| v);
    let result = result.unwrap_or(Value::Invalid);

    let key_value = KeyValue {
        key_type: KeyType::EyeColor,
        value: result,
    };

    Ok((remaining, key_value))
}

fn passport_id(i: &str) -> IResult<&str, KeyValue> {
    let (remaining, raw) = unvalidated_value("pid").parse(i)?;

    let digits = take_while_m_n(9, 9, |c: char| c.is_ascii_digit());
    let digits = all_consuming(digits);
    let mut digits = value(Value::Valid, digits);
    let result: IResult<&str, Value> = digits.parse(raw);

    let value = result.map(|(_, v)| v).unwrap_or(Value::Invalid);

    let key_value = KeyValue {
        key_type: KeyType::PassportId,
        value,
    };

    Ok((remaining, key_value))
}

fn country_id(i: &str) -> IResult<&str, KeyValue> {
    let (remaining, value) = value(Value::Valid, unvalidated_value("cid")).parse(i)?;

    let key_value = KeyValue {
        key_type: KeyType::CountryId,
        value,
    };

    Ok((remaining, key_value))
}

fn unvalidated_value(
    key_name: &str,
) -> impl Parser<&str, Output = &str, Error = nom::error::Error<&str>> {
    map(separated_pair(tag(key_name), tag(":"), any), |(_, v)| v)
}

fn any(i: &str) -> IResult<&str, &str> {
    take_till(is_whitespace)(i)
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

fn space_or_newline(i: &str) -> IResult<&str, ()> {
    value((), alt((tag(" "), tag("\n")))).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birth_year() {
        assert_eq!(
            birth_year("byr:1940"),
            Ok((
                "",
                KeyValue {
                    key_type: KeyType::BirthYear,
                    value: Value::Valid
                }
            ))
        );
    }

    #[test]
    fn test_issue_year() {
        assert_eq!(
            issue_year("iyr:2015"),
            Ok((
                "",
                KeyValue {
                    key_type: KeyType::IssueYear,
                    value: Value::Valid
                }
            ))
        );
    }

    #[test]
    fn test_expiration_year() {
        assert_eq!(
            expiration_year("eyr:2029"),
            Ok((
                "",
                KeyValue {
                    key_type: KeyType::ExpirationYear,
                    value: Value::Valid
                }
            ))
        );
    }

    #[test]
    fn test_height() {
        assert_eq!(
            height("hgt:180cm"),
            Ok((
                "",
                KeyValue {
                    key_type: KeyType::Height,
                    value: Value::Valid
                }
            ))
        );

        assert_eq!(
            height("hgt:70in"),
            Ok((
                "",
                KeyValue {
                    key_type: KeyType::Height,
                    value: Value::Valid
                }
            ))
        );

        assert_eq!(
            height("hgt:125"),
            Ok((
                "",
                KeyValue {
                    key_type: KeyType::Height,
                    value: Value::Invalid
                }
            ))
        );
    }

    #[test]
    fn test_hair_color() {
        assert_eq!(
            hair_color("hcl:#00aaff"),
            Ok((
                "",
                KeyValue {
                    key_type: KeyType::HairColor,
                    value: Value::Valid
                }
            ))
        );

        assert_eq!(
            hair_color("hcl:z"),
            Ok((
                "",
                KeyValue {
                    key_type: KeyType::HairColor,
                    value: Value::Invalid
                }
            ))
        );
    }

    #[test]
    fn test_eye_color() {
        assert_eq!(
            eye_color("ecl:amb"),
            Ok((
                "",
                KeyValue {
                    key_type: KeyType::EyeColor,
                    value: Value::Valid
                }
            ))
        );
    }

    #[test]
    fn test_passport_id() {
        assert_eq!(
            passport_id("pid:000000001"),
            Ok((
                "",
                KeyValue {
                    key_type: KeyType::PassportId,
                    value: Value::Valid
                }
            ))
        );
    }

    #[test]
    fn test_country_id() {
        assert_eq!(
            country_id("cid:asdfdsadsf"),
            Ok((
                "",
                KeyValue {
                    key_type: KeyType::CountryId,
                    value: Value::Valid
                }
            ))
        );
    }

    #[test]
    fn test_passport() {
        let actual = passport(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm",
        );

        let expected = Passport {
            birth_year: Value::Valid,
            issue_year: Value::Valid,
            expiration_year: Value::Valid,
            height: Value::Valid,
            hair_color: Value::Valid,
            eye_color: Value::Valid,
            passport_id: Value::Valid,
        };

        assert_eq!(actual, Ok(("", expected.into())));

        let actual =
            passport("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929");

        assert_eq!(actual, Ok(("", None)));

        // hcl:z byr:2026\npid:61805448\nhgt:125 iyr:2025
        let actual = passport("hcl:z byr:2026\npid:61805448\nhgt:125 iyr:2025");
        assert_eq!(actual, Ok(("", None)));
    }

    #[test]
    fn test_passports() {
        let actual = passports(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929",
        );

        let first = Passport {
            birth_year: Value::Valid,
            issue_year: Value::Valid,
            expiration_year: Value::Valid,
            height: Value::Valid,
            hair_color: Value::Valid,
            eye_color: Value::Valid,
            passport_id: Value::Valid,
        };

        let expected = vec![first];

        assert_eq!(actual, Ok(("", expected)));
    }

    #[test]
    fn test_separated_list_newlines() {
        let parser: IResult<&str, Vec<&str>> =
            separated_list1(tag("\n\n"), tag("fdsa")).parse("fdsa\n\nfdsa\n");

        let (remaining, parsed) = parser.unwrap();
        assert_eq!(remaining, "\n");
        assert_eq!(parsed, vec!["fdsa", "fdsa"]);
    }
}
