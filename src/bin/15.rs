advent_of_code::solution!(15);

use std::collections::HashSet;

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let spl = input.split("\n\n").collect::<Vec<&str>>();
    let board = spl[0].split('\n')
        .map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let moves = spl[1].chars().collect::<Vec<char>>();
    (board, moves)
}

fn simple_move(board: &mut Vec<Vec<char>>, m: char, cur_x: &mut i64, cur_y: &mut i64) {
    let mut check_x: i64 = *cur_x;
    let mut check_y: i64 = *cur_y;
    let mut dx: i64 = 0;
    let mut dy: i64 = -1;
    if m == '>' {
        dx = 1;
        dy = 0;
    } else if m == 'v' {
        dx = 0;
        dy = 1;
    } else if m == '<' {
        dx = -1;
        dy = 0;
    }
    while board[check_y as usize][check_x as usize] != '#' && 
          board[check_y as usize][check_x as usize] != '.' {
        check_x += dx;
        check_y += dy;
    }
    if board[check_y as usize][check_x as usize] == '.' {
        //println!("Moving {}  cur: {},{}  check: {},{}", m, cur_x, cur_y, check_x, check_y);
        while check_x != *cur_x || check_y != *cur_y {
            //println!("Moving piece from {},{} to {},{}", check_x - dx, check_y - dy, check_x, check_y);
            board[check_y as usize][check_x as usize] =
                board[(check_y - dy) as usize][(check_x - dx) as usize];
            check_x -= dx;
            check_y -= dy;
        }
        board[*cur_y as usize][*cur_x as usize] = '.';
        *cur_x += dx;
        *cur_y += dy;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (b, moves) = parse(input);
    let mut board = b.clone();
    let mut cur_x = 0;
    let mut cur_y = 0;
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == '@' {
                cur_x = x as i64;
                cur_y = y as i64;
                break;
            }
        }
    }
    for m in moves {
        if m == '\n' {
            continue;
        }
        simple_move(&mut board, m, &mut cur_x, &mut cur_y);
    }
    let mut sum = 0;
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == 'O' {
                sum += 100 * y + x;
            }
            //print!("{}", board[y][x]);
        }
        //println!("");
    }
    Some(sum)
}

fn expand_board(b: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut ret = Vec::new();
    for y in 0..b.len() {
        let mut line = Vec::new();
        for x in 0..b[y].len() {
            if b[y][x] == '.' {
                line.push('.');
                line.push('.');
            } else if b[y][x] == 'O' {
                line.push('[');
                line.push(']');
            } else if b[y][x] == '#' {
                line.push('#');
                line.push('#');
            } else {
                line.push('@');
                line.push('.');
            }
        }
        ret.push(line);
    }
    ret
}

fn complex_move(board: &mut Vec<Vec<char>>, m: char, cur_x: &mut i64, cur_y: &mut i64) {
    let dy: i64 = if m == '^' { -1 } else { 1 };
    let mut check_y = *cur_y;
    let mut stack = Vec::new();
    let mut pp = HashSet::new();
    pp.insert(*cur_x);
    stack.push(pp.clone());
    loop {
        check_y += dy;
        let obstacles = pp.iter().map(|x| board[check_y as usize][*x as usize]).collect::<HashSet<char>>();
        if obstacles.contains(&'#') {
            return;
        }
        if obstacles.len() == 1 && obstacles.contains(&'.') {
            break;
        }
        let mut new_pp = HashSet::new();
        for x_pos in &pp {
            if board[check_y as usize][*x_pos as usize] == ']' {
                new_pp.insert(*x_pos);
                new_pp.insert(x_pos - 1);
            } else if board[check_y as usize][*x_pos as usize] == '[' {
                new_pp.insert(*x_pos);
                new_pp.insert(x_pos + 1);
            }
        }
        stack.push(new_pp.clone());
        pp = new_pp;
    }
    while stack.len() != 0 {
        let pp = stack.pop().unwrap();
        for x_pos in pp {
            board[check_y as usize][x_pos as usize] = board[(check_y - dy) as usize][x_pos as usize];
            board[(check_y - dy) as usize][x_pos as usize] = '.';
        }
        check_y -= dy;
    }
    *cur_y += dy;
}

pub fn part_two(input: &str) -> Option<usize> {
    let (b, moves) = parse(input);
    let mut board = expand_board(&b);
    let mut cur_x = 0;
    let mut cur_y = 0;
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == '@' {
                cur_x = x as i64;
                cur_y = y as i64;
                break;
            }
        }
    }
    for m in moves {
        if m == '\n' {
            continue;
        }
        if m == '<' || m == '>' {
            simple_move(&mut board, m, &mut cur_x, &mut cur_y);
        } else {
            complex_move(&mut board, m, &mut cur_x, &mut cur_y);
        }
    }

    let mut sum = 0;
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == '[' {
                sum += 100 * y + x;
            }
            //print!("{}", board[y][x]);
        }
        //println!("");
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
