use std::collections::HashMap;

pub fn parse(input: &str) -> Vec<i64> {
    input
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

/// If the given number has an even count of base 10 digits, returns the split
///
/// 1234 => Some(12, 34)
/// 12345 => None
/// 123000 => Some(123, 0)
/// 123001 => Some(123, 1)
fn split_digits(num: i64) -> Option<(i64, i64)> {
    let digit_count = num.ilog10() + 1;

    if digit_count % 2 == 0 {
        let size = 10i64.pow(digit_count / 2);
        let upper = num / size;
        let lower = num % size;
        Some((upper, lower))
    } else {
        None
    }
}

fn p1_inner(num: i64, iterations: usize, memo: &mut HashMap<(i64, usize), u64>) -> u64 {
    if iterations == 0 {
        return 1;
    }

    if let Some(cached) = memo.get(&(num, iterations)) {
        return *cached;
    }

    if num == 0 {
        p1_inner(1, iterations - 1, memo)
    } else if let Some((upper, lower)) = split_digits(num) {
        p1_inner(upper, iterations - 1, memo) + p1_inner(lower, iterations - 1, memo)
    } else {
        p1_inner(num * 2024, iterations - 1, memo)
    }
}

pub fn solve_part_1(input: &[i64]) -> u64 {
    let mut memo: HashMap<(i64, usize), u64> = Default::default();
    input.iter().map(|x| p1_inner(*x, 25, &mut memo)).sum()
}

pub fn solve_part_2(input: &[i64]) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_split_digits() {
        assert_eq!(split_digits(1234), Some((12, 34)));
        assert_eq!(split_digits(12345), None);
        assert_eq!(split_digits(123000), Some((123, 0)));
        assert_eq!(split_digits(123001), Some((123, 1)));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(&[125, 17]), 55312)
    }
}
