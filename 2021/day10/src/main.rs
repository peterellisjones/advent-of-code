use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidToken(char),
    UnexpectedToken(Token),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    LeftAngle,
    LeftBrace,
    LeftBracket,
    LeftParen,
    RightAngle,
    RightBrace,
    RightBracket,
    RightParen,
}

fn main() {
    match part_one("1.txt") {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(res) => println!("Run part one: {:?}", res),
    }

    match part_two("1.txt") {
        Err(e) => println!("Run part two error: {:?}", e),
        Ok(res) => println!("Run part two: {:?}", res),
    }
}

fn part_one(path: &str) -> Result<i64, Error> {
    let input = parse_input(path)?;

    let mut failed_tokens: Vec<Token> = Vec::new();
    for line in input {
        let (token, _) = scan_line(&line);
        if token.is_some() {
            failed_tokens.push(token.unwrap());
        }
    }

    let mut score = 0;
    for token in failed_tokens {
        score += match token {
            Token::RightParen => 3,
            Token::RightBracket => 57,
            Token::RightBrace => 1197,
            Token::RightAngle => 25137,
            _ => return Err(Error::UnexpectedToken(token)),
        }
    }

    Ok(score)
}

fn part_two(path: &str) -> Result<i64, Error> {
    let input = parse_input(path)?;

    let mut scores: Vec<i64> = Vec::new();

    for line in input {
        let (invalid_token, stack) = scan_line(&line);
        if invalid_token.is_some() {
            continue;
        }

        scores.push(stack.iter().rev().fold(0, |s, token| {
            s * 5 + {
                match token {
                    Token::LeftParen => 1,
                    Token::LeftBracket => 2,
                    Token::LeftBrace => 3,
                    Token::LeftAngle => 4,
                    _ => unreachable!(),
                }
            }
        }));
    }

    scores.sort();

    Ok(scores[scores.len()/2])
}

fn scan_line(line: &Vec<Token>) -> (Option<Token>, Vec<Token>) {
    let mut stack: Vec<Token> = Vec::new();
    for &token in line {
        match token {
            Token::LeftAngle => stack.push(token),
            Token::LeftBrace => stack.push(token),
            Token::LeftBracket => stack.push(token),
            Token::LeftParen => stack.push(token),
            Token::RightAngle => {
                if stack.pop() != Some(Token::LeftAngle) {
                    return (Some(token), stack);
                }
            }
            Token::RightBrace => {
                if stack.pop() != Some(Token::LeftBrace) {
                    return (Some(token), stack);
                }
            }
            Token::RightBracket => {
                if stack.pop() != Some(Token::LeftBracket) {
                    return (Some(token), stack);
                }
            }
            Token::RightParen => {
                if stack.pop() != Some(Token::LeftParen) {
                    return (Some(token), stack);
                }
            }
        };
    }
    (None, stack)
}

fn parse_input(path: &str) -> Result<Vec<Vec<Token>>, Error> {
    io::BufReader::new(File::open(path).map_err(Error::IO)?)
        .lines()
        .map(|maybe_line| -> Result<Vec<Token>, Error> {
            let line = maybe_line.map_err(Error::IO)?;

            line.chars()
                .map(|c| match c {
                    '(' => Ok(Token::LeftParen),
                    ')' => Ok(Token::RightParen),
                    '[' => Ok(Token::LeftBracket),
                    ']' => Ok(Token::RightBracket),
                    '{' => Ok(Token::LeftBrace),
                    '}' => Ok(Token::RightBrace),
                    '<' => Ok(Token::LeftAngle),
                    '>' => Ok(Token::RightAngle),
                    _ => Err(Error::InvalidToken(c)),
                })
                .into_iter()
                .collect::<Result<Vec<Token>, Error>>()
        })
        .collect::<Result<Vec<Vec<Token>>, Error>>()
}

mod tests {
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_part_one() {
        let res = part_one("1_test.txt");
        assert_eq!(res.unwrap(), 26397);
    }

    #[test]
    fn test_part_two() {
        let res = part_two("1_test.txt");
        assert_eq!(res.unwrap(), 288957);
    }
}
