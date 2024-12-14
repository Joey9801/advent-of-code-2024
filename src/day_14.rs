use crate::util::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    pos: Vec2,
    vel: Vec2,
}

pub fn parse(input: &str) -> Vec<Robot> {
    // Each line of input in form:
    //   p=<pos.x>,<pos.y> v=<vel.x>,<vel.y>

    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let pos = parts.next().unwrap();
            let vel = parts.next().unwrap();

            let (pos_x, pos_y) = pos[2..].split_once(',').unwrap();
            let pos = Vec2::new(pos_x.parse().unwrap(), pos_y.parse().unwrap());

            let (vel_x, vel_y) = vel[2..].split_once(',').unwrap();
            let vel = Vec2::new(vel_x.parse().unwrap(), vel_y.parse().unwrap());

            Robot { pos, vel }
        })
        .collect()
}

// 3 % 10 => 3
// -3 % 10 => 7
fn signed_mod(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

fn move_robots(robots: &mut [Robot], size: Vec2) {
    for robot in robots.iter_mut() {
        robot.pos += robot.vel;
        robot.pos.x = signed_mod(robot.pos.x, size.x);
        robot.pos.y = signed_mod(robot.pos.y, size.y);
    }
}

fn p1_inner(input: &[Robot], size: Vec2) -> u64 {
    debug_assert_eq!(size.x % 2, 1);
    debug_assert_eq!(size.y % 2, 1);

    let center = Vec2::new(size.x / 2, size.y / 2);

    let mut robots = input.to_vec();
    for _ in 0..100 {
        move_robots(&mut robots, size);
    }

    let mut quadrant_counts = [0; 4];
    for robot in robots.iter() {
        if robot.pos.x == center.x || robot.pos.y == center.y {
            continue;
        }

        let quadrant = match (robot.pos.x < center.x, robot.pos.y < center.y) {
            (true, true) => 0,
            (false, true) => 1,
            (true, false) => 2,
            (false, false) => 3,
        };
        quadrant_counts[quadrant] += 1;
    }
    
    quadrant_counts.iter().product()
}

const REAL_SIZE: Vec2 = Vec2::new(101, 103);

pub fn solve_part_1(input: &[Robot]) -> u64 {
    p1_inner(input, REAL_SIZE)
}

/// Test whether the given set of robots contains a vertical line at least 10 units long.
fn p2_heuristic(robots: &[Robot], storage: &mut Vec<Vec2>) -> bool {
    storage.clear();
    storage.extend(robots.iter().map(|robot| robot.pos));
    storage.sort_by_key(|pos| (pos.x, pos.y));
    
    let mut last = storage[0];
    let mut count = 1;

    for pos in storage.iter().skip(1) {
        if pos.x == last.x && (pos.y == last.y || pos.y == last.y + 1) {
            count += 1;
        } else {
            count = 1;
        }

        if count >= 10 {
            return true;
        }

        last = *pos;
    }
    
    false
}

pub fn solve_part_2(input: &[Robot]) -> u64 {
    let mut robots = input.to_vec();
    let mut storage = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        move_robots(&mut robots, REAL_SIZE);
        if p2_heuristic(&robots, &mut storage) {
            return i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_INPUT: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn test_part_1() {
        let input = parse(TEST_INPUT);
        assert_eq!(p1_inner(&input, Vec2::new(11, 7)), 12);
    }
}
