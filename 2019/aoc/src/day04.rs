pub fn part01() -> i64 {
  let range_low = 153517;
  let range_high = 630395;

  let mut count = 0;
  for n in range_low..=range_high {
    let mut nn = n;
    let mut previous_digit = nn % 10;
    let mut found_doubled = false;
    let mut is_increasing = true;
    nn /= 10;
    for i in 0..5 {
      let next_digit = nn % 10;
      if next_digit == previous_digit {
        found_doubled = true;
      } else if next_digit > previous_digit {
        is_increasing = false;
        break;
      }
      nn /= 10;
      previous_digit = next_digit;
    }

    if is_increasing && found_doubled {
      count += 1;
    }
  }

  count
}

pub fn part02() -> i64 {
  let range_low = 153517;
  let range_high = 630395;

  let mut count = 0;
  for n in range_low..=range_high {
    let mut nn = n;
    let mut previous_digit = nn % 10;
    nn /= 10;
    let mut digit_counts = [0i64; 10];
    digit_counts[previous_digit] += 1;
    let mut is_increasing = true;
    for _ in 0..5 {
      let next_digit = nn % 10;
      if next_digit > previous_digit {
        is_increasing = false;
        break;
      }

      nn /= 10;
      digit_counts[next_digit] += 1;
      previous_digit = next_digit;
    }

    if is_increasing {
      if digit_counts.iter().any(|&c| c == 2) {
        count += 1;
      }
    }
  }

  count
}
