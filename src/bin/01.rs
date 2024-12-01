advent_of_code::solution!(1);

use std::collections::HashMap;

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for l in lines {
        let sp = l.split_ascii_whitespace().collect::<Vec<&str>>();
        left.push(sp[0].parse().unwrap());
        right.push(sp[1].parse().unwrap());
    }
    (left, right)

}

pub fn part_one(input: &str) -> Option<i64> {
    let (vec1, vec2) = parse(input);
    let mut v1 = vec1.clone();
    let mut v2 = vec2.clone();
    v1.sort();
    v2.sort();
    let v = v1.iter().zip(v2).map(|(a, b)| (a - b).abs()).sum::<i64>();
    Some(v)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (vec1, vec2) = parse(input);
    let frequencies = vec2
        .iter()
        .copied()
        .fold(HashMap::new(), |mut map, val|{
            map.entry(val)
            .and_modify(|frq|*frq+=1)
            .or_insert(1);
            map
        });
    let v = vec1.iter().map(|a| if !frequencies.contains_key(a) {0} else {a * frequencies[a]}).sum::<i64>();
    Some(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
