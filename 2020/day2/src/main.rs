use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    RegexError(regex::Error),
    NoCaptureError,
}

#[derive(Debug)]
struct Entry {
    min: usize,
    max: usize,
    letter: char,
    password: Vec<char>,
}

fn main() {
    match part_one("1.txt") {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(count) => println!("Run part one: {:?}", count),
    }

    match part_two("2.txt") {
        Err(e) => println!("Run part two error: {:?}", e),
        Ok(count) => println!("Run part two: {:?}", count),
    }
}

fn part_one(path: &str) -> Result<usize, Error> {
    Ok(open_file(&path)?
        .iter()
        .filter(|e| {
            let count = e.password.iter().filter(|&&c| c == e.letter).count();
            count >= e.min && count <= e.max
        })
        .count())
}

fn part_two(path: &str) -> Result<usize, Error> {
    Ok(open_file(&path)?
        .iter()
        .filter(|e| (e.password[e.min - 1] == e.letter) ^ (e.password[e.max - 1] == e.letter))
        .count())
}

fn open_file(filename: &str) -> Result<Vec<Entry>, Error> {
    let rgx = Regex::new(r"([0-9]+)-([0-9]+) ([A-z]): ([A-z]+)").map_err(Error::RegexError)?;

    io::BufReader::new(File::open(filename).map_err(Error::IOError)?)
        .lines()
        .map(|line| -> Result<Entry, Error> {
            let input = line.map_err(Error::IOError)?;

            let captures = rgx.captures(&input).ok_or(Error::NoCaptureError)?;

            Ok(Entry {
                min: captures
                    .get(1)
                    .ok_or(Error::NoCaptureError)?
                    .as_str()
                    .parse::<usize>()
                    .map_err(Error::ParseIntError)?,
                max: captures
                    .get(2)
                    .ok_or(Error::NoCaptureError)?
                    .as_str()
                    .parse::<usize>()
                    .map_err(Error::ParseIntError)?,
                letter: captures
                    .get(3)
                    .ok_or(Error::NoCaptureError)?
                    .as_str()
                    .chars()
                    .collect::<Vec<char>>()[0],
                password: captures
                    .get(4)
                    .ok_or(Error::NoCaptureError)?
                    .as_str()
                    .chars()
                    .collect(),
            })
        })
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_part_one() {
        let test_input = "1_test.txt";
        let ret = part_one(&test_input);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), 2);
    }

    #[test]
    fn test_part_two() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), 1);
    }
}
