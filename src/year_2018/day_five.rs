use anyhow::anyhow;
use regex::Regex;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let polymer = input.trim();
    let regex = build_regex();
    Ok(react_fully(polymer, &regex).len().to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let polymer = input.trim();
    let regex = build_regex();
    let shortest_polymer = ('a'..='z')
        .map(|c| without(polymer, c))
        .map(|p| react_fully(&p, &regex))
        .min_by_key(|p| p.len())
        .ok_or(anyhow!("Could not find shortest polymer"))?;
    Ok(shortest_polymer.len().to_string())
}

fn without(polymer: &str, c: char) -> String {
    let replace_string = format!("({}|{})", c.to_uppercase(), c);
    let regex = Regex::new(&replace_string).unwrap();
    regex.replace_all(polymer, "").into_owned()
}

fn react_fully(polymer: &str, regex: &Regex) -> String {
    let mut before = polymer.to_string();
    loop {
        let after = react(&before, regex);
        if before == after {
            break;
        }
        before = after;
    }
    before
}

fn react(polymer: &str, regex: &Regex) -> String {
    regex.replace_all(polymer, "").into_owned()
}

fn build_regex() -> Regex {
    let mut regex_parts = Vec::new();

    for c in 'a'..='z' {
        let regex_for_pair = format!("({}{}|{}{})", c, c.to_uppercase(), c.to_uppercase(), c);
        regex_parts.push(regex_for_pair);
    }
    let all_pairs = regex_parts.join("|");
    Regex::new(&all_pairs).ok().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_react() {
        let regex = build_regex();
        let polymer = "aA";
        assert_eq!(react(polymer, &regex), "".to_string());
        let polymer = "bAaBc";
        assert_eq!(react(polymer, &regex), "bBc".to_string());
    }

    #[test]
    fn test_react_fully() {
        let regex = build_regex();
        let polymer = "BaAbc";
        assert_eq!(react_fully(polymer, &regex), "c".to_string());
    }

    #[test]
    fn test_without() {
        let str = "aAbzZ";
        assert_eq!(without(str, 'z'), "aAb");
    }
}
