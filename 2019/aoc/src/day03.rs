use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
enum Segment {
  Right(usize),
  Left(usize),
  Up(usize),
  Down(usize),
}

impl Segment {
  fn traverse(&self) -> (i64, i64, usize) {
    match self {
      Segment::Up(n) => (0, 1, *n),
      Segment::Down(n) => (0, -1, *n),
      Segment::Left(n) => (-1, 0, *n),
      Segment::Right(n) => (1, 0, *n),
    }
  }
}

#[derive(Debug)]
pub enum Error {
  ParseInt(std::num::ParseIntError),
  ParseSegment(String),
  IO(std::io::Error),
  NoIntersections,
}

pub fn part01(path: &str) -> Result<i64, Error> {
  let (wire1, wire2) = open_file(path)?;

  closest_intersect(&wire1, &wire2)
}

pub fn part02(path: &str) -> Result<i64, Error> {
  let (wire1, wire2) = open_file(path)?;

  quickest_intersect(&wire1, &wire2)
}

fn closest_intersect(wire1: &Vec<Segment>, wire2: &Vec<Segment>) -> Result<i64, Error> {
  let mut first_wire: HashSet<(i64, i64)> = HashSet::new();
  let mut intersections: HashSet<(i64, i64)> = HashSet::new();

  let mut x = 0;
  let mut y = 0;
  for segment in wire1 {
    let (dx, dy, n) = segment.traverse();
    for _ in 0..n {
      x = x + dx;
      y = y + dy;

      first_wire.insert((x, y));
    }
  }

  x = 0;
  y = 0;

  for segment in wire2 {
    let (dx, dy, n) = segment.traverse();
    for _ in 0..n {
      x = x + dx;
      y = y + dy;
      if first_wire.contains(&(x, y)) {
        intersections.insert((x, y));
      }
    }
  }

  let shortest_distance_to_intersection =
    intersections.iter().map(|(x, y)| x.abs() + y.abs()).min();

  shortest_distance_to_intersection.ok_or(Error::NoIntersections)
}

fn quickest_intersect(wire1: &Vec<Segment>, wire2: &Vec<Segment>) -> Result<i64, Error> {
  let mut first_wire: HashMap<(i64, i64), i64> = HashMap::new();
  let mut intersections: HashMap<(i64, i64), (i64, i64)> = HashMap::new();

  let mut x = 0;
  let mut y = 0;
  let mut distance = 0;

  for segment in wire1 {
    let (dx, dy, n) = segment.traverse();
    for _ in 0..n {
      x = x + dx;
      y = y + dy;
      distance += 1;
      first_wire.entry((x, y)).or_insert(distance);
    }
  }

  x = 0;
  y = 0;
  distance = 0;

  for segment in wire2 {
    let (dx, dy, n) = segment.traverse();
    for _ in 0..n {
      x = x + dx;
      y = y + dy;
      distance += 1;

      if first_wire.contains_key(&(x, y)) {
        let first_wire_distance = first_wire.get(&(x, y)).unwrap();
        intersections.insert((x, y), (*first_wire_distance, distance));
      }
    }
  }

  let quickest_path_to_intersection = intersections.iter().map(|(_, (w1, w2))| w1 + w2).min();

  quickest_path_to_intersection.ok_or(Error::NoIntersections)
}

fn open_file(path: &str) -> Result<(Vec<Segment>, Vec<Segment>), Error> {
  let lines: Vec<Vec<Segment>> = io::BufReader::new(File::open(path).map_err(Error::IO)?)
    .lines()
    .map(|maybe_line| {
      let line = maybe_line.map_err(Error::IO)?;
      parse_line(&line)
    })
    .into_iter()
    .collect::<Result<Vec<Vec<Segment>>, Error>>()?;

  Ok((lines[0].clone(), lines[1].clone()))
}

fn parse_line(line: &str) -> Result<Vec<Segment>, Error> {
  line
    .split(",")
    .map(|s| {
      if s.starts_with("R") {
        Ok(Segment::Right(
          s[1..].parse::<usize>().map_err(Error::ParseInt)?,
        ))
      } else if s.starts_with("L") {
        Ok(Segment::Left(
          s[1..].parse::<usize>().map_err(Error::ParseInt)?,
        ))
      } else if s.starts_with("U") {
        Ok(Segment::Up(
          s[1..].parse::<usize>().map_err(Error::ParseInt)?,
        ))
      } else if s.starts_with("D") {
        Ok(Segment::Down(
          s[1..].parse::<usize>().map_err(Error::ParseInt)?,
        ))
      } else {
        Err(Error::ParseSegment(s.to_string()))
      }
    })
    .collect::<Result<Vec<Segment>, Error>>()
}

mod tests {
  use super::closest_intersect;
  use super::parse_line;
  use super::quickest_intersect;

  #[test]
  fn test_closest_intersect1() {
    let wire1 = parse_line("R8,U5,L5,D3").unwrap();
    let wire2 = parse_line("U7,R6,D4,L4").unwrap();
    assert_eq!(closest_intersect(&wire1, &wire2).unwrap(), 6);
  }

  #[test]
  fn test_closest_intersect2() {
    let wire1 = parse_line("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap();
    let wire2 = parse_line("U62,R66,U55,R34,D71,R55,D58,R83").unwrap();
    assert_eq!(closest_intersect(&wire1, &wire2).unwrap(), 159);
  }

  #[test]
  fn test_closest_intersect3() {
    let wire1 = parse_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap();
    let wire2 = parse_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap();
    assert_eq!(closest_intersect(&wire1, &wire2).unwrap(), 135);
  }

  #[test]
  fn test_quickest_intersect1() {
    let wire1 = parse_line("R8,U5,L5,D3").unwrap();
    let wire2 = parse_line("U7,R6,D4,L4").unwrap();
    assert_eq!(quickest_intersect(&wire1, &wire2).unwrap(), 30);
  }

  #[test]
  fn test_quickest_intersect2() {
    let wire1 = parse_line("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap();
    let wire2 = parse_line("U62,R66,U55,R34,D71,R55,D58,R83").unwrap();
    assert_eq!(quickest_intersect(&wire1, &wire2).unwrap(), 610);
  }

  #[test]
  fn test_quickest_intersect3() {
    let wire1 = parse_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap();
    let wire2 = parse_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap();
    assert_eq!(quickest_intersect(&wire1, &wire2).unwrap(), 410);
  }
}
