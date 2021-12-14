use super::util;

#[derive(Debug)]
pub enum Error {
  InvalidOpcode(i64),
  Util(util::Error),
  SolutionNotFound,
}

pub fn part01(path: &str) -> Result<i64, Error> {
  let mut input = util::parse_comma_separated_ints(path).map_err(Error::Util)?;
  // To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2.
  input[1] = 12;
  input[2] = 2;

  let output = run_program(&input)?;

  Ok(output[0])
}
pub fn part02(path: &str) -> Result<(i64, i64), Error> {
  let input = util::parse_comma_separated_ints(path).map_err(Error::Util)?;

  for i in 0..=99 {
    for j in 0..=99 {
      let mut program = input.clone();
      program[1] = i;
      program[2] = j;
      let output = run_program(&program)?;

      if output[0] == 19690720 {
        return Ok((i, j));
      }
    }
  }

  Err(Error::SolutionNotFound)
}

fn run_program(input: &Vec<i64>) -> Result<Vec<i64>, Error> {
  let mut program = input.clone();
  let len = program.len();

  for i in (0..len).step_by(4) {
    let opcode = program[i];
    if opcode == 99 {
      break;
    }
    let input_one_idx = program[i + 1] as usize;
    let input_two_idx = program[i + 2] as usize;
    let output_idx = program[i + 3] as usize;

    program[output_idx] = match opcode {
      1 => program[input_one_idx] + program[input_two_idx],
      2 => program[input_one_idx] * program[input_two_idx],
      _ => return Err(Error::InvalidOpcode(opcode)),
    };
  }

  Ok(program)
}

#[cfg(test)]
mod tests {
  use super::run_program;
  #[test]
  fn test_run_program() {
    assert_eq!(
      run_program(&vec![1, 0, 0, 0, 99]).unwrap(),
      vec![2, 0, 0, 0, 99]
    );
    assert_eq!(
      run_program(&vec![2, 3, 0, 3, 99]).unwrap(),
      vec![2, 3, 0, 6, 99]
    );
    assert_eq!(
      run_program(&vec![2, 4, 4, 5, 99, 0]).unwrap(),
      vec![2, 4, 4, 5, 99, 9801]
    );
    assert_eq!(
      run_program(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).unwrap(),
      vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
    assert_eq!(
      run_program(&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]).unwrap(),
      vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
  }
}
