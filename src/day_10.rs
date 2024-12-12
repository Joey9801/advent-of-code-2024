use std::collections::HashSet;

use crate::util::{Map2d, Map2dExt, Vec2};

pub fn parse(input: &str) -> Map2d<u8> {
    Map2d::parse_grid(input, |c| c.to_digit(10).unwrap() as u8)
}

fn adjacency(map: &Map2d<u8>, pos: Vec2) -> impl Iterator<Item = Vec2> + '_ {
    const DIRS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    DIRS.iter()
        .copied()
        .map(move |(dx, dy)| Vec2::new(pos.x + dx, pos.y + dy))
        .filter(move |test| map.get(*test) == Some(map.get(pos).unwrap() + 1))
}

struct StackElement<I: Iterator<Item = Vec2>> {
    pos: Vec2,
    adj: I,
}

/// Count the number of unique 9's it is possible to reach with teh P1 pathing rules
fn count_p1_paths(map: &Map2d<u8>, head: Vec2) -> u64 {
    let mut stack = Vec::with_capacity(10);
    let mut nines: HashSet<Vec2> = Default::default();

    stack.push(StackElement {
        pos: head,
        adj: adjacency(map, head),
    });

    while !stack.is_empty() {
        match stack.last_mut().unwrap().adj.next() {
            Some(next_pos) => stack.push(StackElement {
                pos: next_pos,
                adj: adjacency(map, next_pos),
            }),
            None => {
                if stack.len() == 10 {
                    nines.insert(stack.last().unwrap().pos);
                }
                stack.pop();
            }
        }
    }

    nines.len() as u64
}

pub fn solve_part_1(input: &Map2d<u8>) -> u64 {
    input
        .find_all(|x| *x == 0)
        .map(|head| count_p1_paths(input, head))
        .sum()
}

pub fn solve_part_2(input: &Map2d<u8>) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn test_part_1() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve_part_1(&input), 36);
    }
}
