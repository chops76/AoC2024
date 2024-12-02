advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<Vec<i64>> {
    input.split('\n').map(|s| s.split_whitespace().map(|v| v.parse::<i64>().unwrap()).collect::<Vec<i64>>()).collect::<Vec<Vec<i64>>>()
}

fn safe(report: &Vec<i64>) -> bool {
    let diffs = report.iter().skip(1).zip(report).map(|(a, b)| a - b).collect::<Vec<i64>>();
    diffs.iter().all(|d| *d >= 1 && *d <= 3) || diffs.iter().all(|d| *d <= -1 && *d >= -3)
}

fn dampened_safe(report: &Vec<i64>) -> bool {
    for i in 0..report.len() {
        let mut rpt = report.clone();
        rpt.remove(i);

        let diffs = &rpt.iter().skip(1).zip(&rpt).map(|(a, b)| a - b).collect::<Vec<i64>>();
        if diffs.iter().all(|d| *d >= 1 && *d <= 3) || diffs.iter().all(|d| *d <= -1 && *d >= -3) {
            return true
        }
    }
    return false
}

pub fn part_one(input: &str) -> Option<usize> {
    let reports = parse(input);
    Some(reports.iter().filter(|v| safe(v)).count())

}

pub fn part_two(input: &str) -> Option<usize> {
    let reports = parse(input);
    Some(reports.iter().filter(|v| dampened_safe(v)).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
