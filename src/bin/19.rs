advent_of_code::solution!(19);

use std::collections::HashMap;

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let spl = input.split("\n\n").collect::<Vec<&str>>();
    let towels = spl[0].split(", ").collect::<Vec<&str>>();
    let patterns = spl[1].split("\n").collect::<Vec<&str>>();
    (towels, patterns)
}

pub fn possible(pattern: &str, towels: &Vec<&str>, hm: &mut HashMap<String, usize>) -> usize {
    if pattern == "" {
        return 1;
    }
    if hm.contains_key(pattern) {
        return hm[pattern];
    }
    let sum = towels.iter().filter(|t| pattern.starts_with(*t))
                           .map(|t| possible(&pattern[t.len()..], towels, hm)).sum();
    hm.insert(pattern.to_string(), sum);
    return sum;
}

pub fn part_one(input: &str) -> Option<usize> {
    let (towels, patterns) = parse(input);
    Some(patterns.iter().filter(|p| possible(p, &towels, &mut HashMap::new()) > 0).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (towels, patterns) = parse(input);
    Some(patterns.iter().map(|p| possible(p, &towels, &mut HashMap::new())).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
