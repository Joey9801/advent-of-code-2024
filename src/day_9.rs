#[derive(Clone, Copy, Debug)]
pub enum Contents {
    Free,
    File { id: u64 },
}

pub fn parse(input: &str) -> Vec<Contents> {
    let mut cell_lengths = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u64);

    let mut id = 0;
    let mut contents = Contents::File { id };
    let mut output = Vec::new();
    for len in cell_lengths {
        for _ in 0..len {
            output.push(contents);
        }

        contents = match contents {
            Contents::Free => {
                id += 1;
                Contents::File { id }
            }
            Contents::File { .. } => Contents::Free,
        };
    }

    output
}

pub fn solve_part_1(input: &[Contents]) -> u64 {
    let mut disk = input.to_vec();

    // Two pointers, one to the leftmost free cell, and one to the rightmost file cell
    // Each iteration, swap their contents and update the pointers

    let mut min_free = disk
        .iter()
        .position(|c| matches!(c, Contents::Free))
        .unwrap();
    let mut max_file = disk
        .iter()
        .rposition(|c| matches!(c, Contents::File { .. }))
        .unwrap();

    while min_free < max_file {
        disk.swap(min_free, max_file);

        while let Contents::File { .. } = disk[min_free] {
            min_free += 1;
        }

        while let Contents::Free = disk[max_file] {
            max_file -= 1;
        }
    }

    disk.iter()
        .enumerate()
        .map(|(idx, cell)| match cell {
            Contents::File { id } => idx as u64 * *id,
            Contents::Free => 0,
        })
        .sum()
}

pub fn solve_part_2(input: &[Contents]) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_1() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve_part_1(&input), 1928);
    }
}
