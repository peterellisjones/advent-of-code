use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    ParseLineError,
    ParseDirectionError,
}

fn main() {
    match part_one("input_part_one.txt") {
        Err(e) => println!("part one error: {:?}", e),
        Ok((x, z)) => println!("part one: {:?} * {:?} = {:?}", x, z, x * z),
    }

    match part_two("input_part_two.txt") {
        Err(e) => println!("part two error: {:?}", e),
        Ok((x, z, _)) => println!("part two: {:?} * {:?} = {:?}", x, z, x * z),
    }
}

fn part_one(path: &str) -> Result<(i64, i64), Error> {
    Ok(open_file(&path)?
        .iter()
        .fold((0, 0), |(x, z), (dx, dz)| -> (i64, i64) {
            (x + dx, z + dz)
        }))
}

fn part_two(path: &str) -> Result<(i64, i64, i64), Error> {
    Ok(open_file(&path)?
        .iter()
        .fold((0, 0, 0), |(x, z, aim), (dx, da)| -> (i64, i64, i64) {
            (x + dx, z + dx * aim, aim + da)
        }))
}

fn open_file(filename: &str) -> Result<Vec<(i64, i64)>, Error> {
    io::BufReader::new(File::open(filename).map_err(Error::IOError)?)
        .lines()
        .map(|line| -> Result<(i64, i64), Error> {
            line.map_err(Error::IOError)?
                .split_once(" ")
                .ok_or(Error::ParseLineError)
                .map(|(col_one, col_two)| -> Result<(&str, i64), Error> {
                    Ok((
                        col_one,
                        col_two.parse::<i64>().map_err(Error::ParseIntError)?,
                    ))
                })?
                .map(|(col_one, distance)| -> Result<(i64, i64), Error> {
                    match col_one {
                        "forward" => Ok((distance, 0)),
                        "down" => Ok((0, distance)),
                        "up" => Ok((0, -distance)),
                        _ => Err(Error::ParseDirectionError),
                    }
                })?
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
        let ret = part_one("input_part_one_test.txt");
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), (15, 10));
    }

    #[test]
    fn test_part_two() {
        let ret = part_two("input_part_two_test.txt");
        assert_eq!(ret.is_ok(), true);
        let (x, z, _) = ret.unwrap();
        assert_eq!(x, 15);
        assert_eq!(z, 60);
    }
}
