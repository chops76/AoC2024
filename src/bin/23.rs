advent_of_code::solution!(23);

use std::collections::HashMap;
use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<String>> {
    input.lines().map(|s| s.split("-").map(|st| st.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let strings = parse(input);
    let mut hm = HashMap::new();
    for s in strings {
        if !hm.contains_key(&s[0]) {
            hm.insert(s[0].clone(), HashSet::new());
        }
        hm.get_mut(&s[0]).unwrap().insert(s[1].clone());
        if !hm.contains_key(&s[1]) {
            hm.insert(s[1].clone(), HashSet::new());
        }
        hm.get_mut(&s[1]).unwrap().insert(s[0].clone());
    }
    
    let mut groups = HashSet::new();
    for (node, neighbors) in &hm {
        for n in neighbors {
            for common in neighbors.intersection(&hm[&n.clone()]) {
                let mut new_set = Vec::new();
                new_set.push(node.clone());
                new_set.push(n.clone());
                new_set.push(common.clone());
                new_set.sort();
                groups.insert(new_set);
            }
        }
    } 
    let mut count = 0;
    for s in groups {
        if s[0].chars().next().unwrap() == 't' ||
           s[1].chars().next().unwrap() == 't' ||
           s[2].chars().next().unwrap() == 't' {
            count += 1;
           }
    }
    Some(count)
}

fn bron_kerbosch(R: &HashSet<String>, P: &HashSet<String>, 
                 X: &HashSet<String>, found: &mut Vec<HashSet<String>>,
                 node_map: &HashMap<String, HashSet<String>>) {
    if P.len() == 0 && X.len() == 0 {
        found.push(R.clone());
    }
    let mut cur_p = P.clone();
    let mut cur_x = X.clone();
    for v in P {
        let mut new_r = R.clone();
        new_r.insert(v.clone());
        let new_p = cur_p.intersection(&node_map[v]).map(|s| s.clone()).collect::<HashSet<String>>();
        let new_x = cur_x.intersection(&node_map[v]).map(|s| s.clone()).collect::<HashSet<String>>();
        bron_kerbosch(&new_r, &new_p, &new_x, found, node_map);
        cur_p.remove(v);
        cur_x.insert(v.clone());
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let strings = parse(input);
    let mut hm = HashMap::new();
    for s in strings {
        if !hm.contains_key(&s[0]) {
            hm.insert(s[0].clone(), HashSet::new());
        }
        hm.get_mut(&s[0]).unwrap().insert(s[1].clone());
        if !hm.contains_key(&s[1]) {
            hm.insert(s[1].clone(), HashSet::new());
        }
        hm.get_mut(&s[1]).unwrap().insert(s[0].clone());
    }
    let mut results = Vec::new();
    let nodes = hm.keys().map(|s| s.clone()).collect::<HashSet<String>>();
    bron_kerbosch(&HashSet::new(), &nodes, &HashSet::new(), &mut results, &hm);
    let mut best = Vec::new();
    for r in results {
        if r.len() > best.len() {
            best = r.iter().cloned().collect::<Vec<String>>();
        }
    }
    best.sort();
    Some(best.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
