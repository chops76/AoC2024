advent_of_code::solution!(24);

use std::collections::HashMap;
use std::collections::HashSet;

fn parse(input: &str) -> (HashMap<String, i64>, Vec<Vec<String>>) {
    let spl = input.split("\n\n").collect::<Vec<&str>>();
    let mut hm = HashMap::new();
    for s in spl[0].lines() {
        let sp = s.split(": ").collect::<Vec<&str>>();
        hm.insert(sp[0].to_string(), sp[1].parse::<i64>().unwrap());
    }
    let mut v = Vec::new();
    for s in spl[1].lines() {
        let sp = s.split(" ").collect::<Vec<&str>>();
        v.push(vec![sp[0].to_string(), sp[1].to_string(), sp[2].to_string(), sp[4].to_string()]);
    }
    (hm, v)
}

fn get_result(hm: &HashMap<String, i64>, connections: &Vec<Vec<String>>) -> (i64, HashMap<String, i64>, HashSet<String>) {
    let mut vals = hm.clone();
    let mut used = HashSet::new();
    let mut changed = true;
    while changed {
        changed = false;
        for c in connections {
            if vals.contains_key(&c[0]) && vals.contains_key(&c[2]) && !vals.contains_key(&c[3]) {
                changed = true;
                used.insert(c[0].clone());
                used.insert(c[2].clone());
                if c[1] == "XOR" {
                    vals.insert(c[3].clone(), vals[&c[0]] ^ vals[&c[2]]);
                }
                else if c[1] == "AND" {
                    vals.insert(c[3].clone(), vals[&c[0]] & vals[&c[2]]);
                }
                else if c[1] == "OR" {
                    vals.insert(c[3].clone(), vals[&c[0]] | vals[&c[2]]);
                }
            }
        }
    }
    let mut count = 0;
    let mut cur = 0;
    let mut s = format!("z{:02}", 0);
    while vals.contains_key(&s) {
        cur += vals[&s] << count;
        count += 1;
        s = format!("z{:02}", count);
    }
    (cur, vals, used)
}

pub fn part_one(input: &str) -> Option<i64> {
    let (hm, connections) = parse(input);
    let (result, _, _) = get_result(&hm, &connections);
    Some(result)
}

fn expand_str(s: &str, hm: &HashMap<String, (String, String, String)>) -> String {
    if !hm.contains_key(s) {
        return s.to_string();
    }
    let rule = &hm[s];
    return format!("({} {} {})", rule.0, rule.1, rule.2);
}

// This code gave me output that allowed me to find the nodes that need to be swapped -
// it doesn't find them on its own.
pub fn part_two(input: &str) -> Option<u32> {
    let (_, connections) = parse(input);
    let mut hm = HashMap::new();
    for c in &connections {
        hm.insert(c[3].clone(), (c[0].clone(), c[1].clone(), c[2].clone()));
        if c[3] == c[0] || c[3] == c[2] {
            println!("Violating rule {} {} {} = {}", c[0], c[1], c[2], c[3]);
        }
    }
    let mut s = format!("z{:02}", 0);
    let mut count = 0;
    while hm.contains_key(&s) {
        let rule = &hm[&s];
        if count != 0 {
            let lrule = &hm[&rule.0];
            let rrule = &hm[&rule.2];
            println!("{}: ({} {} {}) {} ({} {} {})", s, expand_str(&lrule.0, &hm),
                     lrule.1, expand_str(&lrule.2, &hm), rule.1,
                     expand_str(&rrule.0, &hm), rrule.1,
                     expand_str(&rrule.2, &hm));
        } else {
            println!("{}: {:?}", s, rule);
        }

        count += 1;
        s = format!("z{:02}", count);
    }
    let mut vals = HashMap::new();
    for i in 0..45 {
        vals.insert(format!("x{:02}", i), 1);
        vals.insert(format!("y{:02}", i), 1);
    }
    let (total, results, used) = get_result(&vals, &connections);
    println!("Sum = {}", total);
    for i in 0..46 {
        let s = format!("z{:02}", i);
        println!("{}: {}", s, results[&s]);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }
}
