use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    ParseInt(char),
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
    let mut input = parse_input(path)?;

    let max_y = input.len() - 1;
    let max_x = input[0].len() - 1;

    let mut neighbour_offsets: Vec<(i64, i64)> = Vec::new();

    for y in [-1, 0, 1] {
        for x in [-1, 0, 1] {
            if !(y == 0 && x == 0) {
                neighbour_offsets.push((y, x));
            }
        }
    }

    let mut total_flashes = 0;

    // model 100 steps
    for step in 0..100 {
        // increase energy level of all octopuses by 1
        // create set of flashing octopuses
        let mut flashing: HashSet<(usize, usize)> = HashSet::new();
        for y in 0..=max_y {
            for x in 0..=max_x {
                input[y][x] += 1;
                if input[y][x] > 9 {
                    flashing.insert((y, x));
                }
            }
        }

        // iterate through flashing octopuses incrementing neighbours
        // until no new flashing octopuses
        let mut edge: HashSet<(usize, usize)> = HashSet::new();
        edge.extend(flashing.clone());
        loop {
            let mut next_edge: HashSet<(usize, usize)> = HashSet::new();
            for &(fy, fx) in edge.iter() {
                for &(dy, dx) in neighbour_offsets.iter() {
                    let y = fy as i64 + dy;
                    let x = fx as i64 + dx;
                    if y >= 0
                        && y <= max_y as i64
                        && x >= 0
                        && x <= max_x as i64
                        && !flashing.contains(&(y as usize, x as usize))
                        && !next_edge.contains(&(y as usize, x as usize))
                    {
                        input[y as usize][x as usize] += 1;
                        if input[y as usize][x as usize] > 9 {
                            next_edge.insert((y as usize, x as usize));
                        }
                    }
                }
            }
            if next_edge.len() == 0 {
                break;
            }
            flashing.extend(next_edge.clone());
            edge = next_edge;
        }

        for &(y, x) in flashing.iter() {
            input[y][x] = 0
        }

        total_flashes += flashing.len();
    }

    Ok(total_flashes as i64)
}

fn part_two(path: &str) -> Result<i64, Error> {
    let mut input = parse_input(path)?;

    let max_y = input.len() - 1;
    let max_x = input[0].len() - 1;

    let mut neighbour_offsets: Vec<(i64, i64)> = Vec::new();

    for y in [-1, 0, 1] {
        for x in [-1, 0, 1] {
            if !(y == 0 && x == 0) {
                neighbour_offsets.push((y, x));
            }
        }
    }

    let mut step = 1;
    loop {
        // increase energy level of all octopuses by 1
        // create set of flashing octopuses
        let mut flashing: HashSet<(usize, usize)> = HashSet::new();
        for y in 0..=max_y {
            for x in 0..=max_x {
                input[y][x] += 1;
                if input[y][x] > 9 {
                    flashing.insert((y, x));
                }
            }
        }

        // iterate through flashing octopuses incrementing neighbours
        // until no new flashing octopuses
        let mut edge: HashSet<(usize, usize)> = HashSet::new();
        edge.extend(flashing.clone());
        loop {
            let mut next_edge: HashSet<(usize, usize)> = HashSet::new();
            for &(fy, fx) in edge.iter() {
                for &(dy, dx) in neighbour_offsets.iter() {
                    let y = fy as i64 + dy;
                    let x = fx as i64 + dx;
                    if y >= 0
                        && y <= max_y as i64
                        && x >= 0
                        && x <= max_x as i64
                        && !flashing.contains(&(y as usize, x as usize))
                        && !next_edge.contains(&(y as usize, x as usize))
                    {
                        input[y as usize][x as usize] += 1;
                        if input[y as usize][x as usize] > 9 {
                            next_edge.insert((y as usize, x as usize));
                        }
                    }
                }
            }
            if next_edge.len() == 0 {
                break;
            }
            flashing.extend(next_edge.clone());
            edge = next_edge;
        }

        for &(y, x) in flashing.iter() {
            input[y][x] = 0
        }

        if flashing.len() == (max_y + 1) * (max_x + 1) {
            return Ok(step);
        }
        step += 1;
    }
}

fn parse_input(path: &str) -> Result<Vec<Vec<i64>>, Error> {
    io::BufReader::new(File::open(path).map_err(Error::IO)?)
        .lines()
        .map(|maybe_line| -> Result<Vec<i64>, Error> {
            let line = maybe_line.map_err(Error::IO)?;

            line.chars()
                .filter(|&c| c >= '0' && c <= '9')
                .map(|c| c.to_digit(10).ok_or(Error::ParseInt(c)).map(|d| d as i64))
                .into_iter()
                .collect::<Result<Vec<i64>, Error>>()
        })
        .collect::<Result<Vec<Vec<i64>>, Error>>()
}

mod tests {
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_part_one() {
        let res = part_one("1_test.txt");
        assert_eq!(res.unwrap(), 1656);
    }

    #[test]
    fn test_part_two() {
        let res = part_two("1_test.txt");
        assert_eq!(res.unwrap(), 195);
    }
}
