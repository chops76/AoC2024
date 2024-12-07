advent_of_code::solution!(7);

use itertools::Itertools;

#[derive(Debug)]
#[derive(PartialEq)]
enum Operation {
    Add,
    Mul,
    Concat
}

fn parse_line(input: &str) -> (i64, Vec<i64>) {
    let spl = input.split(": ").collect::<Vec<&str>>();
    let test_val = spl[0].parse().unwrap();
    let nums = spl[1].split(" ").map(|s| s.parse().unwrap()).collect::<Vec<i64>>();
    return (test_val, nums)
}

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    return input.split('\n').map(|s| parse_line(s)).collect::<Vec<(i64, Vec<i64>)>>();
}

pub fn part_one(input: &str) -> Option<i64> {
    let inputs = parse(input);
    let ops = vec![Operation::Add, Operation::Mul];
    let mut sum = 0;
    for (test_val, nums) in &inputs {
        for p in itertools::repeat_n(ops.iter(), nums.len() - 1).multi_cartesian_product() {
            let mut total = nums[0];
            for i in 0..p.len() {
                if *p[i] == Operation::Mul {
                    total *= nums[i + 1];
                } else {
                    total += nums[i + 1];
                }
            }
            if total == *test_val {
                sum += total;
                break;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let inputs = parse(input);
    let ops = vec![Operation::Add, Operation::Mul, Operation::Concat];
    let mut sum = 0;
    for (test_val, nums) in &inputs {
        for p in itertools::repeat_n(ops.iter(), nums.len() - 1).multi_cartesian_product() {
            let mut total = nums[0];
            for i in 0..p.len() {
                if *p[i] == Operation::Concat {
                    total *= 10_i64.pow(nums[i + 1].to_string().len() as u32);
                    total += nums[i + 1];
                }
                else if *p[i] == Operation::Mul {
                    total *= nums[i + 1];
                } else {
                    total += nums[i + 1];
                }
            }
            if total == *test_val {
                sum += total;
                break;
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
