use std::collections::HashMap;

struct Rule {
    a: i32,
    b: i32,
}

pub struct Input {
    rules: Vec<Rule>,
    updates: Vec<Vec<i32>>,
}

impl Input {
    fn max(&self) -> i32 {
        self.updates
            .iter()
            .flat_map(|update| update.iter())
            .copied()
            .max()
            .unwrap()
    }

    /// Asset that no update has a repeated value within the same update
    fn assert_updates_unique(&self) {
        for (idx, update) in self.updates.iter().enumerate() {
            let unique = update.iter().collect::<std::collections::HashSet<_>>();
            assert_eq!(
                update.len(),
                unique.len(),
                "Update {} has repeated values",
                idx
            );
        }
    }
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Input {
        self
    }
}

pub fn parse(input: &str) -> Input {
    // Input in the form of:
    // <rule_1_a>|<rule_1_b>
    // <rule_2_a>|<rule_2_b>
    // ...
    // <rule_n_a>|<rule_n_b>
    // <update_1_1>,<update_1_2>,...,<update_1_n>
    // <update_2_1>,<update_2_2>,...,<update_2_m>
    // ...

    let rules = input
        .lines()
        .take_while(|line| line.contains('|'))
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            Rule { a, b }
        })
        .collect();

    let updates = input
        .lines()
        .filter(|line| !line.is_empty())
        .skip_while(|line| line.contains('|'))
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    Input { rules, updates }
}

fn is_sorted(update: &[i32], rules: &[Rule], memo: &mut [Option<usize>]) -> bool {
    memo.fill(None);
    for (idx, value) in update.iter().enumerate() {
        memo[*value as usize] = Some(idx);
    }

    let mut sorted = true;
    for rule in rules {
        let a = memo[rule.a as usize];
        let b = memo[rule.b as usize];
        if let (Some(a), Some(b)) = (a, b) {
            if a > b {
                sorted = false;
                break;
            }
        }
    }
    sorted
}

pub fn solve_part_1(input: &Input) -> i32 {
    #[cfg(debug_assertions)]
    input.assert_updates_unique();

    let mut total = 0;
    let mut index_map = vec![None; input.max() as usize + 1];
    for update in &input.updates {
        if is_sorted(update, &input.rules, &mut index_map) {
            let mid = update[(update.len() - 1) / 2];
            total += mid;
        }
    }

    total
}

pub fn solve_part_2(input: &Input) -> i32 {
    // Map integers to the set of integers greater than them in the sorting
    let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in &input.rules {
        rule_map.entry(rule.a).or_default().push(rule.b);
    }

    let mut total = 0;
    let mut index_map = vec![None; input.max() as usize + 1];

    for update in &input.updates {
        if is_sorted(update, &input.rules, &mut index_map) {
            continue;
        }

        let mut update = update.clone();
        update.sort_by(|a, b| {
            if rule_map
                .get(a)
                .map_or(false, |b_values| b_values.contains(b))
            {
                std::cmp::Ordering::Less
            } else if rule_map
                .get(b)
                .map_or(false, |a_values| a_values.contains(a))
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        let mid = update[(update.len() - 1) / 2];
        total += mid;
    }
    total
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_part_1() {
        let input = super::parse(TEST_INPUT);
        assert_eq!(super::solve_part_1(&input), 143);
    }

    #[test]
    fn test_part_2() {
        let input = super::parse(TEST_INPUT);
        assert_eq!(super::solve_part_2(&input), 123);
    }
}
