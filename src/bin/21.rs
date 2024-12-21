advent_of_code::solution!(21);

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::sync::OnceLock;
use cached::proc_macro::cached;
use itertools::Itertools;

fn numeric_paths() -> &'static HashMap<(char, char), Vec<String>> {
    static NUMERIC_PATHS: OnceLock<HashMap<(char, char), Vec<String>>> = OnceLock::new();
    NUMERIC_PATHS.get_or_init(|| {
        let numeric_keypad = vec![
            ('7', vec![('4', 'v'), ('8', '>')]),
            ('8', vec![('5', 'v'), ('9', '>'), ('7', '<')]),
            ('9', vec![('6', 'v'), ('8', '<')]),
            ('4', vec![('1', 'v'), ('5', '>'), ('7', '^')]),
            ('5', vec![('2', 'v'), ('6', '>'), ('4', '<'), ('8', '^')]),
            ('6', vec![('3', 'v'), ('5', '<'), ('9', '^')]),
            ('1', vec![('2', '>'), ('4', '^')]),
            ('2', vec![('3', '>'), ('5', '^'), ('1', '<'), ('0', 'v')]),
            ('0', vec![('2', '^'), ('A', '>')]),
            ('3', vec![('6', '^'), ('2', '<'), ('A', 'v')]),
            ('A', vec![('0', '<'), ('3', '^')]),
        ]
        .into_iter()
        .collect::<HashMap<char, Vec<(char, char)>>>();

        numeric_keypad
            .keys()
            .cartesian_product(numeric_keypad.keys())
            .map(|(&a, &b)| ((a, b), find_shortest_paths(&numeric_keypad, a, b)))
            .collect()
    })
}

fn direction_paths() -> &'static HashMap<(char, char), Vec<String>> {
    static DIRECTION_PATHS: OnceLock<HashMap<(char, char), Vec<String>>> = OnceLock::new();
    DIRECTION_PATHS.get_or_init(|| {
        let direction_keypad = vec![
            ('^', vec![('A', '>'), ('v', 'v')]),
            ('A', vec![('^', '<'), ('>', 'v')]),
            ('>', vec![('A', '^'), ('v', '<')]),
            ('<', vec![('v', '>')]),
            ('v', vec![('<', '<'), ('^', '^'), ('>', '>')]),
        ]
        .into_iter()
        .collect::<HashMap<char, Vec<(char, char)>>>();

        direction_keypad
            .keys()
            .cartesian_product(direction_keypad.keys())
            .map(|(&a, &b)| ((a, b), find_shortest_paths(&direction_keypad, a, b)))
            .collect()
    })
}

fn find_shortest_paths(
    neighbors: &HashMap<char, Vec<(char, char)>>,
    start: char,
    end: char,
) -> Vec<String> {
    let mut queue = VecDeque::new();
    queue.push_back((start, Vec::new(), HashSet::new()));

    let mut paths = Vec::new();
    let mut lowest = std::usize::MAX;

    while let Some((node, path, mut visited)) = queue.pop_front() {
        if node == end {
            if path.len() < lowest {
                lowest = path.len();
                paths.clear();
                paths.push(path.iter().collect::<String>());
            } else if path.len() == lowest {
                paths.push(path.iter().collect::<String>());
            }
            continue;
        }

        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);

        for (next, dir) in neighbors.get(&node).unwrap() {
            let mut path = path.clone();
            path.push(*dir);
            queue.push_back((*next, path, visited.clone()));
        }
    }

    paths
}

#[cached]
fn find_shortest_sequence(sequence: String, depth: usize, numeric: bool) -> usize {
    let paths = if numeric {
        numeric_paths()
    } else {
        direction_paths()
    };

    ("A".to_string() + &sequence)
        .chars()
        .tuple_windows()
        .map(|(a, b)| {
            let shortest_paths = paths.get(&(a, b)).unwrap();
            match depth {
                0 => shortest_paths[0].len() + 1,
                _ => shortest_paths
                    .iter()
                    .cloned()
                    .map(|mut path| {
                        path.push('A');
                        find_shortest_sequence(path, depth - 1, false)
                    })
                    .min()
                    .unwrap(),
            }
        })
        .sum::<usize>()
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
        .lines()
        .map(|line| {
            find_shortest_sequence(line.to_string(), 2, true) as i64
                * line.trim_end_matches('A').parse::<i64>().unwrap()
        })
        .sum::<i64>())
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        input
        .lines()
        .map(|line| {
            find_shortest_sequence(line.to_string(), 25, true) as i64
                * line.trim_end_matches('A').parse::<i64>().unwrap()
        })
        .sum::<i64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }
}
