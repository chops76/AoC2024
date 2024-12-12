advent_of_code::solution!(12);

use std::collections::HashSet;
use std::collections::VecDeque;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.split('\n').map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}

fn find_attached(board: &Vec<Vec<char>>, x_start: &usize, y_start: &usize) -> HashSet<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut attached = HashSet::new();
    let mut checked = HashSet::new();
    attached.insert((*x_start, *y_start));
    queue.push_back((*x_start, *y_start));
    while queue.len() != 0 {
        let (x, y) = queue.pop_front().unwrap();
        if checked.contains(&(x, y)) {
            continue;
        }
        if x > 0 && board[y][x - 1] == board[y][x] {
            attached.insert((x - 1, y));
            queue.push_back((x - 1, y));
        }
        if y > 0 && board[y - 1][x] == board[y][x] {
            attached.insert((x, y - 1));
            queue.push_back((x, y - 1));
        }
        if x < board[y].len() - 1 && board[y][x + 1] == board[y][x] {
            attached.insert((x + 1, y));
            queue.push_back((x + 1, y));
        }
        if y < board.len() - 1 && board[y + 1][x] == board[y][x] {
            attached.insert((x, y + 1));
            queue.push_back((x, y + 1));
        }
        checked.insert((x, y));
    }
    attached
}

fn calc_perimeter(region: &HashSet<(usize, usize)>) -> usize {
    let mut sum = 0;
    for (x, y) in region.iter() {
        if *x == 0 || !region.contains(&(*x - 1, *y)) {
            sum += 1;
        }
        if *y == 0 || !region.contains(&(*x, *y - 1)) {
            sum += 1;
        }
        if !region.contains(&(*x + 1, *y)) {
            sum += 1;
        }
        if !region.contains(&(*x, *y + 1)) {
            sum += 1;
        }
    }
    sum
}

pub fn part_one(input: &str) -> Option<usize> {
    let board = parse(input);
    let mut unseen = HashSet::new();
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            unseen.insert((x, y));
        }
    }
    let mut sum = 0;
    while let Some((x, y)) = unseen.iter().next().cloned() {
        let region = find_attached(&board, &x, &y);
        for c in region.iter() {
            if unseen.contains(c) {
                unseen.remove(c);
            }
        }
        sum += region.len() * calc_perimeter(&region);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let board = parse(input);
    let mut unseen = HashSet::new();
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            unseen.insert((x, y));
        }
    }
    let mut sum = 0;
    while let Some((x, y)) = unseen.iter().next().cloned() {
        let region = find_attached(&board, &x, &y);
        for c in region.iter() {
            if unseen.contains(c) {
                unseen.remove(c);
            }
        }
        let mut sides = 0;
        for y in 0..board.len() {
            let mut top_on = false;
            let mut bot_on = false;
            for x in 0..board[y].len() {
                if region.contains(&(x, y)) && (y == 0 || !region.contains(&(x, y - 1))) {
                    if !top_on {
                        top_on = true;
                        sides += 1;
                    }
                } else {
                    top_on = false;
                }
                if region.contains(&(x, y)) && !region.contains(&(x, y + 1)) {
                    if !bot_on {
                        bot_on = true;
                        sides += 1;
                    }
                } else {
                    bot_on = false;
                }
            }
        }
        for x in 0..board[0].len() {
            let mut left_on = false;
            let mut right_on = false;
            for y in 0..board.len() {
                if region.contains(&(x, y)) && (x == 0 || !region.contains(&(x - 1, y))) {
                    if !left_on {
                        left_on = true;
                        sides += 1;
                    }
                } else {
                    left_on = false;
                }
                if region.contains(&(x, y)) && !region.contains(&(x + 1, y)) {
                    if !right_on {

                        right_on = true;
                        sides += 1;
                    }
                } else {
                    right_on = false;
                }
            }
        }
        sum += sides * region.len();
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
