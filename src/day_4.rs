use crate::util::{Map2d, Map2dExt, Vec2};

pub fn parse(input: &str) -> Map2d<char> {
    Map2d::parse_grid(input, std::convert::identity)
}

const DIRECTIONS: &[Vec2] = &[
    Vec2::new(1, 0),
    Vec2::new(0, 1),
    Vec2::new(1, 1),
    Vec2::new(-1, 1),
];

fn test_xmas(grid: &Map2d<char>, origin: Vec2, dir: Vec2) -> bool {
    let (a, b, c, d) = (
        grid.get(origin),
        grid.get(origin + dir),
        grid.get(origin + dir * 2),
        grid.get(origin + dir * 3),
    );

    matches!(
        (a, b, c, d),
        (Some('X'), Some('M'), Some('A'), Some('S')) | (Some('S'), Some('A'), Some('M'), Some('X'))
    )
}

pub fn solve_part_1(input: &Map2d<char>) -> u64 {
    let mut count = 0;
    for x in 0..input.size.x {
        for y in 0..input.size.y {
            for dir in DIRECTIONS {
                if test_xmas(input, Vec2::new(x, y), *dir) {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn solve_part_2(input: &Map2d<char>) -> u64 {
    let mut count = 0;

    for x in 1..(input.size.x - 1) {
        for y in 1..(input.size.y - 1) {
            if input.get(Vec2::new(x, y)) != Some('A') {
                continue;
            }

            //   a   b
            //     A
            //   c   d

            let (a, b, c, d) = (
                input.get(Vec2::new(x - 1, y + 1)).unwrap(),
                input.get(Vec2::new(x + 1, y + 1)).unwrap(),
                input.get(Vec2::new(x - 1, y - 1)).unwrap(),
                input.get(Vec2::new(x + 1, y - 1)).unwrap(),
            );

            if ((a == 'M' && d == 'S') || (a == 'S' && d == 'M'))
                && ((b == 'M' && c == 'S') || (b == 'S' && c == 'M'))
            {
                count += 1;
            }
        }
    }

    count
}
