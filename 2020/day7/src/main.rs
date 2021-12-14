use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    ParseLine,
    ParseInnerBags,
    RegexError(regex::Error),
    ParseIntError(std::num::ParseIntError),
}

fn main() {
    match part_one("1.txt") {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(count) => println!("Run part one: {:?}", count),
    }

    match part_two("1.txt") {
        Err(e) => println!("Run part_two error: {:?}", e),
        Ok(count) => println!("Run part_two: {:?}", count),
    }
}

fn part_one(path: &str) -> Result<usize, Error> {
    let rules = open_file(path)?;
    let inverted_rules = invert_rules(rules);
    let bags = possible_outer_bags("shiny gold", &inverted_rules);

    Ok(bags.len())
}

fn part_two(path: &str) -> Result<usize, Error> {
    let rules = open_file(path)?;
    let count = count_inner_bags("shiny gold", &rules);

    Ok(count)
}

fn count_inner_bags(outer_bag: &str, rules: &HashMap<String, Vec<(String, usize)>>) -> usize {
    if !rules.contains_key(outer_bag) {
        return 0;
    }

    rules[outer_bag]
        .iter()
        .fold(0, |c, (inner_bag_color, inner_bag_count)| {
            c + inner_bag_count + inner_bag_count * count_inner_bags(inner_bag_color, rules)
        })
}

fn possible_outer_bags(
    inner_bag: &str,
    inverted_rules: &HashMap<String, Vec<(String, usize)>>,
) -> HashSet<String> {
    let mut candidates: HashSet<String> = HashSet::new();

    if inverted_rules.contains_key(inner_bag) {
        for (outer_bag_color, _) in &inverted_rules[inner_bag] {
            candidates.insert(outer_bag_color.clone());
            let others = possible_outer_bags(outer_bag_color, inverted_rules);
            let uni = candidates.union(&others).cloned();
            candidates = HashSet::from_iter(uni);
        }
    }
    candidates
}

fn invert_rules(
    bags_can_contain_rules: HashMap<String, Vec<(String, usize)>>,
) -> HashMap<String, Vec<(String, usize)>> {
    let mut bags_can_be_contained_by_rules: HashMap<String, Vec<(String, usize)>> = HashMap::new();

    for (outer_bag, inner_bags) in bags_can_contain_rules {
        for (inner_bag_color, inner_bag_count) in inner_bags {
            bags_can_be_contained_by_rules
                .entry(inner_bag_color)
                .or_insert(Vec::new())
                .push((outer_bag.clone(), inner_bag_count));
        }
    }

    bags_can_be_contained_by_rules
}

fn open_file(filename: &str) -> Result<HashMap<String, Vec<(String, usize)>>, Error> {
    let mut bags_can_contain_rules: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    let lines = io::BufReader::new(File::open(filename).map_err(Error::IOError)?).lines();
    let rgx = Regex::new(r"([0-9]+) ([a-z]+ [a-z]+) bag").map_err(Error::RegexError)?;
    for maybe_line in lines {
        let line = maybe_line.map_err(Error::IOError)?;
        let mut inner_bags: Vec<(String, usize)> = Vec::new();
        let (outer_bag, inner_bags_str) =
            line.split_once(" bags contain ").ok_or(Error::ParseLine)?;

        if inner_bags_str != "no other bags." {
            for inner_bag_str in inner_bags_str.split(", ") {
                for cap in rgx.captures(&inner_bag_str).iter() {
                    inner_bags.push((
                        match cap.get(2) {
                            Some(s) => s.as_str().to_owned(),
                            _ => return Err(Error::ParseInnerBags),
                        },
                        match cap.get(1) {
                            Some(s) => s.as_str().parse::<usize>().map_err(Error::ParseIntError)?,
                            _ => return Err(Error::ParseInnerBags),
                        },
                    ));
                }
            }
        }

        bags_can_contain_rules.insert(outer_bag.to_owned(), inner_bags);
    }

    Ok(bags_can_contain_rules)
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
        assert_eq!(ret.unwrap(), 4);
    }

    #[test]
    fn test_part_two() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 32);
    }
}
