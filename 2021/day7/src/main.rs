use std::collections::HashMap;
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
        Ok(res) => println!("Run part one: {:?}", res),
    }

    match part_two("1.txt") {
        Err(e) => println!("Run part two error: {:?}", e),
        Ok(res) => println!("Run part two: {:?}", res),
    }
}

fn part_one(path: &str) -> Result<i64, Error> {
    let input = parse_comma_separated_ints(path)?;

    Ok(fuel_cost_linear(&input))
}

fn part_two(path: &str) -> Result<i64, Error> {
    let input = parse_comma_separated_ints(path)?;
    Ok(fuel_cost_exponential(&input))
}

fn fuel_cost_linear(input: &Vec<i64>) -> i64 {
    // map position to count
    let mut crabs: HashMap<i64, i64> = HashMap::new();
    for &i in input {
        *crabs.entry(i).or_insert(0) += 1;
    }

    // calculates cost for a given position
    let cost_for_position = |p: i64| {
        crabs
            .iter()
            .fold(0, |s, (pos, count)| s + (p - pos).abs() * count)
    };

    let mut best_cost = cost_for_position(input[0]);
    for &pos in crabs.keys() {
        let cost = cost_for_position(pos);
        if cost < best_cost {
            best_cost = cost;
        }
    }
    best_cost
}

fn fuel_cost_exponential(input: &Vec<i64>) -> i64 {
    let max_pos = *input.iter().max().unwrap();
    let min_pos = *input.iter().min().unwrap();

    // map position to count
    let mut crabs: HashMap<i64, i64> = HashMap::new();
    for &i in input {
        *crabs.entry(i).or_insert(0) += 1;
    }

    // 1 => 1
    // 2 => 3
    // 3 => 6
    // 4 => 10
    // 5 => 15
    // F => (P+1)P/2

    // calculates cost for a given position
    let cost_for_position = |p: i64| {
        crabs.iter().fold(0, |s, (pos, count)| {
            let d = (p - pos).abs();
            let f = (d * (d + 1)) / 2;
            s + f * count
        })
    };

    let mut best_cost = cost_for_position(input[0]);
    for pos in min_pos..=max_pos {
        let cost = cost_for_position(pos);
        if cost < best_cost {
            best_cost = cost;
        }
    }
    best_cost
}

fn parse_comma_separated_ints(path: &str) -> Result<Vec<i64>, Error> {
    Ok(io::BufReader::new(File::open(path).map_err(Error::IO)?)
        .lines()
        .map(|line| -> Result<Vec<i64>, Error> {
            line.map_err(Error::IO)?
                .split(",")
                .map(|s| s.parse::<i64>().map_err(Error::ParseInt))
                .collect::<Result<Vec<i64>, Error>>()
        })
        .collect::<Result<Vec<Vec<i64>>, Error>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<i64>>())
}

mod tests {
    use super::fuel_cost_exponential;
    use super::fuel_cost_linear;

    #[test]
    fn test_fuel_cost_linear() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(fuel_cost_linear(&input), 37);
    }

    #[test]
    fn test_fuel_cost_exponential() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(fuel_cost_exponential(&input), 168);
    }
}
