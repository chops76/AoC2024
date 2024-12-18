advent_of_code::solution!(18);

use std::collections::HashSet;
use std::collections::VecDeque;

fn parse_line(line: &str) -> (i64, i64) {
    let spl = line.split(",").collect::<Vec<&str>>();
    (spl[0].parse().unwrap(), spl[1].parse().unwrap())
}

fn parse(input: &str) -> Vec<(i64, i64)> {
    input.split("\n").map(|s| parse_line(s)).collect::<Vec<(i64, i64)>>()
}

fn solve(coords: &Vec<(i64, i64)>, max_bytes: usize) -> Option<i64> {
    let END_X = 70;
    let END_Y = 70;

    let mut corrupt = HashSet::new();
    for i in 0..max_bytes {
        corrupt.insert(coords[i]);
    }
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((0, 0, 0));
    while let Some((x_pos, y_pos, dist)) = queue.pop_front() {
        if seen.contains(&(x_pos, y_pos)) {
            continue;
        }
        seen.insert((x_pos, y_pos));
        if x_pos == END_X && y_pos == END_Y {
            return Some(dist)
        }
        if x_pos > 0 && !corrupt.contains(&(x_pos - 1, y_pos)) {
            queue.push_back((x_pos - 1, y_pos, dist + 1));
        }
        if x_pos < END_X && !corrupt.contains(&(x_pos + 1, y_pos)) {
            queue.push_back((x_pos + 1, y_pos, dist + 1));
        }
        if y_pos > 0 && !corrupt.contains(&(x_pos, y_pos - 1)) {
            queue.push_back((x_pos, y_pos - 1, dist + 1));
        }
        if y_pos < END_Y && !corrupt.contains(&(x_pos, y_pos +1)) {
            queue.push_back((x_pos, y_pos + 1, dist + 1));
        }
    }
    None    
}

pub fn part_one(input: &str) -> Option<i64> {
    let coords = parse(input);
    solve(&coords, 1024)
}

pub fn part_two(input: &str) -> Option<String> {
    let coords = parse(input);
    let mut left = 1024;
    let mut right = coords.len();
    while right - left > 1 {
        let check = (left + right) / 2;
        let test_val = solve(&coords, check);
        if test_val == None {
            right = check;
        } else {
            left = check;
        }
    }
    Some(format!("{},{}", coords[left].0, coords[left].1))
}

