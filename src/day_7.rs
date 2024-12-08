#[derive(Clone, Debug)]
pub struct Line {
    target: i64,
    numbers: Vec<i64>,
}

pub fn parse(input: &str) -> Vec<Line> {
    // Lines in the form: "<target>: <number> <number> ... <number>"

    let mut lines = Vec::new();

    for line in input.lines() {
        let (target, numbers) = line.split_once(':').unwrap();

        let target = target.parse().unwrap();
        let numbers = numbers
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        lines.push(Line { target, numbers });
    }

    lines
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

struct Opiter {
    ops: Vec<Op>,
    done: bool,
}

impl Opiter {
    fn new(n: usize) -> Self {
        Self {
            ops: vec![Op::Add; n],
            done: false,
        }
    }

    fn next(&mut self) -> Option<&[Op]> {
        if self.done {
            return None;
        }

        if self.ops.iter().all(|o| *o == Op::Mul) {
            self.done = true;
        }

        for op in self.ops.iter_mut() {
            if *op != Op::Mul {
                *op = Op::Mul;
                break;
            } else {
                *op = Op::Add;
            }
        }

        Some(&self.ops)
    }
}

fn eval(numbers: &[i64], ops: &[Op]) -> i64 {
    debug_assert_eq!(numbers.len(), ops.len() + 1);
    debug_assert!(!numbers.is_empty());

    let mut working = numbers[0];

    for (num, op) in numbers[1..].iter().zip(ops.iter()) {
        working = match op {
            Op::Add => working + num,
            Op::Mul => working * num,
        }
    }

    working
}

pub fn solve_part_1(input: &[Line]) -> i64 {
    let mut sum = 0;
    for line in input {
        let mut op_it = Opiter::new(line.numbers.len() - 1);

        while let Some(ops) = op_it.next() {
            if eval(&line.numbers, ops) == line.target {
                sum += line.target;
                continue;
            }
        }
    }

    sum
}

pub fn solve_part_2(input: &[Line]) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_op_iter() {
        let mut i = Opiter::new(3);
        let all = std::iter::from_fn(move || i.next().map(|slice| slice.to_vec()))
            .collect::<Vec<Vec<Op>>>();

        assert_eq!(all.len(), 8);
        assert_eq!(all[0], [Op::Mul, Op::Add, Op::Add]);
        assert_eq!(all[7], [Op::Add, Op::Add, Op::Add]);
    }

    #[test]
    fn test_part_1() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve_part_1(&input), 3749);
    }
}
