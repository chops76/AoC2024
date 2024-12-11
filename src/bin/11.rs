advent_of_code::solution!(11);

use std::collections::HashMap;

fn parse(input: &str) -> Vec<i64> {
    input.split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>()
}

fn calc(input: &str, blinks: usize) -> Option<usize> {
    let nums = parse(input);
    let mut list = nums.clone();
    for _ in 0..blinks {
        let mut new_list = Vec::new();
        for num in list {
            if num == 0 {
                new_list.push(1);
            } else {
                let num_digits = ((num as f64).log(10.0) + 1.0).floor() as i64;
                if num_digits % 2 == 0 {
                    new_list.push(num / 10_i64.pow((num_digits / 2) as u32));
                    new_list.push(num % 10_i64.pow((num_digits / 2) as u32));
                } else {
                    new_list.push(num * 2024);
                }
            }
        }
        list = new_list;
    }
    Some(list.len())
}

pub fn part_one(input: &str) -> Option<usize> {
    calc(input, 25)
}

pub fn part_two(input: &str) -> Option<u64> {
    let nums = parse(input);
    let mut hm:HashMap<i64, u64> = HashMap::new();
    for n in nums {
        if !hm.contains_key(&n) {
            hm.insert(n, 0);
        }
        *hm.get_mut(&n).unwrap() += 1;
    }
    for _ in 0..75 {
        let mut new_hm:HashMap<i64, u64> = HashMap::new();
        for (num, count) in &hm {
            if *num == 0 {
                if !new_hm.contains_key(&1) {
                    new_hm.insert(1, 0);
                }
                *new_hm.get_mut(&1).unwrap() += count;
            } else {
                let num_digits = ((*num as f64).log(10.0) + 1.0).floor() as i64;
                if num_digits % 2 == 0 {
                    let left = num / 10_i64.pow((num_digits / 2) as u32);
                    if !new_hm.contains_key(&left) {
                        new_hm.insert(left, 0);
                    }
                    *new_hm.get_mut(&left).unwrap() += count;
                    let right = num % 10_i64.pow((num_digits / 2) as u32);
                    if !new_hm.contains_key(&right) {
                        new_hm.insert(right, 0);
                    }
                    *new_hm.get_mut(&right).unwrap() += count;
                } else {
                    let new_val = num * 2024;
                    if !new_hm.contains_key(&new_val) {
                        new_hm.insert(new_val, 0);
                    }
                    *new_hm.get_mut(&new_val).unwrap() += count;
                }
            }
        }
        hm = new_hm;
    }
    let total = hm.iter().map(|(_, v)| v).sum::<u64>();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
