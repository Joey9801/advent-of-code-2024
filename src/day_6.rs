use std::collections::HashSet;

use crate::util::{Dir, Map2d, Map2dExt, Vec2};

#[derive(Clone, Copy, Debug)]
enum RawTile {
    Empty,
    Wall,
    Guard(Dir),
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Clone, Copy, Debug)]
struct Guard {
    pos: Vec2,
    dir: Dir,
}

impl From<RawTile> for Tile {
    fn from(raw: RawTile) -> Tile {
        match raw {
            RawTile::Empty | RawTile::Guard(_) => Tile::Empty,
            RawTile::Wall => Tile::Wall,
        }
    }
}

pub struct Input {
    tiles: Map2d<Tile>,
    guard: Guard,
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Input {
        self
    }
}

pub fn parse(input: &str) -> Input {
    let raw_map = Map2d::parse_grid(input, |c| match c {
        '.' => RawTile::Empty,
        '#' => RawTile::Wall,
        'v' => RawTile::Guard(Dir::Down),
        '^' => RawTile::Guard(Dir::Up),
        '<' => RawTile::Guard(Dir::Left),
        '>' => RawTile::Guard(Dir::Right),
        _ => panic!("Invalid character in input"),
    });

    let guard_pos =
        Map2d::find(&raw_map, |tile| matches!(tile, RawTile::Guard(_))).expect("No guard");

    let guard_dir = match raw_map.get(guard_pos) {
        Some(RawTile::Guard(dir)) => dir,
        _ => unreachable!(),
    };

    Input {
        tiles: raw_map.convert(),
        guard: Guard {
            pos: guard_pos,
            dir: guard_dir,
        },
    }
}

pub fn solve_part_1(input: &Input) -> usize {
    let mut unique_positions = HashSet::new();

    let mut guard = input.guard;
    unique_positions.insert(guard.pos);

    loop {
        let next_pos = guard.pos + guard.dir;
        match input.tiles.get(next_pos) {
            Some(Tile::Empty) => {
                guard.pos = next_pos;
                unique_positions.insert(guard.pos);
            }
            Some(Tile::Wall) => {
                guard.dir = guard.dir.rotate_right();
            },
            None => break,
        }
    }

    unique_positions.len()
}

pub fn solve_part_2(input: &Input) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
    
    #[test]
    fn test_part_1() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve_part_1(&input), 41);
    }
    
    #[test]
    fn test_part_2() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve_part_2(&input), 6);
    }
}