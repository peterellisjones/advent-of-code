use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    NotFound,
}

fn main() {
    match part_one("1.txt") {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(a) => println!("Run part one: {:?}", a),
    }

    match part_two("1.txt") {
        Err(e) => println!("Run part two error: {:?}", e),
        Ok(a) => println!("Run part two: {:?}", a),
    }
}
fn part_one(path: &str) -> Result<isize, Error> {
    Ok(open_file(path)?.iter().fold(0, |s, l| {
        let (_, _, seat_id) = seat_id(l);
        if seat_id > s {
            seat_id
        } else {
            s
        }
    }))
}

fn part_two(path: &str) -> Result<isize, Error> {
    let mut seats: Vec<isize> = open_file(path)?.iter().map(|l| seat_id(l).2).collect();
    seats.sort();
    let first = seats[0];
    for (idx, &seat) in seats.iter().enumerate() {
        if first + idx as isize != seat {
            return Ok(first + idx as isize);
        }
    }

    Err(Error::NotFound)
}

fn seat_id(s: &str) -> (isize, isize, isize) {
    let mut num: String = s.to_string();
    num = num.replace("B", "1");
    num = num.replace("F", "0");
    num = num.replace("R", "1");
    num = num.replace("L", "0");
    let seat = isize::from_str_radix(&num, 2).unwrap();
    let row = seat >> 3;
    let col = seat & 7;
    (row, col, seat)
}

fn open_file(filename: &str) -> Result<Vec<String>, Error> {
    io::BufReader::new(File::open(filename).map_err(Error::IOError)?)
        .lines()
        .map(|l| l.map_err(Error::IOError))
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::seat_id;

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id("BFFFBBFRRR"), (70, 7, 567));
        assert_eq!(seat_id("FFFBBBFRRR"), (14, 7, 119));
        assert_eq!(seat_id("BBFFBBFRLL"), (102, 4, 820));
    }
}
