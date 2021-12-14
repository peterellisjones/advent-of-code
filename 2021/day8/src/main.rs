use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    SplitLine,
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

fn part_one(path: &str) -> Result<i64, Error> {
    let example = parse_input(path)?;

    let mut count = 0;

    for (_, output) in example {
        for word in output {
            if word.len() == 2 || word.len() == 3 || word.len() == 4 || word.len() == 7 {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn part_two(path: &str) -> Result<i64, Error> {
    let example = parse_input(path)?;
    let mut total = 0;
    for (input, output) in example {
        total += part_two_value(&input, &output);
    }

    Ok(total as i64)
}

fn part_two_value(input: &Vec<String>, output: &Vec<String>) -> i64 {
    // Number requirements
    // 0 => a b c   e f g (6)
    // 1 =>     c     f   (2) - UNIQUE
    // 2 => a   c d e   g (5)
    // 3 => a   c d   f g (5)
    // 4 =>   b c d   f   (4) - UNIQUE
    // 5 => a b   d   f g (5)
    // 6 => a b   d e f g (6)
    // 7 => a   c     f   (3) - UNIQUE
    // 8 => a b c d e f g (7) - UNIQUE
    // 9 => a b c d   f g (6)

    // Segment requirements and number of occurences in list of ten numbers
    // a => 0   2 3   5 6 7 8 9 => 8
    // b =>   1     4 5 6   8 9 => 6 => UNIQUE
    // c => 0 1 2 3 4     7 8 9 => 8
    // d =>     2 3 4 5 6   8 9 => 7
    // e => 0   2       6   8   => 4 => UNIQUE
    // f => 0 1   3 4 5 6 7 8 9 => 9 => UNIQUE
    // g => 0   2 3   5 6   8 9 => 7

    // maps segments to possible segments
    let mut mappings: HashMap<char, char> = HashMap::new();

    let mut reverse_mappings: HashMap<char, char> = HashMap::new();
    for c in vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
        let occurences = input
            .iter()
            .fold(0, |s, v| s + v.chars().filter(|&x| x == c).count());

        match occurences {
            // B E F can be identified by number of occurences
            6 => {
                mappings.insert(c, 'b');
                reverse_mappings.insert('b', c);
            }
            4 => {
                mappings.insert(c, 'e');
                reverse_mappings.insert('e', c);
            }
            9 => {
                mappings.insert(c, 'f');
                reverse_mappings.insert('f', c);
            }
            _ => (),
        };
    }
    let b_char = *reverse_mappings.get(&'b').unwrap();
    let e_char = *reverse_mappings.get(&'e').unwrap();
    let f_char = *reverse_mappings.get(&'f').unwrap();

    // remaning segments: A, C, D, G
    // C can be identified because the number 1 has 2 segments: C and F
    let number_1 = input.iter().filter(|s| s.len() == 2).next().unwrap();
    let c_char = number_1.chars().filter(|&x| x != f_char).next().unwrap();
    mappings.insert(c_char, 'c');
    reverse_mappings.insert('c', c_char);

    // remaning segments: A, D, G
    // A can be identified because the number 7 has 3 segments: A, C, and F
    let number_7 = input.iter().filter(|s| s.len() == 3).next().unwrap();
    let a_char = number_7
        .chars()
        .filter(|&x| x != f_char && x != c_char)
        .next()
        .unwrap();
    mappings.insert(a_char, 'a');
    reverse_mappings.insert('a', a_char);

    // remaning segments: D, G
    // D can be identified because the number 4 has 4 segments: B, C, D, and F
    let number_4 = input.iter().filter(|s| s.len() == 4).next().unwrap();
    let d_char = number_4
        .chars()
        .filter(|&x| x != b_char && x != c_char && x != f_char)
        .next()
        .unwrap();
    mappings.insert(d_char, 'd');
    reverse_mappings.insert('d', d_char);

    // remaining segments: G
    // G can be identified because the number 8 has 7 segments: A, B, C, D, E, F, and G
    let number_8 = input.iter().filter(|s| s.len() == 7).next().unwrap();
    let g_char = number_8
        .chars()
        .filter(|&x| {
            x != a_char && x != b_char && x != c_char && x != d_char && x != e_char && x != f_char
        })
        .next()
        .unwrap();
    mappings.insert(g_char, 'g');
    reverse_mappings.insert('g', g_char);

    // now we have mappings for characters create a mapping of
    // sorted segments to numbers

    let mut number_mappings: HashMap<String, usize> = HashMap::new();
    for (n, word) in vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .iter()
    .enumerate()
    {
        let mut sorted_chars: Vec<char> = word
            .chars()
            .map(|c| reverse_mappings.get(&c).unwrap())
            .map(|c| *c)
            .collect();
        sorted_chars.sort();
        let sorted_str: String = sorted_chars.iter().collect();
        number_mappings.insert(sorted_str, n);
    }

    let mut output_value = 0;
    let mut tens = 1;
    for word in output.iter().rev() {
        let mut sorted_chars: Vec<char> = word.chars().collect();
        sorted_chars.sort();
        let sorted_str: String = sorted_chars.iter().collect();
        output_value += number_mappings.get(&sorted_str).unwrap() * tens;
        tens *= 10;
    }

    output_value as i64
}

fn parse_input(path: &str) -> Result<Vec<(Vec<String>, Vec<String>)>, Error> {
    io::BufReader::new(File::open(path).map_err(Error::IO)?)
        .lines()
        .map(|maybe_line| -> Result<(Vec<String>, Vec<String>), Error> {
            let line = maybe_line.map_err(Error::IO)?;
            parse_line(&line)
        })
        .collect::<Result<Vec<(Vec<String>, Vec<String>)>, Error>>()
}

fn parse_line(line: &str) -> Result<(Vec<String>, Vec<String>), Error> {
    let (left, right) = line.split_once(" | ").ok_or(Error::SplitLine)?;

    let input = left
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let output = right
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    Ok((input, output))
}

mod tests {
    use super::parse_line;
    use super::part_one;
    use super::part_two;
    use super::part_two_value;

    #[test]
    fn test_part_one() {
        let res = part_one("1_test.txt");
        assert_eq!(res.unwrap(), 26);
    }

    #[test]
    fn test_part_two_value() {
        let (input, output) = parse_line(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        )
        .unwrap();
        let res = part_two_value(&input, &output);
        assert_eq!(res, 5353);
    }

    #[test]
    fn test_part_two() {
        let res = part_two("1_test.txt");
        assert_eq!(res.unwrap(), 61229);
    }
}
