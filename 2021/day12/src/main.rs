use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    ParseInt(char),
    Parse(String),
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
enum Cave {
    Start,
    End,
    Big([char; 2]),
    Small([char; 2]),
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

fn part_one(path: &str) -> Result<usize, Error> {
    let input = parse_input(path)?;

    Ok(count_paths_to_end(
        &Cave::Start,
        &input,
        true,
        &HashMap::new(),
    ))
}

fn part_two(path: &str) -> Result<usize, Error> {
    let input = parse_input(path)?;

    Ok(count_paths_to_end(
        &Cave::Start,
        &input,
        false,
        &HashMap::new(),
    ))
}

fn count_paths_to_end(
    start: &Cave,
    connections: &HashMap<Cave, HashSet<Cave>>,
    has_already_visited_a_small_cave_twice: bool,
    visits: &HashMap<Cave, usize>,
) -> usize {
    if connections.get(start).is_none() {
        return 0;
    }

    let mut count = 0;
    for next_cave in connections.get(start).unwrap() {
        match next_cave {
            Cave::Small(_) => {
                let mut havasmct = has_already_visited_a_small_cave_twice;
                // if already at max visits, skip
                if visits.get(next_cave) == Some(&2) {
                    continue;
                }

                if visits.get(next_cave) == Some(&1) {
                    if has_already_visited_a_small_cave_twice {
                        continue;
                    }
                    havasmct = true;
                }

                // record visit
                let mut updated_visits = visits.clone();
                *updated_visits.entry(*next_cave).or_insert(0) += 1;

                count += count_paths_to_end(next_cave, connections, havasmct, &updated_visits);
            }
            Cave::Big(_) => {
                count += count_paths_to_end(
                    next_cave,
                    connections,
                    has_already_visited_a_small_cave_twice,
                    visits,
                );
            }
            Cave::Start => unreachable!(),
            Cave::End => {
                count += 1;
                continue;
            }
        }
    }

    count
}

fn parse_input(path: &str) -> Result<HashMap<Cave, HashSet<Cave>>, Error> {
    let mut caves: HashMap<Cave, HashSet<Cave>> = HashMap::new();
    for res in io::BufReader::new(File::open(path).map_err(Error::IO)?)
        .lines()
        .map(|maybe_line| -> Result<(Cave, Cave), Error> {
            let line = maybe_line.map_err(Error::IO)?;
            let (left, right) = line.split_once("-").ok_or(Error::Parse(line.to_string()))?;

            let parse = |part: &str| -> Result<Cave, Error> {
                let c1 = part.chars().next().unwrap();
                let c2 = part.chars().skip(1).next().unwrap_or(' ');
                match part {
                    "start" => Ok(Cave::Start),
                    "end" => Ok(Cave::End),
                    _ if part.to_uppercase() == part => Ok(Cave::Big([c1, c2])),
                    _ if part.to_lowercase() == part => Ok(Cave::Small([c1, c2])),
                    _ => Err(Error::Parse(part.to_string())),
                }
            };

            let from = parse(left)?;
            let to = parse(right)?;

            Ok((from, to))
        })
        .into_iter()
    {
        let (from, to) = res?;

        // insert from => to
        if to != Cave::Start && from != Cave::End {
            (*caves.entry(from).or_insert(HashSet::new())).insert(to);
        }

        if to != Cave::End && from != Cave::Start {
            // insert reverse relation
            (*caves.entry(to).or_insert(HashSet::new())).insert(from);
        }
    }

    Ok(caves)
}

mod tests {
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_part_one() {
        let res = part_one("1_test.txt");
        assert_eq!(res.unwrap(), 10);
        let res = part_one("2_test.txt");
        assert_eq!(res.unwrap(), 19);
        let res = part_one("3_test.txt");
        assert_eq!(res.unwrap(), 226);
    }

    #[test]
    fn test_part_two() {
        let res = part_two("1_test.txt");
        assert_eq!(res.unwrap(), 36);
        let res = part_two("2_test.txt");
        assert_eq!(res.unwrap(), 103);
        let res = part_two("3_test.txt");
        assert_eq!(res.unwrap(), 3509);
    }
}
