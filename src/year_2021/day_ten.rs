use std::collections::HashMap;
use std::time::SystemTime;
use crate::common::answer::*;
use log::debug;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("input/day-10.txt");
    let part_one = part_one(input);
    let part_two = part_two(input);

    Ok((part_one, part_two))
}

fn part_one(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let mut total_score = 0;
    for line in input.split('\n') {
        let parse_result = parse_line(line);
        let score = match parse_result {
            LineResult::Corrupted(c) => match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            },
            _ => 0,
        };

        if score != 0 {
            debug!("{:?}, score {}", parse_result, score);
        }

        total_score += score;
    }

    PartAnswer::new(total_score, start.elapsed().unwrap())
}

fn part_two(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let mut all_scores = Vec::new();

    for line in input.split('\n') {
        let parse_result = parse_line(line);

        let line_score: i64 = match &parse_result {
            LineResult::Autocompleted(chars) => {
                let mut autocompletion_score = 0;
                for c in chars.iter() {
                    let character_score = match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => 0,
                    };

                    autocompletion_score *= 5;
                    autocompletion_score += character_score;
                }

                debug!("{:?}, score {}", parse_result, autocompletion_score);

                autocompletion_score
            }
            _ => 0,
        };

        if line_score > 0 {
            all_scores.push(line_score);
        }
    }

    all_scores.sort_unstable();

    debug!("scores {:?}", all_scores);

    let middle_score_index = all_scores.len() / 2;
    debug!(
        "{} scores, middle index {}",
        all_scores.len(),
        middle_score_index
    );

    let middle_score = all_scores[middle_score_index];

    PartAnswer::new(middle_score, start.elapsed().unwrap())
}

fn parse_line(input: &str) -> LineResult {
    let closing_for_opening: HashMap<char, char> = [('[', ']'), ('{', '}'), ('<', '>'), ('(', ')')]
        .iter()
        .copied()
        .collect();

    let mut stack = Vec::new();
    for c in input.chars() {
        match c {
            '[' | '(' | '{' | '<' => {
                stack.push(c);
            }
            ']' | ')' | '}' | '>' => {
                if let Some(opener) = stack.pop() {
                    let closer = closing_for_opening[&opener];
                    if closer != c {
                        return LineResult::Corrupted(c);
                    }
                }
            }
            _ => {}
        }
    }

    if stack.is_empty() {
        return LineResult::Valid;
    }

    let mut autocompleted = Vec::new();
    while !stack.is_empty() {
        let uncompleted_opener = stack.pop().unwrap();
        let autocompleted_closer = closing_for_opening[&uncompleted_opener];
        autocompleted.push(autocompleted_closer);
    }

    LineResult::Autocompleted(autocompleted)
}

#[derive(Debug, PartialEq)]
enum LineResult {
    Valid,
    Autocompleted(Vec<char>),
    Corrupted(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corrupted_chunk() {
        assert_eq!(parse_line("(]"), LineResult::Corrupted(']'));
        assert_eq!(parse_line("{()()()>"), LineResult::Corrupted('>'));
        assert_eq!(parse_line("(((()))}"), LineResult::Corrupted('}'));
        assert_eq!(
            parse_line("{([(<{}[<>[]}>{[]{[(<()>"),
            LineResult::Corrupted('}')
        );
        assert_eq!(
            parse_line("[[<[([]))<([[{}[[()]]]"),
            LineResult::Corrupted(')')
        );
        assert_eq!(
            parse_line("[{[{({}]{}}([{[{{{}}([]"),
            LineResult::Corrupted(']')
        );
        assert_eq!(
            parse_line("[<(<(<(<{}))><([]([]()"),
            LineResult::Corrupted(')')
        );
        assert_eq!(
            parse_line("<{([([[(<>()){}]>(<<{{"),
            LineResult::Corrupted('>')
        );
    }

    #[test]
    fn test_autocompleted_chunk() {
        assert_eq!(
            parse_line("[({(<(())[]>[[{[]{<()<>>"),
            LineResult::Autocompleted(vec!['}', '}', ']', ']', ')', '}', ')', ']'])
        );
        assert_eq!(
            parse_line("[(()[<>])]({[<{<<[]>>("),
            LineResult::Autocompleted(vec![')', '}', '>', ']', '}', ')'])
        );
        assert_eq!(
            parse_line("(((({<>}<{<{<>}{[]{[]{}"),
            LineResult::Autocompleted(vec!['}', '}', '>', '}', '>', ')', ')', ')', ')'])
        );
        assert_eq!(
            parse_line("{<[[]]>}<{[{[{[]{()[[[]"),
            LineResult::Autocompleted(vec![']', ']', '}', '}', ']', '}', ']', '}', '>'])
        );
        assert_eq!(
            parse_line("<{([{{}}[<[[[<>{}]]]>[]]"),
            LineResult::Autocompleted(vec![']', ')', '}', '>'])
        );
    }
}
