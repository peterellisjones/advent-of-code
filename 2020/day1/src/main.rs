use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    SolutionNotFoundError,
}

fn main() {
    match part_one("1.txt") {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok((a, b)) => println!("Run part one: {:?} * {:?} = {:?}", a, b, a * b),
    }

    match part_two("2.txt") {
        Err(e) => println!("Run part_two error: {:?}", e),
        Ok((a, b, c)) => println!(
            "Run part one: {:?} * {:?} * {:?} = {:?}",
            a,
            b,
            c,
            a * b * c
        ),
    }
}

fn part_one(path: &str) -> Result<(i64, i64), Error> {
    let mut elements = open_file(&path)?;
    elements.sort();

    for (idx, &left) in elements.iter().enumerate() {
        for &right in elements.iter().skip(idx + 1) {
            let sum = left + right;
            if sum == 2020 {
                return Ok((left, right));
            } else if sum > 2020 {
                break;
            }
        }
    }

    Err(Error::SolutionNotFoundError)
}

fn part_two(path: &str) -> Result<(i64, i64, i64), Error> {
    let mut elements = open_file(&path)?;
    elements.sort();

    for (left_idx, &left) in elements.iter().enumerate() {
        for (middle_idx, &middle) in elements[left_idx + 1..].iter().enumerate() {
            if middle + left > 2020 {
                break;
            }

            for &right in elements[left_idx + middle_idx + 1..].iter() {
                let sum = left + middle + right;
                if sum == 2020 {
                    return Ok((left, middle, right));
                } else if sum > 2020 {
                    break;
                }
            }
        }
    }

    Err(Error::SolutionNotFoundError)
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
        let test_input = "1_test.txt";
        let ret = part_one(&test_input);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), (299, 1721));
    }

    #[test]
    fn test_part_two() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), (366, 675, 979));
    }
}
