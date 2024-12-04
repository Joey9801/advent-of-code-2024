use std::collections::HashSet;

use crate::util::{Map2d, Map2dExt, Vec2};

pub fn parse(input: &str) -> Map2d<char> {
    Map2d::parse_grid(input, std::convert::identity)
}

/// All 8 possible directions
const DIRECTIONS: &[Vec2] = &[
    Vec2::new(1, 0),
    Vec2::new(1, 1),
    Vec2::new(0, 1),
    Vec2::new(-1, 1),
    Vec2::new(-1, 0),
    Vec2::new(-1, -1),
    Vec2::new(0, -1),
    Vec2::new(1, -1),
];

fn test_pos(grid: &Map2d<char>, origin: Vec2, dir: Vec2, query: &str) -> bool {
    let mut pos = origin;
    for c in query.chars() {
        if grid.get(pos) != Some(c) {
            return false;
        }
        pos += dir;
    }

    true
}

pub fn solve_part_1(input: &Map2d<char>) -> u64 {
    let mut count = 0;
    for x in 0..input.size.x {
        for y in 0..input.size.y {
            for dir in DIRECTIONS {
                if test_pos(input, Vec2::new(x, y), *dir, "XMAS") {
                    count += 1;
                }
            }
        }
    }
    count
}

/// Return the set of all the positions of the A of MAS on +/- the given
/// direction
fn find_mas(input: &Map2d<char>, dir: Vec2) -> HashSet<Vec2> {
    let mut found = HashSet::new();

    for x in 0..input.size.x {
        for y in 0..input.size.y {
            let origin = Vec2::new(x, y);
            if test_pos(input, origin, dir, "MAS") {
                found.insert(origin + dir);
            }

            if test_pos(input, origin, -dir, "MAS") {
                found.insert(origin - dir);
            }
        }
    }
    
    found
}

pub fn solve_part_2(input: &Map2d<char>) -> u64 {
    let a = find_mas(input, Vec2::new(1, 1));
    let b = find_mas(input, Vec2::new(-1, 1));

    a.intersection(&b).count() as u64
}