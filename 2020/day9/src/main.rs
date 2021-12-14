use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    SolutionNotFound,
}

fn main() {
    match part_one("1.txt", 25) {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(count) => println!("Run part one: {:?}", count),
    }

    match part_two("1.txt", 177777905) {
        Err(e) => println!("Run part_two error: {:?}", e),
        Ok(count) => println!("Run part_two: {:?}", count),
    }
}

fn part_one(path: &str, window: usize) -> Result<i64, Error> {
    let numbers = open_file(path)?;

    for idx in window..numbers.len() {
        let number = numbers[idx];
        let previous_numbers = &numbers[(idx - window)..idx];

        let mut found = false;
        'outer: for i in 0..window {
            for j in i + 1..window {
                if previous_numbers[i] + previous_numbers[j] == number {
                    found = true;
                    break 'outer;
                }
            }
        }

        if !found {
            return Ok(number);
        }
    }

    Err(Error::SolutionNotFound)
}

fn part_two(path: &str, goal: i64) -> Result<i64, Error> {
    let numbers = open_file(path)?;

    let mut range_start = 0;
    let mut range_end = 0;

    'outer: for i in 0..numbers.len() {
        let mut sum = numbers[i];
        for j in i + 1..numbers.len() {
            sum += numbers[j];
            if sum > goal {
                break;
            } else if sum == goal {
                range_start = i;
                range_end = j;
                break 'outer;
            }
        }
    }

    if range_end == range_start {
        return Err(Error::SolutionNotFound);
    }

    let mut smallest = numbers[range_start];
    let mut largest = numbers[range_start];

    for i in range_start + 1..range_end {
        let num = numbers[i];
        if num < smallest {
            smallest = num;
        } else if num > largest {
            largest = num;
        }
    }

    Ok(smallest + largest)
}

fn open_file(filename: &str) -> Result<Vec<i64>, Error> {
    io::BufReader::new(File::open(filename).map_err(Error::IOError)?)
        .lines()
        .map(|maybe_line| -> Result<i64, Error> {
            let line = maybe_line.map_err(Error::IOError)?;
            line.parse::<i64>().map_err(Error::ParseIntError)
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
        let ret = part_one(&test_input, 5);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 127);
    }

    #[test]
    fn test_part_two() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input, 127);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 62);
    }
}
