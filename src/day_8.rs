use std::{collections::HashSet, convert::identity};

use crate::util::{combinatorial, Map2d, Map2dExt, Vec2};

pub struct Input {
    map_size: Vec2,

    // All antenna locations, sorted by char
    antennas: Vec<(char, Vec2)>,
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub fn parse(input: &str) -> Input {
    let map = Map2d::parse_grid(input, identity);

    let mut antennas = Vec::new();
    for y in 0..map.size.y {
        for x in 0..map.size.x {
            let pos = Vec2::new(x, y);
            match map.get(pos) {
                Some('.') => (),
                Some(other) => antennas.push((other, pos)),
                None => unreachable!(),
            }
        }
    }

    antennas.sort_by_key(|(c, _pos)| *c);

    Input {
        map_size: map.size,
        antennas,
    }
}

pub fn solve_part_1(input: &Input) -> usize {
    let mut antinodes = HashSet::new();

    let mut idx = 0;
    while idx < input.antennas.len() {
        let this_c = input.antennas[idx].0;
        let len = input.antennas[idx..]
            .iter()
            .position(|(c, _pos)| *c != this_c)
            .unwrap_or(input.antennas[idx..].len());

        let this_slice = &input.antennas[idx..(idx + len)];

        for ((char_a, pos_a), (char_b, pos_b)) in combinatorial::pairs(this_slice) {
            debug_assert_eq!(char_a, char_b);
            
            // The vector from a to b
            let ab = pos_b - pos_a;
            
            for test in [*pos_a - ab, *pos_b + ab] {
                if test.inside_map(input.map_size) {
                    antinodes.insert(test);
                }
            }
        }

        idx += this_slice.len();
    }

    antinodes.len()
}

pub fn solve_part_2(input: &Input) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn test_part_1() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve_part_1(&input), 14);
    }
}
