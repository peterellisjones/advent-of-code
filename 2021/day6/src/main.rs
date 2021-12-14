use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),
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
    let fish = parse_comma_separated_ints(path)?;

    Ok(count_lanternfish(&fish, 80))
}

fn part_two(path: &str) -> Result<usize, Error> {
    let fish = parse_comma_separated_ints(path)?;

    Ok(count_lanternfish(&fish, 256))
}

fn count_lanternfish(input: &Vec<usize>, days: usize) -> usize {
    let mut populations = [0usize; 9];

    for fish in input {
        populations[*fish] += 1;
    }

    for _ in 0..days {
        let mut updated_populations = [0usize; 9];

        for i in 1..9 {
            updated_populations[i - 1] = populations[i];
        }

        updated_populations[6] += populations[0]; // - reset timer to 6
        updated_populations[8] += populations[0]; // - add children with timer 8

        populations = updated_populations;
    }

    populations.iter().sum()
}

fn parse_comma_separated_ints(path: &str) -> Result<Vec<usize>, Error> {
    Ok(io::BufReader::new(File::open(path).map_err(Error::IO)?)
        .lines()
        .map(|line| -> Result<Vec<usize>, Error> {
            line.map_err(Error::IO)?
                .split(",")
                .map(|s| s.parse::<usize>().map_err(Error::ParseInt))
                .collect::<Result<Vec<usize>, Error>>()
        })
        .collect::<Result<Vec<Vec<usize>>, Error>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<usize>>())
}

mod tests {
    use super::count_lanternfish;
    #[test]
    fn test_count_lanternfish() {
        let input = vec![3, 4, 3, 1, 2];
        assert_eq!(count_lanternfish(&input, 18), 26);
        assert_eq!(count_lanternfish(&input, 80), 5934);
        assert_eq!(count_lanternfish(&input, 256), 26984457539);
    }
}
