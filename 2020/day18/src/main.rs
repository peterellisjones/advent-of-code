use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidRPNToken(Token),
    InvalidToken(char),
    ParseInt(std::num::ParseIntError),
}

#[derive(Debug, PartialEq)]
enum Token {
    Multiply,
    Add,
    LeftParen,
    RightParen,
    Value(i64),
}

fn main() {
    match part_one("1.txt") {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(count) => println!("Run part one: {:?}", count),
    }

    match part_two("1.txt") {
        Err(e) => println!("Run part_two error: {:?}", e),
        Ok(count) => println!("Run part_two: {:?}", count),
    }
}

fn part_one(path: &str) -> Result<i64, Error> {
    let problems = open_file(path)?;

    let mut total = 0;
    for tokens in problems {
        total += solve_problem(tokens, true)?;
    }

    Ok(total)
}

fn part_two(path: &str) -> Result<i64, Error> {
    let problems = open_file(path)?;

    let mut total = 0;
    for tokens in problems {
        total += solve_problem(tokens, false)?;
    }

    Ok(total)
}

fn open_file(filename: &str) -> Result<Vec<Vec<Token>>, Error> {
    io::BufReader::new(File::open(filename).map_err(Error::IO)?)
        .lines()
        .map(|maybe_line| -> Result<Vec<Token>, Error> {
            let line = maybe_line.map_err(Error::IO)?;
            parse_line(&line)
        })
        .into_iter()
        .collect()
}

fn parse_line(line: &str) -> Result<Vec<Token>, Error> {
    line.chars()
        .filter(|&c| c != ' ')
        .map(|c| match c {
            '+' => Ok(Token::Add),
            '*' => Ok(Token::Multiply),
            '(' => Ok(Token::LeftParen),
            ')' => Ok(Token::RightParen),
            _ => {
                if c >= '0' && c <= '9' {
                    c.to_string()
                        .parse::<i64>()
                        .map_err(Error::ParseInt)
                        .map(Token::Value)
                } else {
                    Err(Error::InvalidToken(c))
                }
            }
        })
        .into_iter()
        .collect()
}

fn solve_problem(tokens: Vec<Token>, left_associative: bool) -> Result<i64, Error> {
    // Use Shunting Yard Algorihm to convert to RPN queue
    let mut operator_queue: Vec<Token> = Vec::new();
    let mut output_queue: Vec<Token> = Vec::new();
    // https://en.wikipedia.org/wiki/Shunting-yard_algorithm#The_algorithm_in_detail
    for token in tokens {
        match token {
            Token::Value(_) => output_queue.push(token),
            Token::Add | Token::Multiply => {
                let mut top_operator = operator_queue.last();
                while top_operator.is_some() && top_operator != Some(&Token::LeftParen) {
                    if !left_associative {
                        // if not left associateve then multiply is higher precendence than addition
                        if *top_operator.unwrap() == Token::Multiply && token == Token::Add {
                            break;
                        }
                    }

                    output_queue.push(operator_queue.pop().unwrap());
                    top_operator = operator_queue.last();
                }
                operator_queue.push(token);
            }
            Token::LeftParen => operator_queue.push(token),
            Token::RightParen => {
                while operator_queue.last() != Some(&Token::LeftParen) {
                    output_queue.push(operator_queue.pop().unwrap());
                }
                operator_queue.pop();
            }
        }
    }

    let mut top_operator = operator_queue.last();
    while top_operator.is_some() {
        output_queue.push(operator_queue.pop().unwrap());
        top_operator = operator_queue.last();
    }

    solve_rpn_queue(output_queue)
}

fn solve_rpn_queue(input: Vec<Token>) -> Result<i64, Error> {
    let mut values: Vec<i64> = Vec::new();
    for token in input {
        match token {
            Token::Value(x) => values.push(x),
            Token::Add => {
                let left = values.pop().unwrap();
                let right = values.pop().unwrap();
                values.push(left + right);
            }
            Token::Multiply => {
                let left = values.pop().unwrap();
                let right = values.pop().unwrap();
                values.push(left * right);
            }
            Token::LeftParen | Token::RightParen => return Err(Error::InvalidRPNToken(token)),
        }
    }

    Ok(values.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::parse_line;
    use super::solve_problem;
    use super::Token;

    #[test]
    fn solve_rpn_queue() {
        assert_eq!(
            solve_problem(parse_line("3 4 5 * +").unwrap(), true).unwrap(),
            23
        );
    }

    #[test]
    fn test_solve_problem() {
        assert_eq!(
            solve_problem(parse_line("2 * 3 + (4 * 5)").unwrap(), true).unwrap(),
            26
        );
        assert_eq!(
            solve_problem(parse_line("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap(), true).unwrap(),
            437
        );

        assert_eq!(
            solve_problem(
                parse_line("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap(),
                true
            )
            .unwrap(),
            12240
        );

        assert_eq!(
            solve_problem(
                parse_line("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap(),
                true
            )
            .unwrap(),
            13632
        );

        assert_eq!(
            solve_problem(parse_line("2 * 3 + (4 * 5)").unwrap(), false).unwrap(),
            46
        );
        assert_eq!(
            solve_problem(parse_line("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap(), false).unwrap(),
            1445
        );

        assert_eq!(
            solve_problem(
                parse_line("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap(),
                false
            )
            .unwrap(),
            669060
        );

        assert_eq!(
            solve_problem(
                parse_line("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap(),
                false
            )
            .unwrap(),
            23340
        );
    }

    #[test]
    fn test_parse_line() {
        let expected_tokens = vec![
            Token::Value(1),
            Token::Add,
            Token::LeftParen,
            Token::Value(2),
            Token::Multiply,
            Token::Value(3),
            Token::RightParen,
        ];
        let actual_tokens = parse_line("1 + (2 * 3)").unwrap();
        assert!(expected_tokens.len() == actual_tokens.len());
        for i in 0..expected_tokens.len() {
            assert_eq!(actual_tokens[i], expected_tokens[i]);
        }
    }
}
