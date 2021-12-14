use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),
    Parse(String),
    Regex(regex::Error),
    NotImplemented,
}

#[derive(Debug)]
struct Info {
    my_ticket: Vec<i64>,
    nearby_tickets: Vec<Vec<i64>>,
    fields: Vec<Field>,
}

#[derive(Debug, Clone)]
struct Field {
    name: String,
    low_range_min: i64,
    low_range_max: i64,
    high_range_min: i64,
    high_range_max: i64,
}

fn main() {
    match part_one("1.txt") {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(err_rate) => println!("Run part one: {:?}", err_rate),
    }
    match part_two("1.txt") {
        Err(e) => println!("Run part two error: {:?}", e),
        Ok(err_rate) => println!("Run part two: {:?}", err_rate),
    }
}

fn part_one(path: &str) -> Result<i64, Error> {
    let info = &open_file(&path)?;

    let mut invalid_values: Vec<i64> = Vec::new();

    for ticket in info.nearby_tickets.iter() {
        for &val in ticket.iter() {
            let mut is_valid = false;
            for field in info.fields.iter() {
                if (val >= field.low_range_min && val <= field.low_range_max)
                    || (val >= field.high_range_min && val <= field.high_range_max)
                {
                    is_valid = true;
                    break;
                }
            }
            if !is_valid {
                invalid_values.push(val);
            }
        }
    }

    Ok(invalid_values.iter().fold(0, |s, v| s + v))
}

fn part_two(path: &str) -> Result<i64, Error> {
    let info = &open_file(&path)?;

    let valid_tickets: Vec<&Vec<i64>> = info
        .nearby_tickets
        .iter()
        .filter(|t| ticket_is_valid(t, &info.fields))
        .collect();

    let ticket_index_count = info.nearby_tickets[0].len();

    // maps field names to possible indexes for that field
    let mut possible_field_positions: HashMap<String, HashSet<usize>> = HashMap::new();
    let mut unique_field_positions: HashMap<String, usize> = HashMap::new();

    // STEP 1: FIND POSSIBLE INDEXES FOR EACH FIELD
    for field in &info.fields {
        for idx in 0..ticket_index_count {
            let mut valid_for_all_tickets = true;
            for ticket in &valid_tickets {
                let val = ticket[idx];
                let valid_for_this_ticket = (val >= field.low_range_min
                    && val <= field.low_range_max)
                    || (val >= field.high_range_min && val <= field.high_range_max);

                if !valid_for_this_ticket {
                    valid_for_all_tickets = false;
                    break;
                }
            }
            if valid_for_all_tickets {
                possible_field_positions
                    .entry(field.name.to_string())
                    .or_insert(HashSet::new())
                    .insert(idx);
            }
        }
    }

    // STEP 2: FOR EACH INDEX UNIQUELY ASSOCIATED WITH A FIELD,
    // REMOVE THAT INDEX FROM OTHER FIELDS
    loop {
        if unique_field_positions.len() == info.fields.len() {
            break;
        }
        // Remove all unique indexes from possible field possitions list
        for (_, idx) in &unique_field_positions {
            for (_, possible_indexes) in &mut possible_field_positions {
                possible_indexes.remove(idx);
            }
        }

        // If there is only one possible index for any field
        // then add to list of found positions
        for (field_name, indexes) in &possible_field_positions {
            if unique_field_positions.contains_key(field_name) {
                continue;
            }
            if indexes.len() == 1 {
                let idx = indexes.iter().next().unwrap();
                unique_field_positions.insert(field_name.to_string(), *idx);
            }
        }
    }

    let mut my_ticket_values: HashMap<String, i64> = HashMap::new();

    for (field_name, idx) in &unique_field_positions {
        let value = info.my_ticket[*idx as usize];
        my_ticket_values.insert(field_name.to_string(), value);
    }

    let departure_mult = my_ticket_values
        .iter()
        .filter(|(k, _)| k.contains("departure"))
        .fold(1, |s, (_, v)| s * v);

    Ok(departure_mult)
}

fn ticket_is_valid(ticket: &Vec<i64>, fields: &Vec<Field>) -> bool {
    for &val in ticket.iter() {
        let mut value_is_valid = false;
        for field in fields.iter() {
            if (val >= field.low_range_min && val <= field.low_range_max)
                || (val >= field.high_range_min && val <= field.high_range_max)
            {
                value_is_valid = true;
                break;
            }
        }
        if !value_is_valid {
            return false;
        }
    }

    true
}

fn open_file(filename: &str) -> Result<Info, Error> {
    let mut contents = String::new();
    File::open(filename)
        .map_err(Error::IO)?
        .read_to_string(&mut contents)
        .map_err(Error::IO)?;

    let parts: Vec<&str> = contents.split("\r\n\r\n").collect();
    if parts.len() != 3 {
        return Err(Error::Parse("wrong number of parts".to_string()));
    }

    // FIELDS
    let mut fields: Vec<Field> = Vec::new();
    // arrival station: 43-301 or 309-961
    let field_rgx = Regex::new(r"([A-z\s]+): (\d+)-(\d+) or (\d+)-(\d+)").map_err(Error::Regex)?;

    for line in parts[0].split("\r\n") {
        let captures = field_rgx
            .captures(&line)
            .ok_or(Error::Parse(line.to_string()))?;

        let parse_num = |x| -> Result<i64, Error> {
            captures
                .get(x)
                .ok_or(Error::Parse(line.to_string()))?
                .as_str()
                .parse::<i64>()
                .map_err(Error::ParseInt)
        };

        fields.push(Field {
            name: captures
                .get(1)
                .ok_or(Error::Parse(line.to_string()))?
                .as_str()
                .to_string(),
            low_range_min: parse_num(2)?,
            low_range_max: parse_num(3)?,
            high_range_min: parse_num(4)?,
            high_range_max: parse_num(5)?,
        });
    }

    // MY TICKET
    let (_, my_ticket_str) = parts[1]
        .split_once("\r\n")
        .ok_or(Error::Parse(parts[1].to_string()))?;

    let my_ticket = parse_comma_separated_nums(my_ticket_str)?;

    // NEARBY TICKETS
    let nearby_tickets: Vec<Vec<i64>> = parts[2]
        .split("\r\n")
        .skip(1)
        .map(|line| parse_comma_separated_nums(line))
        .into_iter()
        .collect::<Result<Vec<Vec<i64>>, Error>>()?;

    Ok(Info {
        fields: fields,
        my_ticket: my_ticket,
        nearby_tickets: nearby_tickets,
    })
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
        assert_eq!(ret.unwrap(), 71);
    }

    #[test]
    fn test_part_two() {
        let test_input = "2_test.txt";
        let ret = part_two(&test_input);
        assert_eq!(ret.unwrap(), 12 * 13 * 11);
    }
}
