use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{map, value},
    multi::many1,
    sequence::preceded,
    IResult,
};

use crate::common::parse::finish;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let mut count = 0;
    for line in input.trim().lines() {
        let tokens = parse(line)?;

        let number_of_characters_in_string = tokens.size_on_disk();
        let number_of_characters_in_memory = tokens.size_in_memory();

        count += number_of_characters_in_string - number_of_characters_in_memory;
    }

    Ok(count.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let mut count = 0;
    for line in input.trim().lines() {
        let tokens = parse(line)?;

        let number_of_characters_in_escaped_string = tokens.escaped_len();
        let number_of_characters_in_file = tokens.size_on_disk();

        count += number_of_characters_in_escaped_string - number_of_characters_in_file;
    }

    // 1446 is too low
    Ok(count.to_string())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    DoubleQuote,
    Literal,
    Backslash,
    DoubleQuoteCharacter,
    HexCode,
}

impl Token {
    fn file_len(&self) -> usize {
        match self {
            Token::DoubleQuote => 1,
            Token::Literal => 1,
            Token::Backslash => 2,
            Token::DoubleQuoteCharacter => 2,
            Token::HexCode => 4,
        }
    }

    fn in_memory_len(&self) -> usize {
        match self {
            Token::DoubleQuote => 0,
            Token::Literal => 1,
            Token::Backslash => 1,
            Token::DoubleQuoteCharacter => 1,
            Token::HexCode => 1,
        }
    }

    fn escaped_len(&self) -> usize {
        // " -> "\"
        // \" -> \\\"
        // \x27 -> \\x27

        match self {
            Token::DoubleQuote => 2,
            Token::Literal => 1,
            Token::Backslash => 4,
            Token::DoubleQuoteCharacter => 4,
            Token::HexCode => 5,
        }
    }
}

#[derive(Debug, Clone)]
struct Tokens(Vec<Token>);

impl Tokens {
    fn size_on_disk(&self) -> usize {
        self.0.iter().map(Token::file_len).sum()
    }

    fn size_in_memory(&self) -> usize {
        self.0.iter().map(Token::in_memory_len).sum()
    }

    fn escaped_len(&self) -> usize {
        self.0.iter().map(Token::escaped_len).sum::<usize>() + 2
    }
}

impl PartialEq<Vec<Token>> for Tokens {
    fn eq(&self, other: &Vec<Token>) -> bool {
        &self.0 == other
    }
}

fn parse(i: &str) -> anyhow::Result<Tokens> {
    finish(tokens)(i)
        .map_err(|e| e.to_owned().into())
        .map(|(_, tokens)| tokens)
}

fn tokens(i: &str) -> IResult<&str, Tokens> {
    map(many1(token), Tokens)(i)
}

fn token(i: &str) -> IResult<&str, Token> {
    alt((
        hex_code,
        double_quote_character,
        double_quote,
        backslash,
        literal,
    ))(i)
}

fn double_quote(i: &str) -> IResult<&str, Token> {
    value(Token::DoubleQuote, tag("\""))(i)
}

fn literal(i: &str) -> IResult<&str, Token> {
    value(Token::Literal, take(1_usize))(i)
}

fn backslash(i: &str) -> IResult<&str, Token> {
    value(Token::Backslash, tag("\\\\"))(i)
}

fn double_quote_character(i: &str) -> IResult<&str, Token> {
    value(Token::DoubleQuoteCharacter, tag("\\\""))(i)
}

fn hex_code(i: &str) -> IResult<&str, Token> {
    value(Token::HexCode, preceded(tag("\\x"), take(2_usize)))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tokens() {
        assert_eq!(
            tokens("\"vcqc\"").unwrap().1,
            vec![
                Token::DoubleQuote,
                Token::Literal,
                Token::Literal,
                Token::Literal,
                Token::Literal,
                Token::DoubleQuote
            ]
        );

        assert_eq!(
            tokens("\"du\\x4c\"").unwrap().1,
            vec![
                Token::DoubleQuote,
                Token::Literal,
                Token::Literal,
                Token::HexCode,
                Token::DoubleQuote
            ]
        );

        assert_eq!(
            tokens("\"\\\"oedr\"").unwrap().1,
            vec![
                Token::DoubleQuote,
                Token::DoubleQuoteCharacter,
                Token::Literal,
                Token::Literal,
                Token::Literal,
                Token::Literal,
                Token::DoubleQuote
            ]
        );

        assert_eq!(
            tokens("\"ky\\\\m\"").unwrap().1,
            vec![
                Token::DoubleQuote,
                Token::Literal,
                Token::Literal,
                Token::Backslash,
                Token::Literal,
                Token::DoubleQuote
            ]
        );
    }

    #[test]
    fn test_lengths() {
        let tokens = parse("\"\"").unwrap();
        assert_eq!(tokens, vec![Token::DoubleQuote, Token::DoubleQuote]);
        assert_eq!(tokens.size_on_disk(), 2);
        assert_eq!(tokens.size_in_memory(), 0);
        assert_eq!(tokens.escaped_len(), 6);

        let tokens = parse("\"abc\"").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::DoubleQuote,
                Token::Literal,
                Token::Literal,
                Token::Literal,
                Token::DoubleQuote
            ]
        );
        assert_eq!(tokens.size_on_disk(), 5);
        assert_eq!(tokens.size_in_memory(), 3);
        assert_eq!(tokens.escaped_len(), 9);

        let tokens = parse("\"aaa\\\"aaa\"").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::DoubleQuote,
                Token::Literal,
                Token::Literal,
                Token::Literal,
                Token::DoubleQuoteCharacter,
                Token::Literal,
                Token::Literal,
                Token::Literal,
                Token::DoubleQuote
            ]
        );
        assert_eq!(tokens.size_on_disk(), 10);
        assert_eq!(tokens.size_in_memory(), 7);
        assert_eq!(tokens.escaped_len(), 16);
    }
}
