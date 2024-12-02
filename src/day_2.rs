pub fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(mut numbers: impl Iterator<Item = u64>) -> bool {
    let mut last = numbers.next().unwrap();
    let mut last_diff = 0;

    for num in numbers {
        let diff = num as i32 - last as i32;

        let this_safe = ((diff > 0 && last_diff >= 0) || (diff < 0 && last_diff <= 0)) // same direction
            && (diff.abs() <= 3) && (diff.abs() >= 1); // valid size

        if this_safe {
            last = num;
            last_diff = diff;
        } else {
            return false;
        }
    }

    true
}

pub fn solve_part_1(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .filter(|line| is_safe(line.iter().copied()))
        .count() as u64
}

fn is_safe_p2(line: &[u64]) -> bool {
    if is_safe(line.iter().copied()) {
        return true;
    }

    for idx in 0..line.len() {
        let base_line = line.iter().copied();
        let skipped_line = base_line
            .clone()
            .take(idx)
            .chain(base_line.clone().skip(idx + 1));
        
        if is_safe(skipped_line) {
            return true;
        }
    }
    
    false
}

pub fn solve_part_2(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .filter(|line| is_safe_p2(line))
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_part_1() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(solve_part_1(&test_input), 2);
    }

    #[test]
    fn test_part_2() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(solve_part_2(&test_input), 4);
    }
}
