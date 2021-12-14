mod day01;
mod day02;
mod day03;
mod day04;
mod util;

fn main() {
    match day01::part01("day01.txt") {
        Err(e) => println!("day01 part01 ERROR: {:?}", e),
        Ok(result) => println!("day01 part01 result: {:?}", result),
    }

    match day01::part02("day01.txt") {
        Err(e) => println!("day01 part02 ERROR: {:?}", e),
        Ok(result) => println!("day01 part02 result: {:?}", result),
    }

    match day02::part01("day02.txt") {
        Err(e) => println!("day02 part01 ERROR: {:?}", e),
        Ok(result) => println!("day02 part01 result: {:?}", result),
    }

    match day02::part02("day02.txt") {
        Err(e) => println!("day02 part02 ERROR: {:?}", e),
        Ok((noun, verb)) => println!("day02 part02 result: {:?}", 100 * noun + verb),
    }

    match day03::part01("day03.txt") {
        Err(e) => println!("day03 part01 ERROR: {:?}", e),
        Ok(result) => println!("day03 part01 result: {:?}", result),
    }

    match day03::part02("day03.txt") {
        Err(e) => println!("day03 part02 ERROR: {:?}", e),
        Ok(result) => println!("day03 part02 result: {:?}", result),
    }

    println!("day04 part01 result: {:?}", day04::part01());
    println!("day04 part02 result: {:?}", day04::part02());
}
