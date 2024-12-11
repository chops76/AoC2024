advent_of_code::solution!(11);

use std::collections::HashMap;

fn parse(input: &str) -> Vec<i64> {
    input.split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>()
}

fn calc(input: &str, blinks: usize) -> Option<u64> {
    let nums = parse(input);
    let mut hm:HashMap<i64, u64> = HashMap::new();
    for n in nums {
        *hm.entry(n).or_insert(0) += 1;
    }
    for _ in 0..blinks {
        let mut new_hm:HashMap<i64, u64> = HashMap::new();
        for (num, count) in &hm {
            if *num == 0 {
                *new_hm.entry(1).or_insert(0) += count;
            } else {
                let num_digits = ((*num as f64).log(10.0) + 1.0).floor() as i64;
                if num_digits % 2 == 0 {
                    let left = num / 10_i64.pow((num_digits / 2) as u32);
                    *new_hm.entry(left).or_insert(0) += count;

                    let right = num % 10_i64.pow((num_digits / 2) as u32);
                    *new_hm.entry(right).or_insert(0) += count;
                } else {
                    let new_val = num * 2024;
                    *new_hm.entry(new_val).or_insert(0) += count;
                }
            }
        }
        hm = new_hm;
    }
    let total = hm.iter().map(|(_, v)| v).sum::<u64>();
    Some(total)
}

pub fn part_one(input: &str) -> Option<u64> {
    calc(input, 25)
}

pub fn part_two(input: &str) -> Option<u64> {
    calc(input, 75)
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
