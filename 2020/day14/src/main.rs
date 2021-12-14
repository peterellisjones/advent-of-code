use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),
    Parse(String),
    RegexError(regex::Error),
}

#[derive(Debug)]
enum Instruction {
    Mask(u64, u64, u64),
    MemSet(u64, u64),
}

fn main() {
    match part_one("1.txt") {
        Err(e) => println!("Run part one error: {:?}", e),
        Ok(sum) => println!("Run part one: {:?}", sum),
    }
    match part_two("1.txt") {
        Err(e) => println!("Run part two error: {:?}", e),
        Ok(sum) => println!("Run part two: {:?}", sum),
    }
}

fn part_one(path: &str) -> Result<u64, Error> {
    let instructions = open_file(&path)?;
    let mut mask_ones = 0;
    let mut mask_zeroes = 0;
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(ones, zeroes, _) => {
                mask_ones = ones;
                mask_zeroes = zeroes;
            }
            Instruction::MemSet(address, value) => {
                let mut val = value | mask_ones;
                val &= u64::MAX ^ mask_zeroes;
                memory.insert(address, val);
            }
        }
    }

    Ok(memory.iter().fold(0, |s, (_, v)| s + v))
}

fn part_two(path: &str) -> Result<u64, Error> {
    let instructions = open_file(&path)?;
    let mut mask_ones = 0;
    let mut mask_xs = 0;
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(ones, _, xs) => {
                mask_ones = ones;
                mask_xs = xs;
            }
            Instruction::MemSet(address, value) => {
                // If the bitmask bit is 0, the corresponding memory address bit is unchanged.
                // If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
                let updated_address = address | mask_ones;
                // If the bitmask bit is X, the corresponding memory address bit is floating.

                let floating_address_count: u64 = 1 << mask_xs.count_ones();
                let mut floating_address_idx_masks: Vec<u64> = Vec::new();
                let mut xs: u64 = mask_xs;
                while xs != 0 {
                    let bit = xs & 0u64.wrapping_sub(xs);
                    floating_address_idx_masks.push(bit as u64);
                    xs ^= bit;
                }

                for i in 0..floating_address_count {
                    let mut addr = updated_address;
                    for j in 0..floating_address_idx_masks.len() {
                        let idx_mask: u64 = floating_address_idx_masks[j];

                        if i & (1 << j) != 0 {
                            addr |= idx_mask;
                        } else {
                            addr &= u64::MAX ^ idx_mask;
                        }
                    }
                    memory.insert(addr, value);
                }
            }
        }
    }

    Ok(memory.iter().fold(0, |s, (_, v)| s + v))
}

fn open_file(filename: &str) -> Result<Vec<Instruction>, Error> {
    // mask = 100X000X01XX11X10X01X11100101XX11101
    let mask_rgx = Regex::new(r"mask = ([10X]+)").map_err(Error::RegexError)?;
    // mem[7] = 101
    let memset_rgx = Regex::new(r"mem\[([0-9]+)\] = ([0-9]+)").map_err(Error::RegexError)?;
    io::BufReader::new(File::open(filename).map_err(Error::IO)?)
        .lines()
        .map(|maybe_line| -> Result<Instruction, Error> {
            let line = maybe_line.map_err(Error::IO)?;

            if memset_rgx.is_match(&line) {
                let captures = memset_rgx
                    .captures(&line)
                    .ok_or(Error::Parse(line.to_string()))?;

                let parse_int = |x| -> Result<u64, Error> {
                    captures
                        .get(x)
                        .ok_or(Error::Parse(line.to_string()))?
                        .as_str()
                        .parse::<u64>()
                        .map_err(Error::ParseInt)
                };

                Ok(Instruction::MemSet(parse_int(1)?, parse_int(2)?))
            } else if mask_rgx.is_match(&line) {
                let capture: &str = mask_rgx
                    .captures(&line)
                    .ok_or(Error::Parse(line.to_string()))?
                    .get(1)
                    .ok_or(Error::Parse(line.to_string()))?
                    .as_str();

                if capture.len() != 36 {
                    return Err(Error::Parse(line.to_string()));
                }

                let mut mask_ones: u64 = 0;
                let mut mask_zeroes: u64 = 0;
                let mut mask_xs: u64 = 0;

                for (idx, c) in capture.chars().rev().enumerate() {
                    match c {
                        '1' => mask_ones |= 1 << idx,
                        '0' => mask_zeroes |= 1 << idx,
                        'X' => mask_xs |= 1 << idx,
                        _ => return Err(Error::Parse(line.to_string())),
                    }
                }

                Ok(Instruction::Mask(mask_ones, mask_zeroes, mask_xs))
            } else {
                Err(Error::Parse(line.to_string()))
            }
        })
        .into_iter()
        .collect::<Result<Vec<Instruction>, Error>>()
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
        assert_eq!(ret.unwrap(), 165);
    }

    #[test]
    fn test_part_two() {
        let test_input = "2_test.txt";
        let ret = part_two(&test_input);
        assert_eq!(ret.is_ok(), true);
        assert_eq!(ret.unwrap(), 208);
    }
}
