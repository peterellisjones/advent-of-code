use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
}

fn main() {
    let part_one_input = "input_part_one.txt";
    let part_two_input = "input_part_two.txt";

    match part_one(&part_one_input) {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(count) => println!("Run part one: {:?}", count),
    }

    match part_two(&part_two_input) {
        Err(e) => println!("Run part_two error: {:?}", e),
        Ok(count) => println!("Run part_two: {:?}", count),
    }
}

fn part_one(path: &str) -> Result<usize, Error> {
    let depths = open_file(&path)?;
    return Ok(depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|&(l, r)| l < r)
        .count());
}

fn part_two(path: &str) -> Result<usize, Error> {
    let depths = open_file(&path)?;

    let sums = depths
        .iter()
        .zip(depths.iter().skip(1))
        .zip(depths.iter().skip(2))
        .map(&|((a, b), c)| a + b + c);

    return Ok(sums
        .clone()
        .zip(sums.clone().skip(1))
        .filter(|&(l, r)| l < r)
        .count());
}

fn open_file(filename: &str) -> Result<Vec<i64>, Error> {
    io::BufReader::new(File::open(filename).map_err(Error::IOError)?)
        .lines()
        .map(|line| {
            line.map_err(Error::IOError)?
                .parse::<i64>()
                .map_err(Error::ParseIntError)
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
        let test_input = "input_part_one_test.txt";
        let ret = part_one(&test_input);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), 7);
    }

    #[test]
    fn test_part_two() {
        let test_input = "input_part_two_test.txt";
        let ret = part_two(&test_input);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), 5);
    }
}
