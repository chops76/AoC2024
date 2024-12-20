advent_of_code::solution!(20);

use std::collections::HashMap;

fn parse(input: &str) -> Vec<Vec<char>> {
    let board = input.split('\n')
        .map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    board
}

fn solve(board: &Vec<Vec<char>>, ns: usize, min_savings: usize) -> usize {
    let mut cur_x = 0;
    let mut cur_y = 0;
    for y in 0..board.len() {
        for x in 0..board.len() {
            if board[y][x] == 'S' {
                cur_x = x;
                cur_y = y;
            }
        }
    }
    let mut hm = HashMap::new();
    let mut path = Vec::new();
    path.push((cur_x, cur_y));
    while board[cur_y][cur_x] != 'E' {
        if board[cur_y][cur_x + 1] != '#' && !path.contains(&(cur_x + 1, cur_y)) {
            cur_x += 1;
            path.push((cur_x, cur_y));
        } else if board[cur_y][cur_x - 1] != '#' && !path.contains(&(cur_x - 1, cur_y)) {
            cur_x -= 1;
            path.push((cur_x, cur_y));
        } else if board[cur_y + 1][cur_x] != '#' && !path.contains(&(cur_x, cur_y + 1)) {
            cur_y += 1;
            path.push((cur_x, cur_y));
        } else if board[cur_y - 1][cur_x] != '#' && !path.contains(&(cur_x, cur_y - 1)) {
            cur_y -= 1;
            path.push((cur_x, cur_y));
        }
    }
    path.push((cur_x, cur_y));
    for i in 0..path.len() {
        for j in i..path.len() {
            let dist = ((path[i].0 as i32 - path[j].0 as i32).abs() + (path[i].1 as i32 - path[j].1 as i32).abs()) as usize;
            let elapsed = j - i;
            if dist <= ns && elapsed > dist {                 
                    let saved = elapsed - dist;
                    if saved % 2 == 0 {
                        if !hm.contains_key(&saved) {
                            hm.insert(saved, 0);
                        }
                        *hm.get_mut(&saved).unwrap() += 1;
                    }
                }
        }
    }
    hm.iter().filter(|(k, _)| **k >= min_savings).map(|(_, v)| v).sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let board = parse(input);

    Some(solve(&board, 2, 100))
}

pub fn part_two(input: &str) -> Option<usize> {
    let board = parse(input);

    Some(solve(&board, 20, 100))
}
