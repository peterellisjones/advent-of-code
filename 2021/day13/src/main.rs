use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),
    Parse(String),
    Regex(regex::Error),
}

#[derive(Debug)]
enum Fold {
    Y(i64),
    X(i64),
}

#[derive(Debug)]
struct Instructions {
    dots: Vec<(i64, i64)>,
    folds: Vec<Fold>,
}

fn main() {
    match part_one("input_1.txt", 1, false) {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(res) => println!("Run part one: {:?}", res),
    }

    match part_one("input_1.txt", 100, true) {
        Err(e) => println!("Run part two error: {:?}", e),
        Ok(res) => println!("Run part two: {:?}", res),
    }
}

fn part_one(path: &str, num_folds: usize, show_output: bool) -> Result<usize, Error> {
    let input = parse_input(path)?;
    let mut paper: HashSet<(i64, i64)> = HashSet::new();

    for (x, y) in input.dots {
        paper.insert((x, y));
    }

    for fold in input.folds.iter().take(num_folds) {
        let updated_paper: HashSet<(i64, i64)> =
            HashSet::from_iter(paper.iter().map(|&(x, y)| match fold {
                Fold::X(fold_x) => (
                    if x < *fold_x {
                        x
                    } else {
                        fold_x - (x - fold_x)
                    },
                    y,
                ),
                Fold::Y(fold_y) => (
                    x,
                    if y < *fold_y {
                        y
                    } else {
                        fold_y - (y - fold_y)
                    },
                ),
            }));
        paper = updated_paper;
    }

    if !show_output {
        return Ok(paper.len());
    }
    let max_x = *paper.iter().map(|(x, y)| x).max().unwrap();
    let max_y = *paper.iter().map(|(x, y)| y).max().unwrap();
    let mut display = String::new();

    for y in 0i64..max_y {
        for x in 0i64..max_x {
            display += if paper.contains(&(x, y)) { "#" } else { "." };
        }
        display += "\n"
    }

    println!("{}", display);

    Ok(paper.len())
}

fn parse_input(path: &str) -> Result<Instructions, Error> {
    let mut contents = String::new();
    File::open(path)
        .map_err(Error::IO)?
        .read_to_string(&mut contents)
        .map_err(Error::IO)?;

    let (top, bottom) = contents
        .split_once("\r\n\r\n")
        .ok_or(Error::Parse(contents.to_string()))?;

    let x_fold_rgx = Regex::new(r"fold along x=([0-9]+)").map_err(Error::Regex)?;
    let y_fold_rgx = Regex::new(r"fold along y=([0-9]+)").map_err(Error::Regex)?;

    let parse_fold = |rgx: &Regex, line: &str| {
        rgx.captures(&line)
            .ok_or(Error::Parse(line.to_string()))?
            .get(1)
            .ok_or(Error::Parse(line.to_string()))?
            .as_str()
            .parse::<i64>()
            .map_err(Error::ParseInt)
    };

    Ok(Instructions {
        dots: top
            .split("\r\n")
            .map(|line| -> Result<(i64, i64), Error> {
                let (left, right) = line.split_once(",").ok_or(Error::Parse(line.to_string()))?;

                Ok((
                    left.parse::<i64>().map_err(Error::ParseInt)?,
                    right.parse::<i64>().map_err(Error::ParseInt)?,
                ))
            })
            .into_iter()
            .collect::<Result<Vec<(i64, i64)>, Error>>()?,
        folds: bottom
            .split("\r\n")
            .map(|line| -> Result<Fold, Error> {
                if x_fold_rgx.is_match(line) {
                    Ok(Fold::X(parse_fold(&x_fold_rgx, line)?))
                } else if y_fold_rgx.is_match(line) {
                    Ok(Fold::Y(parse_fold(&y_fold_rgx, line)?))
                } else {
                    Err(Error::Parse(line.to_string()))
                }
            })
            .into_iter()
            .collect::<Result<Vec<Fold>, Error>>()?,
    })
}

mod tests {
    use super::part_one;

    #[test]
    fn test_part_one() {
        let res = part_one("test_1.txt", 100, false);
        assert_eq!(res.unwrap(), 16);
    }
}
