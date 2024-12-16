advent_of_code::solution!(16);

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
struct State {
    score: usize,
    facing: char,
    x_pos: usize,
    y_pos: usize,
    visited: Vec<(usize, usize, char)>
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let board = input.split('\n')
        .map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    board
}

pub fn part_one(input: &str) -> Option<usize> {
    let board = parse(input);

    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == 'S' {
                start_x = x;
                start_y = y;
            }
            if board[y][x] == 'E' {
                end_x = x;
                end_y = y;
            }
        }
    }
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    heap.push(State { score: 0, facing: 'E', x_pos: start_x, y_pos: start_y, visited: Vec::new() });
    while let Some(State { score, facing, x_pos, y_pos, visited }) = heap.pop() {
        if seen.contains(&(facing, x_pos, y_pos)) {
            continue;
        }
        if x_pos == end_x && y_pos == end_y {
            return Some(score);
        }
        seen.insert((facing, x_pos, y_pos));
        if facing == 'E' && board[y_pos][x_pos + 1] != '#' {
            heap.push(State { score: score + 1, facing: 'E', x_pos: x_pos + 1, y_pos: y_pos, visited: visited.clone() });
        } else if facing == 'W' && board[y_pos][x_pos - 1] != '#' {
            heap.push(State { score: score + 1, facing: 'W', x_pos: x_pos - 1, y_pos: y_pos, visited: visited.clone()  });
        } else if facing == 'N' && board[y_pos - 1][x_pos] != '#' {
            heap.push(State { score: score + 1, facing: 'N', x_pos: x_pos, y_pos: y_pos - 1, visited: visited.clone()  });
        } else if facing == 'S' && board[y_pos + 1][x_pos] != '#' {
            heap.push(State { score: score + 1, facing: 'S', x_pos: x_pos, y_pos: y_pos + 1, visited: visited.clone()  });
        }
        if facing == 'E' || facing == 'W' {
            heap.push(State { score: score + 1000, facing: 'N', x_pos: x_pos, y_pos: y_pos, visited: visited.clone()  });
            heap.push(State { score: score + 1000, facing: 'S', x_pos: x_pos, y_pos: y_pos, visited: visited.clone()  });
        } else {
            heap.push(State { score: score + 1000, facing: 'E', x_pos: x_pos, y_pos: y_pos, visited: visited.clone()  });
            heap.push(State { score: score + 1000, facing: 'W', x_pos: x_pos, y_pos: y_pos, visited: visited.clone()  });
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let board = parse(input);

    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == 'S' {
                start_x = x;
                start_y = y;
            }
            if board[y][x] == 'E' {
                end_x = x;
                end_y = y;
            }
        }
    }
    let mut heap = BinaryHeap::new();
    let mut lowest: HashMap<(usize, usize, char), usize> = HashMap::new();
    let mut found_score = usize::MAX;
    let mut all_seen:HashSet<(usize, usize)> = HashSet::new();
    heap.push(State { score: 0, facing: 'E', x_pos: start_x, y_pos: start_y, visited: Vec::new() });
    while let Some(State { score, facing, x_pos, y_pos, visited }) = heap.pop() {
        let mut new_visited = visited.clone();
        new_visited.push((x_pos, y_pos, facing));
        if score > found_score {
            break;
        }
        if !lowest.contains_key(&(x_pos, y_pos, facing)) {
            lowest.insert((x_pos, y_pos, facing), score);
        }
        if lowest[&(x_pos, y_pos, facing)] < score {
            continue;
        }
        if x_pos == end_x && y_pos == end_y {
            for pos in &new_visited {
                all_seen.insert((pos.0, pos.1));
            }
            found_score = score;
        }
        if facing == 'E' && board[y_pos][x_pos + 1] != '#' {
            heap.push(State { score: score + 1, facing: 'E', x_pos: x_pos + 1, y_pos: y_pos, visited: new_visited.clone() });
        } else if facing == 'W' && board[y_pos][x_pos - 1] != '#' {
            heap.push(State { score: score + 1, facing: 'W', x_pos: x_pos - 1, y_pos: y_pos, visited: new_visited.clone()  });
        } else if facing == 'N' && board[y_pos - 1][x_pos] != '#' {
            heap.push(State { score: score + 1, facing: 'N', x_pos: x_pos, y_pos: y_pos - 1, visited: new_visited.clone()  });
        } else if facing == 'S' && board[y_pos + 1][x_pos] != '#' {
            heap.push(State { score: score + 1, facing: 'S', x_pos: x_pos, y_pos: y_pos + 1, visited: new_visited.clone()  });
        }
        if facing == 'E' || facing == 'W' {
            heap.push(State { score: score + 1000, facing: 'N', x_pos: x_pos, y_pos: y_pos, visited: new_visited.clone()  });
            heap.push(State { score: score + 1000, facing: 'S', x_pos: x_pos, y_pos: y_pos, visited: new_visited.clone()  });
        } else {
            heap.push(State { score: score + 1000, facing: 'E', x_pos: x_pos, y_pos: y_pos, visited: new_visited.clone()  });
            heap.push(State { score: score + 1000, facing: 'W', x_pos: x_pos, y_pos: y_pos, visited: new_visited.clone()  });
        }
    }
    Some(all_seen.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
