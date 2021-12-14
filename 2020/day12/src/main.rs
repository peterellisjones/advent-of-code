use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    Regex(regex::Error),
    ParseInt(std::num::ParseIntError),
    UnknownFacing(i64),
    ParseLine(String),
}

#[derive(Debug)]
enum Instruction {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

fn main() {
    match part_one("1.txt") {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok((ew, ns)) => println!("Run part one: {:?} {:?} {:?}", ew, ns, ew + ns),
    }

    match part_two("1.txt") {
        Err(e) => println!("Run part two error: {:?}", e),
        Ok((ew, ns)) => println!("Run part two: {:?} {:?} {:?}", ew, ns, ew + ns),
    }
}

fn part_one(path: &str) -> Result<(i64, i64), Error> {
    let instructions = open_file(&path)?;

    let mut facing = 90; // east
    let mut north_south = 0;
    let mut east_west = 0;

    for instruction in instructions {
        match instruction {
            Instruction::North(v) => north_south += v,
            Instruction::South(v) => north_south -= v,
            Instruction::East(v) => east_west += v,
            Instruction::West(v) => east_west -= v,
            Instruction::Left(v) => facing = (facing - v + 360) % 360,
            Instruction::Right(v) => facing = (facing + v) % 360,
            Instruction::Forward(v) => match facing {
                90 => east_west += v,
                270 => east_west -= v,
                0 => north_south += v,
                180 => north_south -= v,
                _ => return Err(Error::UnknownFacing(facing)),
            },
        }
    }

    Ok((east_west.abs(), north_south.abs()))
}

fn part_two(path: &str) -> Result<(i64, i64), Error> {
    let instructions = open_file(&path)?;

    let mut ship_north_south = 0;
    let mut ship_east_west = 0;
    let mut wp_north_south = 1;
    let mut wp_east_west = 10;

    for instruction in instructions {
        match instruction {
            Instruction::North(v) => wp_north_south += v,
            Instruction::South(v) => wp_north_south -= v,
            Instruction::East(v) => wp_east_west += v,
            Instruction::West(v) => wp_east_west -= v,
            Instruction::Right(v) => {
                let ew = wp_east_west;
                let ns = wp_north_south;
                match v {
                    90 => {
                        wp_north_south = -ew;
                        wp_east_west = ns;
                    }
                    270 => {
                        wp_north_south = ew;
                        wp_east_west = -ns;
                    }
                    180 => {
                        wp_north_south = -ns;
                        wp_east_west = -ew;
                    }
                    _ => return Err(Error::UnknownFacing(v)),
                }
            }
            Instruction::Left(v) => {
                let ew = wp_east_west;
                let ns = wp_north_south;
                match v {
                    90 => {
                        wp_north_south = ew;
                        wp_east_west = -ns;
                    }
                    270 => {
                        wp_north_south = -ew;
                        wp_east_west = ns;
                    }
                    180 => {
                        wp_north_south = -ns;
                        wp_east_west = -ew;
                    }
                    _ => return Err(Error::UnknownFacing(v)),
                }
            }
            Instruction::Forward(v) => {
                ship_east_west += wp_east_west * v;
                ship_north_south += wp_north_south * v;
            }
        }
    }

    Ok((ship_east_west.abs(), ship_north_south.abs()))
}

fn open_file(filename: &str) -> Result<Vec<Instruction>, Error> {
    let rgx = Regex::new(r"(N|S|E|W|L|R|F)([0-9]+)").map_err(Error::Regex)?;
    io::BufReader::new(File::open(filename).map_err(Error::IO)?)
        .lines()
        .map(|maybe_line| -> Result<Instruction, Error> {
            let line = maybe_line.map_err(Error::IO)?;
            let captures = rgx
                .captures(&line)
                .ok_or(Error::ParseLine(line.to_string()))?;
            let dir = captures
                .get(1)
                .ok_or(Error::ParseLine(line.to_string()))?
                .as_str();
            let val = captures
                .get(2)
                .ok_or(Error::ParseLine(line.to_string()))?
                .as_str()
                .parse::<i64>()
                .map_err(Error::ParseInt)?;

            match dir {
                "N" => Ok(Instruction::North(val)),
                "S" => Ok(Instruction::South(val)),
                "E" => Ok(Instruction::East(val)),
                "W" => Ok(Instruction::West(val)),
                "L" => Ok(Instruction::Left(val)),
                "R" => Ok(Instruction::Right(val)),
                "F" => Ok(Instruction::Forward(val)),
                _ => Err(Error::ParseLine(line.to_string())),
            }
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
        assert_eq!(ret.unwrap(), (17, 8));
    }

    #[test]
    fn test_part_two() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), (214, 72));
    }
}
