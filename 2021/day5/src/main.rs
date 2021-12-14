use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),
    Parse(String),
    Regex(regex::Error),
    NotImplemented,
}

#[derive(Debug)]
struct Vent {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

fn main() {
    match part_one("1.txt") {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(overlapping_points) => println!("Run part one: {:?}", overlapping_points),
    }
    match part_two("1.txt") {
        Err(e) => println!("Run part two error: {:?}", e),
        Ok(overlapping_points) => println!("Run part two: {:?}", overlapping_points),
    }
}

fn part_one(path: &str) -> Result<usize, Error> {
    let vents = open_file(&path)?;

    Ok(count_overlaps(vents, false))
}

fn part_two(path: &str) -> Result<usize, Error> {
    let vents = open_file(&path)?;

    Ok(count_overlaps(vents, true))
}

fn count_overlaps(vents: Vec<Vent>, include_diagonals: bool) -> usize {
    // create sparse 2D array of danger spots
    let mut grid: HashMap<(i64, i64), usize> = HashMap::new();

    for vent in vents {
        if vent.x1 == vent.x2 {
            // vertical vent
            let y_start = std::cmp::min(vent.y1, vent.y2);
            let y_end = std::cmp::max(vent.y1, vent.y2);
            // +1 to include final cell
            for y in y_start..y_end + 1 {
                *grid.entry((vent.x1, y)).or_insert(0) += 1;
            }
        } else if vent.y1 == vent.y2 {
            // vertical vent
            let x_start = std::cmp::min(vent.x1, vent.x2);
            let x_end = std::cmp::max(vent.x1, vent.x2);
            // +1 to include final cell
            for x in x_start..x_end + 1 {
                *grid.entry((x, vent.y1)).or_insert(0) += 1;
            }
        } else if include_diagonals {
            // diagonal vent
            let length = std::cmp::max(vent.x1, vent.x2) - std::cmp::min(vent.x1, vent.x2);
            let d_x = if vent.x2 > vent.x1 { 1 } else { -1 };
            let d_y = if vent.y2 > vent.y1 { 1 } else { -1 };

            // +1 to include final cell
            for i in 0..length + 1 {
                let x = vent.x1 + d_x * i;
                let y = vent.y1 + d_y * i;
                *grid.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    grid.iter().filter(|(_, &v)| v >= 2).count()
}

fn open_file(filename: &str) -> Result<Vec<Vent>, Error> {
    // 234,455 -> 604,85
    let rgx = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").map_err(Error::Regex)?;

    let parse_int = |x, captures: &regex::Captures| -> Result<i64, Error> {
        captures
            .get(x)
            .ok_or(Error::Parse("couldn't match regex".to_string()))?
            .as_str()
            .parse::<i64>()
            .map_err(Error::ParseInt)
    };

    io::BufReader::new(File::open(filename).map_err(Error::IO)?)
        .lines()
        .map(|maybe_line| {
            let line = maybe_line.map_err(Error::IO)?;
            let captures = rgx
                .captures(&line)
                .ok_or(Error::Parse("couldn't match regex".to_string()))?;

            Ok(Vent {
                x1: parse_int(1, &captures)?,
                y1: parse_int(2, &captures)?,
                x2: parse_int(3, &captures)?,
                y2: parse_int(4, &captures)?,
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
        assert_eq!(ret.unwrap(), 5);
    }

    #[test]
    fn test_part_two() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input);
        assert_eq!(ret.unwrap(), 12);
    }
}
