use std::collections::HashMap;

fn main() {
    let res_one = nth_number(&vec![1, 2, 16, 19, 18], 2020);
    println!("part one: {:?}", res_one);
    let res_one = nth_number(&vec![1, 2, 16, 19, 18], 30000000);
    println!("part two: {:?}", res_one);
}

fn nth_number(initial: &Vec<i64>, turn: usize) -> i64 {
    let n = turn - 1;
    let mut history: HashMap<i64, i64> = HashMap::new();
    let mut history_prev: HashMap<i64, i64> = HashMap::new();

    for i in 0..initial.len() {
        let num = initial[i];
        if history.contains_key(&num) {
            history_prev.insert(num, *history.get(&num).unwrap());
        }
        history.insert(num, i as i64);
    }

    if n < initial.len() {
        return initial[n];
    }
    let mut last_spoken = initial[initial.len() - 1];
    for i in initial.len()..turn {
        let first_time_spoken = !history_prev.contains_key(&last_spoken);
        let num = if first_time_spoken {
            0
        } else {
            history.get(&last_spoken).unwrap() - history_prev.get(&last_spoken).unwrap()
        };

        if history.contains_key(&num) {
            history_prev.insert(num, *history.get(&num).unwrap());
        }
        history.insert(num, i as i64);

        last_spoken = num;
    }

    last_spoken
}

#[cfg(test)]
mod tests {
    use super::nth_number;

    #[test]
    fn test_nth_number() {
        assert_eq!(nth_number(vec![0, 3, 6], 1), 0);
        assert_eq!(nth_number(vec![0, 3, 6], 2), 3);
        assert_eq!(nth_number(vec![0, 3, 6], 3), 6);
        assert_eq!(nth_number(vec![0, 3, 6], 4), 0);
        assert_eq!(nth_number(vec![0, 3, 6], 5), 3);
        assert_eq!(nth_number(vec![0, 3, 6], 6), 3);
        assert_eq!(nth_number(vec![0, 3, 6], 7), 1);
        assert_eq!(nth_number(vec![0, 3, 6], 8), 0);
        assert_eq!(nth_number(vec![0, 3, 6], 9), 4);
        assert_eq!(nth_number(vec![0, 3, 6], 10), 0);
    }

    #[test]
    fn test_2020th_number() {
        assert_eq!(nth_number(vec![1, 3, 2], 2020), 1);
        assert_eq!(nth_number(vec![2, 1, 3], 2020), 10);
        assert_eq!(nth_number(vec![1, 2, 3], 2020), 27);
        assert_eq!(nth_number(vec![2, 3, 1], 2020), 78);
        assert_eq!(nth_number(vec![3, 2, 1], 2020), 438);
        assert_eq!(nth_number(vec![3, 1, 2], 2020), 1836);
    }

    #[test]
    fn test_30000000th_number() {
        assert_eq!(nth_number(vec![0, 3, 6], 30000000), 175594);
        assert_eq!(nth_number(vec![1, 3, 2], 30000000), 2578);
        assert_eq!(nth_number(vec![2, 1, 3], 30000000), 3544142);
        assert_eq!(nth_number(vec![1, 2, 3], 30000000), 261214);
        assert_eq!(nth_number(vec![2, 3, 1], 30000000), 6895259);
        assert_eq!(nth_number(vec![3, 2, 1], 30000000), 18);
        assert_eq!(nth_number(vec![3, 1, 2], 30000000), 362);
    }
}
