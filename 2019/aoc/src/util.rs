use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub enum Error {
  IO(std::io::Error),
  ParseInt(std::num::ParseIntError),
}

pub fn parse_int_per_line(path: &str) -> Result<Vec<i64>, Error> {
  io::BufReader::new(File::open(path).map_err(Error::IO)?)
    .lines()
    .map(|maybe_line| -> Result<i64, Error> {
      let line = maybe_line.map_err(Error::IO)?;
      line.parse::<i64>().map_err(Error::ParseInt)
    })
    .into_iter()
    .collect()
}

pub fn parse_comma_separated_ints(path: &str) -> Result<Vec<i64>, Error> {
  Ok(
    io::BufReader::new(File::open(path).map_err(Error::IO)?)
      .lines()
      .map(|line| -> Result<Vec<i64>, Error> {
        line
          .map_err(Error::IO)?
          .split(",")
          .map(|s| s.parse::<i64>().map_err(Error::ParseInt))
          .collect::<Result<Vec<i64>, Error>>()
      })
      .collect::<Result<Vec<Vec<i64>>, Error>>()?
      .into_iter()
      .flatten()
      .collect::<Vec<i64>>(),
  )
}
