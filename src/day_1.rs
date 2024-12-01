use std::collections::HashMap;

pub struct Input {
    left: Vec<u64>,
    right: Vec<u64>,
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Input {
        self
    }
}

pub fn parse(input: &str) -> Input {
    // Input is in the form "L1 R1\nL2 R2\nL3 R3\n..."
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    }

    Input { left, right }
}

pub fn solve_part_1(input: &Input) -> u64 {
    let mut left_sorted = input.left.clone();
    left_sorted.sort();

    let mut right_sorted = input.right.clone();
    right_sorted.sort();

    // Sum of differences
    left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

pub fn solve_part_2(input: &Input) -> u64 {
    let mut right_hist = HashMap::new();
    for r in &input.right {
        *right_hist.entry(r).or_insert(0) += 1;
    }

    input
        .left
        .iter()
        .map(|l| right_hist.get(l).unwrap_or(&0) * l)
        .sum()
}
