advent_of_code::solution!(13);

use regex::Regex;

fn parse_line(input: &str) -> (i64, i64, i64, i64, i64, i64) {
    let re = Regex::new(r"[\+\=](\d+).+[\+\=](\d+)").unwrap();
    let mut caps = re.captures_iter(input);
    let c = caps.next().unwrap();
    let ax = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let ay = c.get(2).unwrap().as_str().parse::<i64>().unwrap();
    let c = caps.next().unwrap();
    let bx = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let by = c.get(2).unwrap().as_str().parse::<i64>().unwrap();
    let c = caps.next().unwrap();
    let x = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let y = c.get(2).unwrap().as_str().parse::<i64>().unwrap();
    return (ax, ay, bx, by, x, y)
}

fn parse(input: &str) -> Vec<(i64, i64, i64, i64, i64, i64)> {
    input.split("\n\n").map(|s| parse_line(s)).collect()
}

fn get_price(xa: i64, ya: i64, xb: i64, yb: i64, x: i64, y: i64) -> Option<i64> {
    let xa = xa as f64;
    let ya = ya as f64;
    let xb = xb as f64;
    let yb = yb as f64;
    let x = x as f64;
    let y = y as f64;
    let b = (y * xa - ya * x) / (xa * yb - xb * ya);
    let a = (x - b * xb) / xa;
    if a.fract() != 0.0 || b.fract() != 0.0 {
        return None
    }
    Some((3.0*a+b) as i64)
}

pub fn part_one(input: &str) -> Option<i64> {
    let vals = parse(input);
    let mut sum = 0;
    for v in vals {
        if let Some(v) = get_price(v.0, v.1, v.2, v.3, v.4, v.5) {
            sum += v;
        };
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let vals = parse(input);
    let mut sum = 0;
    for v in vals {
        if let Some(v) = get_price(v.0, v.1, v.2, v.3, v.4 + 10000000000000, v.5 + 10000000000000) {
            sum += v;
        };
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
