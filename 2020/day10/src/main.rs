use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    SolutionNotFound,
}

fn main() {
    match part_one("1.txt") {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok((diffs_1, diffs_3)) => println!("Run part one: {:?}", diffs_1 * diffs_3),
    }

    match part_two("1.txt") {
        Err(e) => println!("Run part two error: {:?}", e),
        Ok(count) => println!("Run part two: {:?}", count),
    }
}

fn part_one(path: &str) -> Result<(i64, i64), Error> {
    let mut numbers = open_file(path)?;
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers.last().ok_or(Error::SolutionNotFound)? + 3);

    let mut diff_counts: HashMap<i64, i64> = HashMap::new();

    for i in 1..numbers.len() {
        *diff_counts.entry(numbers[i] - numbers[i - 1]).or_insert(0) += 1;
    }

    Ok((*diff_counts.get(&1).unwrap(), *diff_counts.get(&3).unwrap()))
}

fn part_two(path: &str) -> Result<usize, Error> {
    let mut numbers = open_file(path)?;
    numbers.push(0);
    numbers.sort();

    let mut arrangements: HashMap<i64, usize> = HashMap::new();
    arrangements.insert(numbers.last().ok_or(Error::SolutionNotFound)? + 3, 1);

    for idx in 0..numbers.len() {
        let i = numbers[numbers.len() - 1 - idx];
        for j in [i + 1, i + 2, i + 3] {
            *arrangements.entry(i).or_insert(0) += *arrangements.entry(j).or_insert(0);
        }
    }

    Ok(*arrangements.get(&0).ok_or(Error::SolutionNotFound)?)
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
    fn test_part_one_a() {
        let test_input = "1_test.txt";
        let ret = part_one(&test_input);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), (7, 5));
    }

    #[test]
    fn test_part_one_b() {
        let test_input = "2_test.txt";
        let ret = part_one(&test_input);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), (22, 10));
    }

    #[test]
    fn test_part_two_a() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 8);
    }

    #[test]
    fn test_part_two_b() {
        let test_input = "2_test.txt";
        let ret = part_two(&test_input);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 19208);
    }
}
