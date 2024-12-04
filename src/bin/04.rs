advent_of_code::solution!(4);

fn check(board: &Vec<Vec<char>>, x: i32, y: i32, max_x: i32, max_y: i32, xdelt: i32, ydelt: i32) -> bool {
    if x + xdelt * 3 < 0 || x + xdelt * 3 >= max_x ||
       y + ydelt * 3 < 0 || y + ydelt * 3 >= max_y {
        return false;
       }
    board[y as usize][x as usize] == 'X' && board[(y + ydelt) as usize][(x + xdelt) as usize] == 'M' &&
       board[(y + ydelt * 2) as usize][(x + xdelt * 2) as usize] == 'A' && board[(y + ydelt * 3) as usize][(x + xdelt * 3) as usize] == 'S'
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = parse(input);
    let mut count = 0;
    for x in 0..board[0].len() {
        for y in 0..board.len() {
            for xdelt in -1..2 {
                for ydelt in -1..2 {
                    if check(&board, x as i32, y as i32, board[0].len() as i32, board.len() as i32, xdelt, ydelt) {
                        count += 1;
                    }
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = parse(input);
    let mut count = 0;
    for x in 1..board[0].len() - 1 {
        for y in 1..board.len() - 1 {
            if board[y][x] == 'A' &&
               ((board[y - 1][x - 1] == 'M' && board[y + 1][x + 1] == 'S') ||
                (board[y - 1][x - 1] == 'S' && board[y + 1][x + 1] == 'M')) &&
               ((board[y - 1][x + 1] == 'M' && board[y + 1][x - 1] == 'S') ||
                (board[y - 1][x + 1] == 'S' && board[y + 1][x - 1] == 'M')) {
                    count += 1;
                }
        }
    }
    Some(count)
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.split('\n').map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
