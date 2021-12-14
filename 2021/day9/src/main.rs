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
    let heightmap = parse_input(path)?;

    Ok(get_low_points(&heightmap)
        .iter()
        .map(|(x, y)| heightmap[*x][*y] + 1)
        .sum())
}
fn part_two(path: &str) -> Result<i64, Error> {
    let heightmap = parse_input(path)?;
    let max_x = heightmap[0].len() - 1;
    let max_y = heightmap.len() - 1;
    let low_points = get_low_points(&heightmap);
    let mut basin_sizes: Vec<i64> = Vec::new();

    for (lp_y, lp_x) in low_points {
        // flood-fill starting at low point
        let mut basin: HashSet<(usize, usize)> = HashSet::new();
        basin.insert((lp_y, lp_x));

        loop {
            // add all neighbouring cells to border that
            //   are in bounds
            //   have a value of less than 9
            //   are not already in the basin
            let mut border: HashSet<(usize, usize)> = HashSet::new();
            for &(by, bx) in basin.iter() {
                if by > 0 && !basin.contains(&(by - 1, bx)) && heightmap[by - 1][bx] < 9 {
                    border.insert((by - 1, bx));
                }
                if by < max_y && !basin.contains(&(by + 1, bx)) && heightmap[by + 1][bx] < 9 {
                    border.insert((by + 1, bx));
                }
                if bx > 0 && !basin.contains(&(by, bx - 1)) && heightmap[by][bx - 1] < 9 {
                    border.insert((by, bx - 1));
                }
                if bx < max_x && !basin.contains(&(by, bx + 1)) && heightmap[by][bx + 1] < 9 {
                    border.insert((by, bx + 1));
                }
            }

            if border.len() == 0 {
                break;
            }

            basin.extend(border);
        }

        basin_sizes.push(basin.len() as i64);
    }

    basin_sizes.sort();
    Ok(basin_sizes.iter().rev().take(3).fold(1, |a, s| a * s))
}

fn get_low_points(heightmap: &Vec<Vec<i64>>) -> Vec<(usize, usize)> {
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for (y, row) in heightmap.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let mut lowest_neighbour = val + 1;
            if y > 0 {
                lowest_neighbour = lowest_neighbour.min(heightmap[y - 1][x]);
            }
            if y < heightmap.len() - 1 {
                lowest_neighbour = lowest_neighbour.min(heightmap[y + 1][x]);
            }
            if x > 0 {
                lowest_neighbour = lowest_neighbour.min(heightmap[y][x - 1]);
            }
            if x < row.len() - 1 {
                lowest_neighbour = lowest_neighbour.min(heightmap[y][x + 1]);
            }

            if val < lowest_neighbour {
                low_points.push((y, x));
            }
        }
    }

    low_points
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
        assert_eq!(res.unwrap(), 15);
    }

    #[test]
    fn test_part_two() {
        let res = part_two("1_test.txt");
        assert_eq!(res.unwrap(), 1134);
    }
}
