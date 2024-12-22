advent_of_code::solution!(22);

use std::collections::HashMap;
use std::collections::HashSet;

fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<i64>>()
}

fn find_secret(initial: i64, iterations: usize) -> i64 {
    let mut val = initial;
    for _ in 0..iterations {
        val ^= val * 64;
        val %= 16777216;
        val ^= val / 32;
        val %= 16777216;
        val ^= val * 2048;
        val %= 16777216;
    }
    val
}

fn find_prices(initial: i64, iterations: usize) -> Vec<i64> {
    let mut ret_vec = Vec::new();
    ret_vec.push(initial % 10);
    let mut val = initial;
    for _ in 0..iterations {
        val ^= val * 64;
        val %= 16777216;
        val ^= val / 32;
        val %= 16777216;
        val ^= val * 2048;
        val %= 16777216;
        ret_vec.push(val % 10);
    }
    ret_vec
}

fn calc_deltas(vals: &Vec<i64>) -> Vec<i64> {
    vals.iter().zip(vals.iter().skip(1)).map(|(a, b)| b - a).collect::<Vec<i64>>()
}

fn calc_map(vals: &Vec<Vec<i64>>) -> HashMap<(i64, i64, i64, i64), i64> {
    let mut ret_map = HashMap::new();
    for monkey in vals {
        let deltas = calc_deltas(monkey);
        let mut seen = HashSet::new();
        for i in 0..deltas.len() - 3 {
            let seq = (deltas[i], deltas[i+1], deltas[i+2], deltas[i+3]);
            if !seen.contains(&seq) {
                if !ret_map.contains_key(&seq) {
                    ret_map.insert(seq, 0);
                }
                *ret_map.get_mut(&seq).unwrap() += monkey[i + 4];
                seen.insert(seq);
            }
        }
    }

    ret_map
}

pub fn part_one(input: &str) -> Option<i64> {
    let vals = parse(input);
    Some(vals.iter().map(|&v| find_secret(v, 2000)).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let vals = parse(input);
    let prices = vals.iter().map(|&v| find_prices(v, 2000)).collect::<Vec<Vec<i64>>>();
    let seq_map = calc_map(&prices); 
    Some(*seq_map.iter().map(|(_, v)| v).max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37990510));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
