use crate::util::Vec2;

#[derive(Debug)]
pub struct Machine {
    a: Vec2,
    b: Vec2,
    prize: Vec2,
}

pub fn parse(input: &str) -> Vec<Machine> {
    // Input like:
    //      Button A: X+94, Y+34
    //      Button B: X+22, Y+67
    //      Prize: X=8400, Y=5400
    //
    //      Button A: X+26, Y+66
    //      Button B: X+67, Y+21
    //      Prize: X=12748, Y=12176

    fn parse_line(line: &str) -> Vec2 {
        let (_, coords) = line.split_once(':').unwrap();
        let (x, y) = coords.split_once(',').unwrap();

        let x = x.trim()[2..].parse().unwrap();
        let y = y.trim()[2..].parse().unwrap();
        Vec2 { x, y }
    }

    input
        .split("\n\n")
        .map(|section| {
            let mut lines = section.lines();
            let a = parse_line(lines.next().unwrap());
            let b = parse_line(lines.next().unwrap());
            let prize = parse_line(lines.next().unwrap());

            Machine { a, b, prize }
        })
        .collect()
}

/// Find A, B, such that A*a + B*b = Prize
/// 
/// prize.x = A * a.x + Y * b.x
/// prize.y = A * a.y + Y * b.y
/// s.t X, Y >= 0, everything is an integer
fn min_machine_presses(machine: &Machine) -> Option<Vec2> {
    // [ a.x  b.x ] * [A] = [prize.x]
    // [ a.y  b.y ]   [B]   [prize.y]
    // 
    // [A] =  1 / (a.x*b.y - b.x*a.y) * [ b.y  -b.x ] * [ prize.x ]
    // [B]                              [ -a.y  a.x ]   [ prize.y ]
    // 
    // A = (b.y * prize.x - b.x * prize.y) / (a.x*b.y - b.x*a.y)
    // B = (-a.y * prize.x + a.x * prize.y) / (a.x*b.y - b.x*a.y)
    
    let Machine { a, b, prize } = *machine;
    
    let det = a.x * b.y - b.x * a.y;
    if det == 0 {
        return None;
    }
    
    let A = (b.y * prize.x - b.x * prize.y) / det;
    let B = (-a.y * prize.x + a.x * prize.y) / det;

    let soln = a * A + b * B;
    if soln == prize {
        Some(Vec2::new(A, B))
    } else {
        // Rounding error -> non integer solution
        None
    }
}

pub fn solve_part_1(input: &[Machine]) -> i64 {
    input
        .iter()
        .filter_map(min_machine_presses)
        .map(|presses| 3 * presses.x + presses.y)
        .sum()
}

pub fn solve_part_2(input: &[Machine]) -> i64 {
    input
        .iter()
        .map(|machine| Machine {
            prize: machine.prize + Vec2::new(10000000000000, 10000000000000),
            ..*machine
        })
        .filter_map(|machine| min_machine_presses(&machine))
        .map(|presses| 3 * presses.x + presses.y)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    #[test]
    fn test_machine_solve() {
        let input = parse(TEST_INPUT);
        assert_eq!(min_machine_presses(&input[0]), Some(Vec2::new(80, 40)));
        assert_eq!(min_machine_presses(&input[1]), None);
        assert_eq!(min_machine_presses(&input[2]), Some(Vec2::new(38, 86)));
        assert_eq!(min_machine_presses(&input[3]), None);
    }
    
    #[test]
    fn test_part_1() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve_part_1(&input), 480)
    }
}
