advent_of_code::solution!(5);

use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

fn parse(input: &str) -> (HashMap<i64, Vec<i64>>, HashMap<i64, Vec<i64>>, Vec<Vec<i64>>) {
    let spl = input.split("\n\n").collect::<Vec<&str>>();
    let mut hm1: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut hm2: HashMap<i64, Vec<i64>> = HashMap::new();
    for s in spl[0].split('\n') {
        let ds = s.split('|').collect::<Vec<&str>>();
        let lnum = ds[0].parse::<i64>().unwrap();
        let rnum = ds[1].parse::<i64>().unwrap();
        if !hm1.contains_key(&lnum) {
            hm1.insert(lnum, Vec::new());
        }
        hm1.get_mut(&lnum).unwrap().push(rnum);
        if !hm2.contains_key(&rnum) {
            hm2.insert(rnum, Vec::new());
        }
        hm2.get_mut(&rnum).unwrap().push(lnum);
    }
    let lines = spl[1].split('\n').map(|s| s.split(',').map(|v| v.parse::<i64>().unwrap()).collect::<Vec<i64>>()).collect::<Vec<Vec<i64>>>();
    (hm1, hm2, lines)
}

fn check_valid(line: &Vec<i64>, after: &HashMap<i64, Vec<i64>>, before: &HashMap<i64, Vec<i64>>) -> bool {
    for i in 0..line.len() {
        let left = &line[0..i];
        let right = &line[i+1..line.len()];
        let left_set: HashSet<i64> = left.iter().cloned().collect();
        let right_set: HashSet<i64> = right.iter().cloned().collect();

        if after.contains_key(&line[i]) {
            let after_set: HashSet<i64> = after[&line[i]].iter().cloned().collect();
            if left_set.intersection(&after_set).count() != 0 {
                return false;
            }
        }
        if before.contains_key(&line[i]) {
            let before_set: HashSet<i64> = before[&line[i]].iter().cloned().collect();
            if right_set.intersection(&before_set).count() != 0 {
                return false;
            }
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<i64> {
    let (after, before, lines) = parse(input);
    let mut sum = 0;
    for line in &lines {
        if check_valid(line, &after, &before) {
            sum += line[line.len() / 2];
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (after, before, lines) = parse(input);
    let mut sum = 0;
    for line in &lines {
        if check_valid(line, &after, &before) {
            continue;
        }
        let mut l = line.clone();
        l.sort_by(|a, b| {
            if after.contains_key(&a) && after[&a].contains(&b) {
                return Ordering::Less;
            }
            if before.contains_key(&a) && before[&a].contains(&b) {
                return Ordering::Greater;
            }
            return Ordering::Equal;
        });
        sum += l[l.len() / 2];
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
