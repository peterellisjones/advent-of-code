use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    ParseLine,
    UnknownInstruction,
    SolutionNotFound,
    ParseIntError(std::num::ParseIntError),
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
    let codes = open_file(path)?;
    let (accumulator, _) = run(&codes)?;
    Ok(accumulator)
}

fn part_two(path: &str) -> Result<i64, Error> {
    let codes = &mut open_file(path)?;

    for idx in 0..codes.len() {
        let (instruction, value) = codes[idx].clone();
        let new_instruction = if instruction == "nop" {
            "jmp"
        } else if instruction == "jmp" {
            "nop"
        } else {
            continue;
        };

        // replace instruction
        codes[idx] = (new_instruction.to_string(), value);

        let (accumulator, terminates) = run(codes)?;
        if terminates {
            return Ok(accumulator);
        }

        // put instruction back
        codes[idx] = (instruction, value);
    }

    Err(Error::SolutionNotFound)
}

fn run(codes: &Vec<(String, i64)>) -> Result<(i64, bool), Error> {
    let mut accumulator = 0;
    let mut idx = 0;
    let mut visited_idxs: HashSet<i64> = HashSet::new();
    let mut terminates = false;
    loop {
        if visited_idxs.contains(&idx) {
            break;
        }

        if idx == codes.len() as i64 {
            terminates = true;
            break;
        }

        let (instruction, value) = &codes[idx as usize];
        visited_idxs.insert(idx);
        if instruction == "nop" {
            idx += 1;
        } else if instruction == "acc" {
            idx += 1;
            accumulator += value;
        } else if instruction == "jmp" {
            idx += value;
        } else {
            return Err(Error::UnknownInstruction);
        }
    }

    Ok((accumulator, terminates))
}

fn open_file(filename: &str) -> Result<Vec<(String, i64)>, Error> {
    io::BufReader::new(File::open(filename).map_err(Error::IOError)?)
        .lines()
        .map(|maybe_line| -> Result<(String, i64), Error> {
            let line = maybe_line.map_err(Error::IOError)?;
            let (instruction, value) = line.split_once(" ").ok_or(Error::ParseLine)?;
            Ok((
                instruction.to_string(),
                value.parse::<i64>().map_err(Error::ParseIntError)?,
            ))
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
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 5);
    }

    #[test]
    fn test_part_two() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 8);
    }
}
