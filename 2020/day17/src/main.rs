use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    NotImplemented,
}

fn neighbour_offsets_3d() -> Vec<(i64, i64, i64)> {
    let mut ret = Vec::new();

    for x in [-1, 0, 1] {
        for y in [-1, 0, 1] {
            for z in [-1, 0, 1] {
                if !(x == 0 && y == 0 && z == 0) {
                    ret.push((x, y, z));
                }
            }
        }
    }

    ret
}

fn neighbour_offsets_4d() -> Vec<(i64, i64, i64, i64)> {
    let mut ret = Vec::new();

    for x in [-1, 0, 1] {
        for y in [-1, 0, 1] {
            for z in [-1, 0, 1] {
                for w in [-1, 0, 1] {
                    if !(x == 0 && y == 0 && z == 0 && w == 0) {
                        ret.push((x, y, z, w));
                    }
                }
            }
        }
    }

    ret
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
    let state_2d = open_file(path)?;
    let mut state: HashSet<(i64, i64, i64)> = HashSet::new();

    for (x, y) in state_2d.iter() {
        state.insert((*x, *y, 0));
    }

    for _ in 0..6 {
        state = run_cycle_3d(state);
    }

    Ok(state.len())
}

fn part_two(path: &str) -> Result<usize, Error> {
    let state_2d = open_file(path)?;
    let mut state: HashSet<(i64, i64, i64, i64)> = HashSet::new();

    for (x, y) in state_2d.iter() {
        state.insert((*x, *y, 0, 0));
    }

    for _ in 0..6 {
        state = run_cycle_4d(state);
    }

    Ok(state.len())
}

fn run_cycle_3d(initial_state: HashSet<(i64, i64, i64)>) -> HashSet<(i64, i64, i64)> {
    let offsets = neighbour_offsets_3d();
    // set of inactive neighbours
    let mut inactive_neighbours: HashSet<(i64, i64, i64)> = HashSet::new();
    for (x, y, z) in initial_state.iter() {
        for (dx, dy, dz) in &offsets {
            let neighbour = (x + dx, y + dy, z + dz);
            if !initial_state.contains(&neighbour) {
                inactive_neighbours.insert(neighbour);
            }
        }
    }

    let mut next_state: HashSet<(i64, i64, i64)> = HashSet::new();

    // for each active cell
    for (x, y, z) in initial_state.iter() {
        // for each neighbour
        let mut active_neighbour_count = 0;
        for (dx, dy, dz) in &offsets {
            let neighbour = (x + dx, y + dy, z + dz);
            if initial_state.contains(&neighbour) {
                active_neighbour_count += 1
            }
            if active_neighbour_count > 3 {
                break;
            }
        }
        // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active.
        // Otherwise, the cube becomes inactive.
        if active_neighbour_count >= 2 && active_neighbour_count <= 3 {
            next_state.insert((*x, *y, *z));
        }
    }
    // for each inactive neighbour
    for (x, y, z) in inactive_neighbours.iter() {
        let mut active_neighbour_count = 0;
        for (dx, dy, dz) in &offsets {
            let neighbour = (x + dx, y + dy, z + dz);
            if initial_state.contains(&neighbour) {
                active_neighbour_count += 1
            }
            if active_neighbour_count > 3 {
                break;
            }
        }
        // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
        if active_neighbour_count == 3 {
            next_state.insert((*x, *y, *z));
        }
    }

    next_state
}

fn run_cycle_4d(initial_state: HashSet<(i64, i64, i64, i64)>) -> HashSet<(i64, i64, i64, i64)> {
    let offsets = neighbour_offsets_4d();
    // set of inactive neighbours
    let mut inactive_neighbours: HashSet<(i64, i64, i64, i64)> = HashSet::new();
    for (x, y, z, w) in initial_state.iter() {
        for (dx, dy, dz, dw) in &offsets {
            let neighbour = (x + dx, y + dy, z + dz, w + dw);
            if !initial_state.contains(&neighbour) {
                inactive_neighbours.insert(neighbour);
            }
        }
    }

    let mut next_state: HashSet<(i64, i64, i64, i64)> = HashSet::new();

    // for each active cell
    for (x, y, z, w) in initial_state.iter() {
        // for each neighbour
        let mut active_neighbour_count = 0;
        for (dx, dy, dz, dw) in &offsets {
            let neighbour = (x + dx, y + dy, z + dz, w + dw);
            if initial_state.contains(&neighbour) {
                active_neighbour_count += 1
            }
            if active_neighbour_count > 3 {
                break;
            }
        }
        // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active.
        // Otherwise, the cube becomes inactive.
        if active_neighbour_count >= 2 && active_neighbour_count <= 3 {
            next_state.insert((*x, *y, *z, *w));
        }
    }
    // for each inactive neighbour
    for (x, y, z, w) in inactive_neighbours.iter() {
        let mut active_neighbour_count = 0;
        for (dx, dy, dz, dw) in &offsets {
            let neighbour = (x + dx, y + dy, z + dz, w + dw);
            if initial_state.contains(&neighbour) {
                active_neighbour_count += 1
            }
            if active_neighbour_count > 3 {
                break;
            }
        }

        // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
        if active_neighbour_count == 3 {
            next_state.insert((*x, *y, *z, *w));
        }
    }

    next_state
}

fn open_file(filename: &str) -> Result<HashSet<(i64, i64)>, Error> {
    let mut active_vec: Vec<(i64, i64)> = Vec::new();
    for (y, maybe_line) in io::BufReader::new(File::open(filename).map_err(Error::IOError)?)
        .lines()
        .enumerate()
    {
        let line = maybe_line.map_err(Error::IOError)?;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_vec.push((x as i64, y as i64))
            }
        }
    }
    let mut active_set: HashSet<(i64, i64)> = HashSet::new();
    for coord in active_vec {
        active_set.insert(coord);
    }
    Ok(active_set)
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
        assert_eq!(ret.unwrap(), 112);
    }

    #[test]
    fn test_part_two() {
        let test_input = "1_test.txt";
        let ret = part_two(&test_input);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 848);
    }
}
