advent_of_code::solution!(6);

use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.split('\n').map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let board = parse(input);
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut dir = 0;
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == '^' {
                x_pos = x;
                y_pos = y;
            }
        }
    }
    let mut visited = HashSet::new();
    visited.insert((x_pos, y_pos));
    while x_pos > 0 && y_pos > 0 && x_pos < board[0].len() - 1 && y_pos < board.len() - 1 {
        if dir == 0 {
            if board[y_pos - 1][x_pos] == '#' {
                dir = 1;
            } else {
                y_pos -= 1;
            }
        } else if dir == 1 {
            if board[y_pos][x_pos + 1] == '#' {
                dir = 2;
            } else {
                x_pos += 1;
            }
        } else if dir == 2 {
            if board[y_pos + 1][x_pos] == '#' {
                dir = 3;
            } else {
                y_pos += 1;
            }
        } else {
            if board[y_pos][x_pos - 1] == '#' {
                dir = 0;
            } else {
                x_pos -= 1;
            }
        }
        visited.insert((x_pos, y_pos));
    }
    Some(visited.len())
}

fn find_loop(board: &Vec<Vec<char>>, start_x: usize, start_y: usize) -> bool {
    let mut x_pos = start_x;
    let mut y_pos = start_y;
    let mut dir = 0;

    let mut visited = HashSet::new();
    visited.insert((x_pos, y_pos, dir));
    while x_pos > 0 && y_pos > 0 && x_pos < board[0].len() - 1 && y_pos < board.len() - 1 {
        if dir == 0 {
            if board[y_pos - 1][x_pos] == '#' {
                dir = 1;
            } else {
                y_pos -= 1;
            }
        } else if dir == 1 {
            if board[y_pos][x_pos + 1] == '#' {
                dir = 2;
            } else {
                x_pos += 1;
            }
        } else if dir == 2 {
            if board[y_pos + 1][x_pos] == '#' {
                dir = 3;
            } else {
                y_pos += 1;
            }
        } else {
            if board[y_pos][x_pos - 1] == '#' {
                dir = 0;
            } else {
                x_pos -= 1;
            }
        }
        if visited.contains(&(x_pos, y_pos, dir)) {
            return true;
        }
        visited.insert((x_pos, y_pos, dir));
    }
    false
}    

pub fn part_two(input: &str) -> Option<u32> {
    let board = parse(input);
    let mut x_start = 0;
    let mut y_start = 0;
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == '^' {
                x_start = x;
                y_start = y;
            }
        }
    }
    let mut found = 0;
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == '.' {
                let mut new_board = board.clone();
                new_board[y][x] = '#';
                if find_loop(&new_board, x_start, y_start) {
                    found += 1;
                }
            }
        }
    }    
    Some(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
