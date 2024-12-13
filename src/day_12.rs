use crate::util::{Dir, Map2d, Map2dExt, Vec2};

pub fn parse(input: &str) -> Map2d<char> {
    Map2d::parse_grid(input, std::convert::identity)
}

fn adjacency(map: &Map2d<char>, pos: Vec2) -> impl Iterator<Item = Vec2> + '_ {
    Dir::ALL
        .iter()
        .copied()
        .map(move |dir| pos + dir)
        .filter(move |test| map.get(*test) == map.get(pos))
}

struct StackElement<I: Iterator<Item = Vec2>> {
    pos: Vec2,
    adj: I,
}

// These numbers came to me in a dream
const P1_MAGIC: [i8; 256] = [
    4, 4, 2, 2, 4, 4, 2, 2, 2, 2, 0, 0, 2, 2, 0, 0, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, -2, -2, 0, 0, -2,
    -2, 4, 4, 2, 2, 4, 4, 2, 2, 2, 2, 0, 0, 2, 2, 0, 0, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, -2, -2, 0, 0,
    -2, -2, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, -2, -2, 0, 0, -2, -2, 0, 0, -2, -2, 0, 0, -2, -2, -2, -2,
    -4, -4, -2, -2, -4, -4, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, -2, -2, 0, 0, -2, -2, 0, 0, -2, -2, 0, 0,
    -2, -2, -2, -2, -4, -4, -2, -2, -4, -4, 4, 4, 2, 2, 4, 4, 2, 2, 2, 2, 0, 0, 2, 2, 0, 0, 2, 2,
    0, 0, 2, 2, 0, 0, 0, 0, -2, -2, 0, 0, -2, -2, 4, 4, 2, 2, 4, 4, 2, 2, 2, 2, 0, 0, 2, 2, 0, 0,
    2, 2, 0, 0, 2, 2, 0, 0, 0, 0, -2, -2, 0, 0, -2, -2, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, -2, -2, 0, 0,
    -2, -2, 0, 0, -2, -2, 0, 0, -2, -2, -2, -2, -4, -4, -2, -2, -4, -4, 2, 2, 0, 0, 2, 2, 0, 0, 0,
    0, -2, -2, 0, 0, -2, -2, 0, 0, -2, -2, 0, 0, -2, -2, -2, -2, -4, -4, -2, -2, -4, -4,
];
const P2_MAGIC: [i8; 256] = [
    4, 4, 0, 2, 4, 4, 2, 4, 0, 2, -2, -2, 0, 2, 0, 0, 0, 0, -2, 0, 2, 2, -2, 0, -4, -2, -4, -4, -2,
    0, -4, -4, 4, 4, 0, 2, 4, 4, 2, 4, 2, 4, 0, 0, 2, 4, 2, 2, 0, 0, -2, 0, 2, 2, -2, 0, -2, 0, -2,
    -2, 0, 2, -2, -2, 0, 0, -4, -2, 0, 0, -2, 0, -2, 0, -4, -4, -2, 0, -2, -2, -2, -2, -4, -2, 0,
    0, -4, -2, -4, -2, -4, -4, -2, 0, -4, -4, 2, 2, -2, 0, 2, 2, 0, 2, -2, 0, -4, -4, -2, 0, -2,
    -2, 0, 0, -2, 0, 2, 2, -2, 0, -4, -2, -4, -4, -2, 0, -4, -4, 4, 4, 0, 2, 4, 4, 2, 4, 0, 2, -2,
    -2, 0, 2, 0, 0, 2, 2, 0, 2, 4, 4, 0, 2, -2, 0, -2, -2, 0, 2, -2, -2, 4, 4, 0, 2, 4, 4, 2, 4, 2,
    4, 0, 0, 2, 4, 2, 2, 2, 2, 0, 2, 4, 4, 0, 2, 0, 2, 0, 0, 2, 4, 0, 0, 2, 2, -2, 0, 2, 2, 0, 2,
    0, 2, -2, -2, 0, 2, 0, 0, -2, -2, -4, -2, 0, 0, -4, -2, -4, -2, -4, -4, -2, 0, -4, -4, 4, 4, 0,
    2, 4, 4, 2, 4, 0, 2, -2, -2, 0, 2, 0, 0, 0, 0, -2, 0, 2, 2, -2, 0, -4, -2, -4, -4, -2, 0, -4,
    -4,
];

fn magic_key(region: &Map2d<bool>, pos: Vec2) -> usize {
    const OFFSETS: [Vec2; 8] = [
        Vec2::new(-1, -1),
        Vec2::new(0, -1),
        Vec2::new(1, -1),
        Vec2::new(-1, 0),
        Vec2::new(1, 0),
        Vec2::new(-1, 1),
        Vec2::new(0, 1),
        Vec2::new(1, 1),
    ];

    let mut key = 0;
    for offset in OFFSETS.iter().copied() {
        key <<= 1;
        if let Some(true) = region.get(pos + offset) {
            key |= 1;
        }
    }

    key
}

fn region_cost(map: &Map2d<char>, visited: &mut Map2d<bool>, seed: Vec2, magic: &[i8]) -> i64 {
    debug_assert_eq!(map.size, visited.size);

    let mut this_region = Map2d::new_default(map.size, false);

    *visited.get_mut(seed).unwrap() = true;
    *this_region.get_mut(seed).unwrap() = true;

    let mut stack = vec![StackElement {
        pos: seed,
        adj: adjacency(map, seed),
    }];

    let mut area = 1;
    let mut edges = magic[0] as i64;
    while !stack.is_empty() {
        match stack.last_mut().unwrap().adj.next() {
            Some(next_pos) => {
                let Some(false) = visited.get(next_pos) else {
                    continue;
                };

                area += 1;
                edges += magic[magic_key(&this_region, next_pos)] as i64;

                *visited.get_mut(next_pos).unwrap() = true;
                *this_region.get_mut(next_pos).unwrap() = true;

                stack.push(StackElement {
                    pos: next_pos,
                    adj: adjacency(map, next_pos),
                });
            }
            None => {
                stack.pop();
            }
        }
    }

    area * edges
}

pub fn solve_part_1(input: &Map2d<char>) -> i64 {
    let mut visited = Map2d::new_default(input.size, false);

    let mut total = 0;
    while let Some(seed) = visited.find(|x| !x) {
        total += region_cost(input, &mut visited, seed, &P1_MAGIC);
    }

    total
}

pub fn solve_part_2(input: &Map2d<char>) -> i64 {
    let mut visited = Map2d::new_default(input.size, false);

    let mut total = 0;
    while let Some(seed) = visited.find(|x| !x) {
        total += region_cost(input, &mut visited, seed, &P2_MAGIC);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    #[test]
    fn test_part_1() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve_part_1(&input), 1930);
    }
}
