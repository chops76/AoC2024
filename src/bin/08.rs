advent_of_code::solution!(8);

use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;
use gcd::Gcd;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.split('\n').map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}

pub fn part_one(input: &str) -> Option<i64> {
    let board = parse(input);
    let x_max = board[0].len();
    let y_max = board.len();
    let mut nodes: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for y in 0..y_max {
        for x in 0..x_max {
            if board[y][x] != '.' {
                if !nodes.contains_key(&board[y][x]) {
                    nodes.insert(board[y][x], Vec::new());
                }
                nodes.get_mut(&board[y][x]).unwrap().push((x as i64, y as i64));
            }
        }
    }
    let mut antinodes = HashSet::new();
    for (k, v) in &nodes {
        for pair in v.iter().permutations(2) {
            let x_pos = 2 * pair[1].0 - pair[0].0;
            let y_pos = 2 * pair[1].1 - pair[0].1;
            if x_pos >= 0 && y_pos >= 0 && x_pos < (x_max as i64) && y_pos < (y_max as i64) {
                antinodes.insert((x_pos, y_pos));
            }
        }
    }
    Some(antinodes.len() as i64)
}

pub fn part_two(input: &str) -> Option<i64> {
    let board = parse(input);
    let x_max = board[0].len();
    let y_max = board.len();
    let mut nodes: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for y in 0..y_max {
        for x in 0..x_max {
            if board[y][x] != '.' {
                if !nodes.contains_key(&board[y][x]) {
                    nodes.insert(board[y][x], Vec::new());
                }
                nodes.get_mut(&board[y][x]).unwrap().push((x as i64, y as i64));
            }
        }
    }
    let mut antinodes = HashSet::new();
    for (k, v) in &nodes {
        for pair in v.iter().combinations(2) {
            let xd = pair[1].0 - pair[0].0;
            let yd = pair[1].1 - pair[0].1;
            let div = (xd.abs() as u64).gcd(yd.abs() as u64);
            let xd = xd / div as i64;
            let yd = yd / div as i64;
            let mut x_pos = pair[0].0;
            let mut y_pos = pair[0].1;
            while x_pos >= 0 && y_pos >= 0 && x_pos < (x_max as i64) && y_pos < (y_max as i64) {
                antinodes.insert((x_pos, y_pos));
                x_pos += xd;
                y_pos += yd;
            }
            x_pos = pair[0].0;
            y_pos = pair[0].1;
            while x_pos >= 0 && y_pos >= 0 && x_pos < (x_max as i64) && y_pos < (y_max as i64) {
                antinodes.insert((x_pos, y_pos));
                x_pos -= xd;
                y_pos -= yd;
            }
        }
    }
    Some(antinodes.len() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
