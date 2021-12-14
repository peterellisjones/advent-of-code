use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
}

fn main() {
    match part_one("1.txt") {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(count) => println!("Run part one: {:?}", count),
    }

    match part_two("1.txt") {
        Err(e) => println!("Run part two error: {:?}", e),
        Ok(count) => println!("Run part two: {:?}", count),
    }
}

fn part_one(path: &str) -> Result<usize, Error> {
    let course = open_file(path)?;
    let mut tree_hits = 0;
    for (y_idx, row) in course.iter().enumerate() {
        let x_idx = y_idx * 3 % row.len();
        if row[x_idx] {
            tree_hits += 1;
        }
    }

    Ok(tree_hits)
}

fn part_two(path: &str) -> Result<usize, Error> {
    let course = open_file(path)?;

    Ok([(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |n, &(dx, dy): &(usize, usize)| {
            n * course
                .iter()
                .step_by(dy)
                .enumerate()
                .filter(|(y_idx, row)| row[y_idx * dx % row.len()])
                .count()
        }))
}

fn open_file(filename: &str) -> Result<Vec<Vec<bool>>, Error> {
    io::BufReader::new(File::open(filename).map_err(Error::IOError)?)
        .lines()
        .map(|line| -> Result<Vec<bool>, Error> {
            Ok(line
                .map_err(Error::IOError)?
                .chars()
                .map(|c| c == '#')
                .collect())
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
        assert_eq!(ret.unwrap(), 7);
    }

    #[test]
    fn test_part_two() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), 336);
    }
}
