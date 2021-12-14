use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
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
    Ok(open_file(path)?.iter().fold(0, |sum, group| {
        let mut questions: HashSet<char> = HashSet::new();
        group.iter().flatten().for_each(|&question| {
            questions.insert(question);
        });
        sum + questions.len()
    }))
}

fn part_two(path: &str) -> Result<usize, Error> {
    Ok(open_file(path)?.iter().fold(0, |sum, group| {
        let mut questions = HashMap::new();
        group
            .iter()
            .flatten()
            .for_each(|question| *questions.entry(question).or_insert(0) += 1);
        sum + questions.iter().filter(|(_, &v)| v == group.len()).count()
    }))
}

fn open_file(filename: &str) -> Result<Vec<Vec<Vec<char>>>, Error> {
    let mut contents = String::new();
    File::open(filename)
        .map_err(Error::IOError)?
        .read_to_string(&mut contents)
        .map_err(Error::IOError)?;

    contents
        .split("\r\n\r\n")
        .map(|group| -> Result<Vec<Vec<char>>, Error> {
            Ok(group
                .split("\r\n")
                .collect::<Vec<&str>>()
                .iter()
                .map(|p| p.chars().collect())
                .collect())
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
        assert_eq!(ret.unwrap(), 11);
    }

    #[test]
    fn test_part_two() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), 6);
    }
}
