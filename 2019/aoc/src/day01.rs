use super::util;

pub fn part01(path: &str) -> Result<i64, util::Error> {
  let masses = util::parse_int_per_line(path)?;
  Ok(masses.iter().fold(0, |s, &m| s + fuel_required(m)))
}

pub fn part02(path: &str) -> Result<i64, util::Error> {
  let masses = util::parse_int_per_line(path)?;
  Ok(masses.iter().fold(0, |s, &m| s + fuel_required_part2(m)))
}

fn fuel_required(mass: i64) -> i64 {
  (mass / 3) - 2
}

fn fuel_required_part2(mass: i64) -> i64 {
  let fuel = fuel_required(mass);
  if fuel <= 0 {
    return 0;
  }

  fuel + fuel_required_part2(fuel)
}

#[cfg(test)]
mod tests {
  use super::fuel_required;
  use super::fuel_required_part2;

  #[test]
  fn test_fuel_required() {
    assert_eq!(fuel_required(12), 2);
    assert_eq!(fuel_required(14), 2);
    assert_eq!(fuel_required(1969), 654);
    assert_eq!(fuel_required(100756), 33583);
  }

  #[test]
  fn test_fuel_required_part2() {
    assert_eq!(fuel_required_part2(14), 2);
    assert_eq!(fuel_required_part2(1969), 966);
    assert_eq!(fuel_required_part2(100756), 50346);
  }
}
