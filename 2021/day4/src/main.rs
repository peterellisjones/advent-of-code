use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),
    Parse(String),
    NotImplemented,
    WinerNotFound,
}

#[derive(Debug)]
struct Board([[i64; 5]; 5]);

fn main() {
    // 64124 -> too high
    match part_one("1.txt") {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(final_score) => println!("Run part one: {:?}", final_score),
    }
    match part_two("1.txt") {
        Err(e) => println!("Run part two error: {:?}", e),
        Ok(final_score) => println!("Run part two: {:?}", final_score),
    }
}

fn part_one(path: &str) -> Result<i64, Error> {
    let (numbers, mut boards) = open_file(&path)?;
    for n in numbers.iter() {
        for board in &mut boards {
            let is_winner = board.remove_number(*n);
            if is_winner {
                return Ok(board.umarked_sum() * n);
            }
        }
    }

    Err(Error::WinerNotFound)
}

fn part_two(path: &str) -> Result<i64, Error> {
    let (numbers, mut boards) = open_file(&path)?;
    for n in numbers.iter() {
        let mut boards_to_remove: Vec<usize> = Vec::new();
        let remaining = boards.len();
        for b_idx in 0..remaining {
            let board = &mut boards[b_idx];
            let is_winner = board.remove_number(*n);

            if is_winner {
                boards_to_remove.push(b_idx);
                if remaining == boards_to_remove.len() {
                    return Ok(board.umarked_sum() * n);
                }
            }
        }

        for idx in boards_to_remove.iter().rev() {
            boards.remove(*idx);
        }
    }

    Err(Error::WinerNotFound)
}

impl Board {
    fn umarked_sum(&self) -> i64 {
        let mut s = 0;
        for i in 0..5 {
            for j in 0..5 {
                if self.0[i][j] != -1 {
                    s += self.0[i][j];
                }
            }
        }
        s
    }

    fn remove_number(&mut self, n: i64) -> bool {
        for i in 0..5 {
            for j in 0..5 {
                if n == self.0[i][j] {
                    self.0[i][j] = -1;
                    return self.row_complete(i) || self.col_complete(j);
                }
            }
        }

        false
    }

    fn row_complete(&self, i: usize) -> bool {
        for j in 0..5 {
            if self.0[i][j] != -1 {
                return false;
            }
        }
        true
    }

    fn col_complete(&self, j: usize) -> bool {
        for i in 0..5 {
            if self.0[i][j] != -1 {
                return false;
            }
        }
        true
    }
}

fn open_file(filename: &str) -> Result<(Vec<i64>, Vec<Board>), Error> {
    let mut contents = String::new();
    File::open(filename)
        .map_err(Error::IO)?
        .read_to_string(&mut contents)
        .map_err(Error::IO)?;

    let parts: Vec<&str> = contents.split("\r\n\r\n").collect();
    if parts.len() < 2 {
        return Err(Error::Parse(format!(
            "wrong number of parts: {}",
            parts.len(),
        )));
    }

    let numbers = parse_comma_separated_nums(parts[0])?;
    let mut boards: Vec<Board> = Vec::new();

    for part in parts.iter().skip(1) {
        boards.push(parse_board(part)?);
    }

    Ok((numbers, boards))
}

fn parse_board(s: &str) -> Result<Board, Error> {
    let mut board = [[0i64; 5]; 5];
    for (i, row) in s.split("\r\n").enumerate() {
        for (j, c) in row
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| s.len() > 0)
            .enumerate()
        {
            board[i][j] = c.parse::<i64>().map_err(Error::ParseInt)?;
        }
    }

    Ok(Board(board))
}

fn parse_comma_separated_nums(s: &str) -> Result<Vec<i64>, Error> {
    s.split(",")
        .map(|s| s.parse::<i64>().map_err(Error::ParseInt))
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
        assert_eq!(ret.unwrap(), 4512);
    }

    #[test]
    fn test_part_two() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), 1924);
    }
}
