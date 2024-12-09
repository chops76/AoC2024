advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let mut memory = Vec::new();
    for (idx, c) in input.chars().enumerate() {
        let val = c.to_digit(10).unwrap();
        if idx % 2 == 0 {
            memory.extend(vec![(idx / 2) as i64; val as usize]);
        } else {
            memory.extend(vec![-1; val as usize]);
        }
    }
    let mut front_ptr = 0;
    let mut back_ptr = memory.len() - 1;
    while front_ptr < back_ptr {
        while memory[front_ptr] != -1 {
            front_ptr += 1;
        }
        while memory[back_ptr] == -1 {
            back_ptr -= 1;
        }
        if memory[front_ptr] == -1 && memory[back_ptr] != -1 && front_ptr < back_ptr {
            memory[front_ptr] = memory[back_ptr];
            memory[back_ptr] = -1;
        }
    }
    let mut sum = 0;
    for (idx, val) in memory.iter().enumerate() {
        if *val != -1 {
            sum += idx as i64 * val;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut free_space = Vec::new();
    let mut files = Vec::new();
    let mut pos = 0;
    for (idx, c) in input.chars().enumerate() {
        let val = c.to_digit(10).unwrap();
        if idx % 2 == 0 {
            files.push((pos, val, (idx / 2) as i64));
        } else {
            free_space.push((pos, val));
        }
        pos += val;
    }
    for i in (0..files.len()).rev() {
        for check_pos in 0..free_space.len() {
            if free_space[check_pos].0 >= files[i].0 {
                break;
            }
            if files[i].1 <= free_space[check_pos].1 {
                files[i].0 = free_space[check_pos].0;
                free_space[check_pos].0 += files[i].1;
                free_space[check_pos].1 -= files[i].1;
                break;
            }
        }
    }
    let mut sum: i64 = 0;
    for (p, l, id) in files {
        for i in 0..l {
            sum += id * (p + i) as i64;
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
