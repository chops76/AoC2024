advent_of_code::solution!(10);

use std::collections::HashSet;
use std::collections::VecDeque;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input.split('\n').map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>()
}

fn find_9s(board: &Vec<Vec<u32>>, x_start: &usize, y_start: &usize) -> usize {
    let mut found = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((*x_start, *y_start, 0));
    while queue.len() != 0 {
        let (x, y, height) = queue.pop_front().unwrap();
        if x > 0 && board[y][x - 1] == height + 1 {
            if height == 8 {
                found.insert((x - 1, y));
            } else {
                queue.push_back((x - 1, y, height + 1));
            }
        }
        if y > 0 && board[y - 1][x] == height + 1 {
            if height == 8 {
                found.insert((x, y - 1));
            } else {
                queue.push_back((x, y - 1, height + 1));
            }
        }
        if x < board[y].len() - 1 && board[y][x + 1] == height + 1 {
            if height == 8 {
                found.insert((x + 1, y));
            } else {
                queue.push_back((x + 1, y, height + 1));
            }
        }
        if y < board.len() - 1 && board[y + 1][x] == height + 1 {
            if height == 8 {
                found.insert((x, y + 1));
            } else {
                queue.push_back((x, y + 1, height + 1));
            }
        }
    }
    found.len()
}

fn find_paths(board: &Vec<Vec<u32>>, x_start: &usize, y_start: &usize) -> usize {
    let mut sum = 0;
    let mut queue = VecDeque::new();
    queue.push_back((*x_start, *y_start, 0));
    while queue.len() != 0 {
        let (x, y, height) = queue.pop_front().unwrap();
        if x > 0 && board[y][x - 1] == height + 1 {
            if height == 8 {
                sum += 1;
            } else {
                queue.push_back((x - 1, y, height + 1));
            }
        }
        if y > 0 && board[y - 1][x] == height + 1 {
            if height == 8 {
                sum += 1;
            } else {
                queue.push_back((x, y - 1, height + 1));
            }
        }
        if x < board[y].len() - 1 && board[y][x + 1] == height + 1 {
            if height == 8 {
                sum += 1;
            } else {
                queue.push_back((x + 1, y, height + 1));
            }
        }
        if y < board.len() - 1 && board[y + 1][x] == height + 1 {
            if height == 8 {
                sum += 1;
            } else {
                queue.push_back((x, y + 1, height + 1));
            }
        }
    }
    sum
}

pub fn part_one(input: &str) -> Option<usize> {
    let board = parse(input);
    let mut sum = 0;
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == 0 {
                sum += find_9s(&board, &x, &y);
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let board = parse(input);
    let mut sum = 0;
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == 0 {
                sum += find_paths(&board, &x, &y);
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
