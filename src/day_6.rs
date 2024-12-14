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

enum GuardRoute {
    Finite { unique_positions: HashSet<Vec2> },
    Loop,
}

fn guard_route(map: &Map2d<Tile>, initial: Guard) -> GuardRoute {
    let mut guard = initial;
    let mut visited_states = HashSet::new(); // Track (position, direction)
    let mut unique_positions = HashSet::new(); // Track all unique positions visited

    loop {
        // Record the current state (position and direction)
        if !visited_states.insert((guard.pos, guard.dir)) {
            return GuardRoute::Loop; // Detected a loop
        }

        // Record the unique position visited
        unique_positions.insert(guard.pos);

        // Determine the next position based on the guard's direction
        let next_pos = guard.pos + guard.dir;

        match map.get(next_pos) {
            Some(Tile::Empty) => {
                guard.pos = next_pos;
            }
            Some(Tile::Wall) => {
                guard.dir = guard.dir.rotate_right();
            }
            None => {
                return GuardRoute::Finite { unique_positions };
            }
        }
    }
}

pub fn solve_part_1(input: &Input) -> usize {
    let route = guard_route(&input.tiles, input.guard);
    let GuardRoute::Finite { unique_positions } = route else {
        panic!("Infinite loop in initial input")
    };
    unique_positions.len()
}

pub fn solve_part_2(input: &Input) -> u64 {
    let GuardRoute::Finite {
        unique_positions: mut initial_route,
    } = guard_route(&input.tiles, input.guard)
    else {
        unreachable!()
    };
    
    // Remove the guard's starting position from the initial route
    initial_route.remove(&input.guard.pos);

    let mut possible_positions = 0;
    let mut test_map = input.tiles.clone();

    // Loop over all positions in the map
    for pos in initial_route {
        // Temporarily place a wall at this position
        *test_map.get_mut(pos).unwrap() = Tile::Wall;

        // Simulate guard's movement with the obstruction
        let mut guard = input.guard;
        let mut visited_states = HashSet::new();
        let mut is_loop = false;

        loop {
            // Check if the current state has been visited before
            if !visited_states.insert((guard.pos, guard.dir)) {
                is_loop = true;
                break;
            }

            let next_pos = guard.pos + guard.dir;
            match test_map.get(next_pos) {
                Some(Tile::Empty) => {
                    guard.pos = next_pos;
                }
                Some(Tile::Wall) => {
                    guard.dir = guard.dir.rotate_right();
                }
                None => break,
            }
        }

        // If a loop is found, count this obstruction position as valid
        if is_loop {
            possible_positions += 1;
        }

        // Reset the test map
        *test_map.get_mut(pos).unwrap() = Tile::Empty;
    }

    possible_positions
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
