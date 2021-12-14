use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    ParseSeat,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Square {
    Empty,
    Occupied,
    Floor,
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

fn visible_seats(y: usize, x: usize, seats: &Vec<Vec<Square>>) -> (usize, usize) {
    let y_max = seats.len() as i64;
    let x_max = seats[x].len() as i64;
    let max = if y_max > x_max { y_max } else { x_max };

    let mut neighbour_offsets: Vec<(i64, i64)> = Vec::new();
    for x in [-1, 0, 1] {
        for y in [-1, 0, 1] {
            if !(x == 0 && y == 0) {
                neighbour_offsets.push((x, y));
            }
        }
    }

    let mut visible_occupied_seats = 0;
    let mut visible_empty_seats = 0;

    for (dy, dx) in neighbour_offsets {
        for i in 1..max {
            let (ny, nx) = (y as i64 + i * dy, x as i64 + i * dx);
            if ny < 0 || ny >= y_max || nx < 0 || nx >= x_max {
                // reached border
                break;
            }

            let sq = seats[ny as usize][nx as usize];
            if sq == Square::Occupied {
                visible_occupied_seats += 1;
                break;
            } else if sq == Square::Empty {
                visible_empty_seats += 1;
                break;
            }
        }
    }
    (visible_occupied_seats, visible_empty_seats)
}

fn apply_rules(area: &Vec<Vec<Square>>) -> (Vec<Vec<Square>>, usize) {
    let mut updated_area: Vec<Vec<Square>> = Vec::new();
    let mut changes = 0;

    let mut neighbour_offsets: Vec<(i64, i64)> = Vec::new();
    for x in [-1, 0, 1] {
        for y in [-1, 0, 1] {
            if !(x == 0 && y == 0) {
                neighbour_offsets.push((x, y));
            }
        }
    }

    for y in 0..area.len() {
        let mut row: Vec<Square> = Vec::new();
        for x in 0..area[y].len() {
            row.push(if area[y][x] == Square::Empty {
                let mut occupied_adjacent_seats = false;
                for (dy, dx) in &neighbour_offsets {
                    let nx = x as i64 + dx;
                    let ny = y as i64 + dy;
                    if ny >= 0 && ny < area.len() as i64 && nx >= 0 && nx < area[y].len() as i64 {
                        if area[ny as usize][nx as usize] == Square::Occupied {
                            occupied_adjacent_seats = true;
                            break;
                        }
                    }
                }

                // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
                if !occupied_adjacent_seats {
                    changes += 1;
                    Square::Occupied
                } else {
                    Square::Empty
                }
            } else if area[y][x] == Square::Occupied {
                let mut occupied_adjacent_seats = 0;

                for (dy, dx) in &neighbour_offsets {
                    let (ny, nx) = (y as i64 + dy, x as i64 + dx);
                    if ny >= 0 && ny < area.len() as i64 && nx >= 0 && nx < area[y].len() as i64 {
                        if area[ny as usize][nx as usize] == Square::Occupied {
                            occupied_adjacent_seats += 1;

                            // no need to count more than 4
                            if occupied_adjacent_seats > 4 {
                                break;
                            }
                        }
                    }
                }

                // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
                if occupied_adjacent_seats >= 4 {
                    changes += 1;
                    Square::Empty
                } else {
                    Square::Occupied
                }
            } else {
                Square::Floor
            })
        }
        updated_area.push(row);
    }

    (updated_area, changes)
}

fn apply_rules_2(area: &Vec<Vec<Square>>) -> (Vec<Vec<Square>>, usize) {
    let mut updated_area: Vec<Vec<Square>> = Vec::new();
    let mut changes = 0;

    let mut neighbour_offsets: Vec<(i64, i64)> = Vec::new();
    for x in [-1, 0, 1] {
        for y in [-1, 0, 1] {
            if !(x == 0 && y == 0) {
                neighbour_offsets.push((x, y));
            }
        }
    }

    for y in 0..area.len() {
        let mut row: Vec<Square> = Vec::new();
        for x in 0..area[y].len() {
            if area[y][x] == Square::Floor {
                row.push(Square::Floor);
                continue;
            }

            let (visible_occupied_seats, _) = visible_seats(y, x, area);

            if area[y][x] == Square::Empty {
                if visible_occupied_seats == 0 {
                    row.push(Square::Occupied);
                    changes += 1;
                } else {
                    row.push(Square::Empty);
                }
            } else if area[y][x] == Square::Occupied {
                if visible_occupied_seats >= 5 {
                    row.push(Square::Empty);
                    changes += 1;
                } else {
                    row.push(Square::Occupied);
                }
            }
        }
        updated_area.push(row);
    }

    (updated_area, changes)
}

fn part_one(path: &str) -> Result<usize, Error> {
    let mut area = open_file(path)?;

    loop {
        let ret = apply_rules(&area);
        area = ret.0;
        if ret.1 == 0 {
            break;
        }
    }

    let mut occupied_seats = 0;
    for row in area.iter() {
        for &s in row.iter() {
            if s == Square::Occupied {
                occupied_seats += 1;
            }
        }
    }

    Ok(occupied_seats)
}

fn part_two(path: &str) -> Result<usize, Error> {
    let mut area = open_file(path)?;

    loop {
        let ret = apply_rules_2(&area);
        area = ret.0;
        if ret.1 == 0 {
            break;
        }
    }

    let mut occupied_seats = 0;
    for row in area.iter() {
        for &s in row.iter() {
            if s == Square::Occupied {
                occupied_seats += 1;
            }
        }
    }

    Ok(occupied_seats)
}

fn open_file(filename: &str) -> Result<Vec<Vec<Square>>, Error> {
    io::BufReader::new(File::open(filename).map_err(Error::IOError)?)
        .lines()
        .map(|maybe_line| -> Result<Vec<Square>, Error> {
            let line = maybe_line.map_err(Error::IOError)?;

            line.chars()
                .map(|c| match c {
                    'L' => Ok(Square::Empty),
                    '.' => Ok(Square::Floor),
                    '#' => Ok(Square::Occupied),
                    _ => Err(Error::ParseSeat),
                })
                .into_iter()
                .collect()
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
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 37);
    }

    #[test]
    fn test_part_two() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 26);
    }
}
