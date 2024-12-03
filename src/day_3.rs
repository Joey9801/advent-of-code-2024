/// A poor-man's parser combinator implementation
enum ParseResult<'a, T> {
    Ok { item: T, tail: &'a str },
    Fail { input: &'a str },
}

impl<'a, T> ParseResult<'a, T> {
    fn tail(&self) -> &'a str {
        match self {
            ParseResult::Ok { tail, .. } => tail,
            ParseResult::Fail { input } => input,
        }
    }
}

/// Skip a single any character
fn parse_any(input: &str) -> ParseResult<()> {
    ParseResult::Ok {
        item: (),
        tail: &input[1..],
    }
}

fn parse_lit<'a>(input: &'a str, lit: &str) -> ParseResult<'a, ()> {
    if let Some(tail) = input.strip_prefix(lit) {
        ParseResult::Ok { item: (), tail }
    } else {
        ParseResult::Fail { input }
    }
}

fn parse_int(input: &str) -> ParseResult<u64> {
    let int_len = input.chars().take_while(|c| c.is_ascii_digit()).count();

    if int_len == 0 {
        ParseResult::Fail { input }
    } else {
        ParseResult::Ok {
            item: input[..int_len].parse().unwrap(),
            tail: &input[int_len..],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MulStatement {
    a: u64,
    b: u64,
}

// Poor man's combination, not going to bother writing a generic macro for all
// cases like this
fn parse_mul_statement(input: &str) -> ParseResult<MulStatement> {
    let tail = match parse_lit(input, "mul(") {
        ParseResult::Ok { item: _, tail } => tail,
        ParseResult::Fail { .. } => return ParseResult::Fail { input },
    };

    let (a, tail) = match parse_int(tail) {
        ParseResult::Ok { item, tail } => (item, tail),
        ParseResult::Fail { .. } => return ParseResult::Fail { input },
    };

    let tail = match parse_lit(tail, ",") {
        ParseResult::Ok { item: _, tail } => tail,
        ParseResult::Fail { .. } => return ParseResult::Fail { input },
    };

    let (b, tail) = match parse_int(tail) {
        ParseResult::Ok { item, tail } => (item, tail),
        ParseResult::Fail { .. } => return ParseResult::Fail { input },
    };

    let tail = match parse_lit(tail, ")") {
        ParseResult::Ok { item: _, tail } => tail,
        ParseResult::Fail { .. } => return ParseResult::Fail { input },
    };

    ParseResult::Ok {
        item: MulStatement { a, b },
        tail,
    }
}

// Not going to bother writing a proper alternation macro, this special case will suffice for now
#[derive(Debug, PartialEq, Eq)]
pub enum Alternation {
    Do,
    Dont,
    Mul(MulStatement),
}

fn parse_alternation(input: &str) -> ParseResult<Alternation> {
    match parse_lit(input, "do()") {
        ParseResult::Ok { item: _, tail } => {
            return ParseResult::Ok {
                item: Alternation::Do,
                tail,
            }
        }
        ParseResult::Fail { .. } => (),
    }

    match parse_lit(input, "don't()") {
        ParseResult::Ok { item: _, tail } => {
            return ParseResult::Ok {
                item: Alternation::Dont,
                tail,
            }
        }
        ParseResult::Fail { .. } => (),
    }

    match parse_mul_statement(input) {
        ParseResult::Ok { item, tail } => ParseResult::Ok {
            item: Alternation::Mul(item),
            tail,
        },
        ParseResult::Fail { input } => ParseResult::Fail { input },
    }
}

pub fn parse(mut input: &str) -> Vec<Alternation> {
    assert!(input.is_ascii());

    let mut symbols = Vec::new();
    while !input.is_empty() {
        match parse_alternation(input) {
            ParseResult::Ok { item, tail } => {
                symbols.push(item);
                input = tail;
            }
            ParseResult::Fail { input: _ } => {
                input = parse_any(input).tail();
            }
        }
    }

    symbols
}

pub fn solve_part_1(input: &[Alternation]) -> u64 {
    input
        .iter()
        .filter_map(|sym| match sym {
            Alternation::Mul(MulStatement { a, b }) => Some(a * b),
            _ => None,
        })
        .sum()
}

pub fn solve_part_2(input: &[Alternation]) -> u64 {
    let mut result = 0;
    let mut enabled = true;

    for symbol in input {
        match symbol {
            Alternation::Do => enabled = true,
            Alternation::Dont => enabled = false,
            Alternation::Mul(ms) => {
                if enabled {
                    result += ms.a * ms.b;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(TEST_INPUT_2),
            [
                Alternation::Mul(MulStatement { a: 2, b: 4 }),
                Alternation::Dont,
                Alternation::Mul(MulStatement { a: 5, b: 5 }),
                Alternation::Mul(MulStatement { a: 11, b: 8 }),
                Alternation::Do,
                Alternation::Mul(MulStatement { a: 8, b: 5 }),
            ]
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(&parse(TEST_INPUT)), 161)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(&parse(TEST_INPUT_2)), 48)
    }
}
