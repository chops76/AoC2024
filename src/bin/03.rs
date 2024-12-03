advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r".?mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r".?(?:mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();
    let mut sum = 0;
    let mut counting = true;
    for cap in re.captures_iter(input) {
        if cap.get(3) != None {
            counting = true;
        } else if cap.get(4) != None {
            counting = false;
        } else if counting {
            sum += cap.get(1).unwrap().as_str().parse::<u32>().unwrap() * cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
