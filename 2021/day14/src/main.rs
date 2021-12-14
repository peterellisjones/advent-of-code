use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    Parse(String),
}

#[derive(Debug)]
struct Input {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

fn main() {
    match part_one("input_1.txt", 10) {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(res) => println!("Run part one: {:?}", res),
    }

    match part_one("input_1.txt", 40) {
        Err(e) => println!("Run part two error: {:?}", e),
        Ok(res) => println!("Run part two: {:?}", res),
    }
}

fn part_one(path: &str, steps: usize) -> Result<usize, Error> {
    let input = parse_input(path)?;

    let mut counts: HashMap<char, usize> = HashMap::new();
    for i in 0..input.template.len() {
        *counts.entry(input.template[i]).or_insert(0) += 1;
    }

    let mut polymer_pairs: HashMap<(char, char), usize> = HashMap::new();
    for i in 0..input.template.len() - 1 {
        let a = input.template[i];
        let b = input.template[i + 1];
        *polymer_pairs.entry((a, b)).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut updated_polymer_pairs: HashMap<(char, char), usize> = HashMap::new();

        for (pair, count) in polymer_pairs.iter() {
            let insertion = input.rules.get(pair).unwrap();

            *updated_polymer_pairs
                .entry((pair.0, *insertion))
                .or_insert(0) += count;
            *updated_polymer_pairs
                .entry((*insertion, pair.1))
                .or_insert(0) += count;
            *counts.entry(*insertion).or_insert(0) += count;
        }

        polymer_pairs = updated_polymer_pairs;
    }

    let least_common_count = counts.iter().map(|(_, v)| v).min().unwrap();
    let most_common_count = counts.iter().map(|(_, v)| v).max().unwrap();

    Ok(most_common_count - least_common_count)
}

fn parse_input(path: &str) -> Result<Input, Error> {
    let mut contents = String::new();
    File::open(path)
        .map_err(Error::IO)?
        .read_to_string(&mut contents)
        .map_err(Error::IO)?;

    let (top, bottom) = contents
        .split_once("\r\n\r\n")
        .ok_or(Error::Parse(contents.to_string()))?;

    Ok(Input {
        template: top.chars().collect(),
        rules: bottom
            .split("\r\n")
            .map(|line| -> ((char, char), char) {
                let chars: Vec<char> = line.chars().collect();

                ((chars[0], chars[1]), chars[6])
            })
            .into_iter()
            .collect::<HashMap<(char, char), char>>(),
    })
}

mod tests {
    use super::part_one;

    #[test]
    fn test_part_one() {
        let res = part_one("test_1.txt", 10);
        assert_eq!(res.unwrap(), 1588);
    }
}
