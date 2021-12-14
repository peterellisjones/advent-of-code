use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    RegexError(regex::Error),
    ParseField,
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

fn part_one(path: &str) -> Result<usize, Error> {
    let passports = open_file(&path)?;

    let required_fields: std::collections::HashSet<&str> =
        HashSet::from_iter(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);

    Ok(passports
        .iter()
        .filter(|fields| {
            required_fields
                .iter()
                .all(|&k| fields.contains_key(k as &str))
        })
        .count())
}

fn part_two(path: &str) -> Result<usize, Error> {
    let passports = open_file(&path)?;

    let required_fields: std::collections::HashSet<&str> =
        HashSet::from_iter(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);

    let hcl_rgx = Regex::new(r"^#[0-9a-f]{6}$").map_err(Error::RegexError)?;
    let ecl_rgx = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").map_err(Error::RegexError)?;
    let pid_rgx = Regex::new(r"^[0-9]{9}$").map_err(Error::RegexError)?;
    let hgt_in_regex = Regex::new(r"^[0-9]+in$").map_err(Error::RegexError)?;
    let hgt_cm_regex = Regex::new(r"^[0-9]+cm$").map_err(Error::RegexError)?;
    Ok(passports
        .iter()
        .filter(|fields| {
            required_fields
                .iter()
                .all(|&k| fields.contains_key(k as &str))
        })
        .filter(|fields| {
            let byr = fields.get("byr").unwrap().parse::<i64>().unwrap();
            byr >= 1920 && byr <= 2002
        })
        .filter(|fields| {
            let iyr = fields.get("iyr").unwrap().parse::<i64>().unwrap();
            iyr >= 2010 && iyr <= 2020
        })
        .filter(|fields| {
            let eyr = fields.get("eyr").unwrap().parse::<i64>().unwrap();
            eyr >= 2020 && eyr <= 2030
        })
        .filter(|fields| {
            let hgt = fields.get("hgt").unwrap();
            if hgt_in_regex.is_match(hgt) {
                let hgt_num = hgt[..hgt.len() - 2].parse::<i64>().unwrap();
                hgt_num >= 59 && hgt_num <= 76
            } else if hgt_cm_regex.is_match(hgt) {
                let hgt_num = hgt[..hgt.len() - 2].parse::<i64>().unwrap();
                hgt_num >= 150 && hgt_num <= 193
            } else {
                false
            }
        })
        .filter(|fields| {
            let hcl = fields.get("hcl").unwrap();
            hcl_rgx.is_match(hcl)
        })
        .filter(|fields| {
            let ecl = fields.get("ecl").unwrap();
            ecl_rgx.is_match(ecl)
        })
        .filter(|fields| {
            let pid = fields.get("pid").unwrap();
            pid_rgx.is_match(pid)
        })
        .count())
}

fn open_file(filename: &str) -> Result<Vec<HashMap<String, String>>, Error> {
    let mut contents = String::new();
    File::open(filename)
        .map_err(Error::IOError)?
        .read_to_string(&mut contents)
        .map_err(Error::IOError)?;

    contents
        .split("\r\n\r\n")
        .map(|passport| -> Result<HashMap<String, String>, Error> {
            let mut field_map = HashMap::new();

            for res in passport
                .split_whitespace()
                .map(|field| -> Result<(String, String), Error> {
                    field
                        .split_once(":")
                        .ok_or(Error::ParseField)
                        .map(|(a, b)| (a.to_owned(), b.to_owned()))
                })
                .into_iter()
            {
                let (k, v) = res?;
                field_map.insert(k, v);
            }

            Ok(field_map)
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
        let ret = part_one("1_test.txt");
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), 2);
    }

    #[test]
    fn test_part_two() {
        let test_input = "2_test.txt";
        let ret = part_two(&test_input);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), 4);
    }
}
