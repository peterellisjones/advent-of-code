use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    SolutionNotFound,
}

fn main() {
    match part_one("1.txt", 12) {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(count) => println!("Run part one: {:?}", count),
    }

    match part_two("1.txt", 12) {
        Err(e) => println!("Run part_two error: {:?}", e),
        Ok(count) => println!("Run part_two: {:?}", count),
    }
}

fn part_one(path: &str, bits: usize) -> Result<(usize, usize, usize), Error> {
    let report = open_file(&path)?;

    let gamma_rate = (0..bits)
        .map(|i| 1 << i)
        .filter(|m| report.iter().filter(|&n| (n & m) != 0).count() > report.len() / 2)
        .fold(0, |s, m| s | m);

    let epsilon_rate = (usize::MAX ^ gamma_rate) & ((1 << bits) - 1);

    Ok((gamma_rate, epsilon_rate, gamma_rate * epsilon_rate))
}

fn find_candidate(
    report: &Vec<usize>,
    bits: usize,
    filter: fn(isize, isize) -> bool,
) -> Result<usize, Error> {
    let mut candidates = report.clone();

    for m in (0..bits).rev().map(|i| 1 << i) {
        let ones = candidates.iter().filter(|&n| (n & m) != 0).count() as isize;
        let filter_ones = filter(ones, candidates.len() as isize - ones);

        candidates = candidates
            .iter()
            .filter(|&n| (n & m != 0) == filter_ones)
            .map(|&n| n)
            .collect();
        if candidates.len() == 1 {
            return Ok(candidates[0]);
        } else if candidates.len() == 0 {
            return Err(Error::SolutionNotFound);
        }
    }
    Err(Error::SolutionNotFound)
}

fn part_two(path: &str, bits: usize) -> Result<(usize, usize, usize), Error> {
    let report = open_file(&path)?;
    let og = find_candidate(&report, bits, |ones, zeroes| ones >= zeroes)?;
    let co2 = find_candidate(&report, bits, |ones, zeroes| ones < zeroes)?;

    Ok((og, co2, og * co2))
}

fn open_file(filename: &str) -> Result<Vec<usize>, Error> {
    io::BufReader::new(File::open(filename).map_err(Error::IOError)?)
        .lines()
        .map(|line| -> Result<usize, Error> {
            let num_str = line.map_err(Error::IOError)?;
            usize::from_str_radix(&num_str, 2).map_err(Error::ParseIntError)
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
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), (22, 9, 198));
    }

    #[test]
    fn test_part_two() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input, 5);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), (23, 10, 230));
    }
}
