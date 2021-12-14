use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),
    Parse,
}

fn main() {
    match part_one("1.txt") {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok((id, minutes)) => println!("Run part one: {:?} {:?} {:?}", id, minutes, id * minutes),
    }

    match part_two("1.txt") {
        Err(e) => println!("Run part two error: {:?}", e),
        Ok(timestamp) => println!("Run part one: {:?}", timestamp),
    }
}

fn part_one(path: &str) -> Result<(i64, i64), Error> {
    let (earliest_start, bus_ids_and_indexes) = open_file(&path)?;
    let bus_ids: Vec<i64> = bus_ids_and_indexes.iter().map(|(id, _)| *id).collect();

    let mut earliest_bus_time = -1;
    let mut earliest_bus_id = -1;

    for id in bus_ids {
        let start_offset = earliest_start % id;
        let next_bus = id - start_offset;

        if earliest_bus_time == -1 {
            earliest_bus_id = id;
            earliest_bus_time = next_bus;
        } else if next_bus < earliest_bus_time {
            earliest_bus_id = id;
            earliest_bus_time = next_bus;
        }
    }

    Ok((earliest_bus_id, earliest_bus_time))
}

fn part_two(path: &str) -> Result<i64, Error> {
    let (_, bus_ids) = open_file(&path)?;

    Ok(find_timestamp_quick(bus_ids))
}

fn find_timestamp(buses_ref: Vec<(i64, i64)>) -> i64 {
    let mut buses = buses_ref.clone();
    buses.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let (last_id, last_offset) = buses.pop().unwrap();
    buses = buses.iter().rev().map(|(a, b)| (*a, *b)).collect();

    let mut timestamp = last_id - last_offset;
    loop {
        let found = buses
            .iter()
            .all(|(id, offset)| (timestamp + offset) % id == 0);

        if found {
            return timestamp;
        }

        timestamp += last_id;
    }
}

fn find_timestamp_quick(buses_ref: Vec<(i64, i64)>) -> i64 {
    if buses_ref.len() == 2 {
        return find_timestamp(buses_ref);
    }

    let mut buses = buses_ref.clone();
    buses.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let timestamp_offset = find_timestamp_quick(
        buses[0..buses.len() - 1]
            .iter()
            .map(|(id, offset)| (*id, *offset))
            .collect(),
    );

    let timestamp_multiplier = buses[0..buses.len() - 1]
        .iter()
        .fold(1, |s, (id, _)| s * id);

    let mut timestamp = timestamp_offset;
    loop {
        let found = buses
            .iter()
            .all(|(id, offset)| (timestamp + offset) % id == 0);

        if found {
            return timestamp;
        }

        timestamp += timestamp_multiplier;
    }
}

fn open_file(filename: &str) -> Result<(i64, Vec<(i64, i64)>), Error> {
    let lines: Vec<String> = io::BufReader::new(File::open(filename).map_err(Error::IO)?)
        .lines()
        .map(|maybe_line| -> Result<String, Error> { maybe_line.map_err(Error::IO) })
        .into_iter()
        .collect::<Result<Vec<String>, Error>>()?;

    if lines.len() != 2 {
        return Err(Error::Parse);
    }

    let earliest_start = &lines[0].parse::<i64>().map_err(Error::ParseInt)?;
    let buses_ids = (&lines[1])
        .split(",")
        .enumerate()
        .filter(|(_, id_str)| *id_str != "x")
        .map(|(idx, id_str)| -> Result<(i64, i64), Error> {
            Ok((id_str.parse::<i64>().map_err(Error::ParseInt)?, idx as i64))
        })
        .into_iter()
        .collect::<Result<Vec<(i64, i64)>, Error>>()?;

    Ok((*earliest_start, buses_ids))
}

#[cfg(test)]
mod tests {
    use super::find_timestamp;
    use super::find_timestamp_quick;
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_part_one() {
        let test_input = "1_test.txt";
        let ret = part_one(&test_input);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), (59, 5));
    }

    #[test]
    fn test_part_two() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), 1068781);
    }

    #[test]
    fn test_find_timestamp() {
        assert_eq!(find_timestamp_quick(vec![(17, 0), (13, 2), (19, 3)]), 3417);
        assert_eq!(
            find_timestamp(vec![(67, 0), (7, 1), (59, 2), (61, 3)]),
            754018
        );
        assert_eq!(
            find_timestamp(vec![(67, 0), (7, 2), (59, 3), (61, 4)]),
            779210
        );
        assert_eq!(
            find_timestamp(vec![(67, 0), (7, 1), (59, 3), (61, 4)]),
            1261476
        );
        assert_eq!(
            find_timestamp(vec![(1789, 0), (37, 1), (47, 2), (1889, 3)]),
            1202161486
        );
    }
}

// -41
