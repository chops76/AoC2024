advent_of_code::solution!(25);

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    input.split("\n\n").map(|s| parse_grid(s)).collect::<Vec<Vec<Vec<char>>>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grids = parse(input);
    let mut keys: Vec<Vec<usize>> = Vec::new();
    let mut locks: Vec<Vec<usize>> = Vec::new();
    for g in &grids {
        if g[0].iter().collect::<String>() == "#####" {
            let mut lock = vec![0, 0, 0, 0, 0];
            for y in 1..=5 {
                for x in 0..5 {
                    if g[y][x] == '#' {
                        lock[x] = y;
                    }
                }
            }
            locks.push(lock);
        } else {
            let mut key = vec![0, 0, 0, 0, 0];
            for y in 1..=5 {
                for x in 0..5 {
                    if g[6-y][x] == '#' {
                        key[x] = y;
                    }
                }
            }
            keys.push(key);
        }
    }
    let mut total = 0;
    for l in &locks {
        for k in &keys {
            if l.iter().zip(k.iter()).map(|(a, b)| a+b).all(|v| v <= 5) {
                total += 1;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
