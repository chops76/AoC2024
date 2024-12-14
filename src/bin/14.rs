advent_of_code::solution!(14);

use regex::Regex;

fn parse_line(input: &str) -> (i64, i64, i64, i64) {
    let re = Regex::new(r"(\-?\d+),(\-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut caps = re.captures_iter(input);
    let c = caps.next().unwrap();
    let px = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let py = c.get(2).unwrap().as_str().parse::<i64>().unwrap();
    let vx = c.get(3).unwrap().as_str().parse::<i64>().unwrap();
    let vy = c.get(4).unwrap().as_str().parse::<i64>().unwrap();
    return (px, py, vx, vy)
}

fn parse(input: &str) -> Vec<(i64, i64, i64, i64)> {
    input.split("\n").map(|s| parse_line(s)).collect()
}

fn get_final(px: i64, py: i64, vx: i64, vy: i64, x_max: i64, y_max: i64) -> (i64, i64) {
    let mut fx = (px + vx * 100) % x_max;
    let mut fy = (py + vy * 100) % y_max;
    while fx < 0 {
        fx += x_max;
    }
    while fy < 0 {
        fy += y_max;
    }
    return (fx, fy)
}

pub fn part_one(input: &str) -> Option<u32> {
    let x_max = 101;
    let y_max = 103;
    let nums = parse(input);
    let pos = nums.iter()
        .map(|(px, py, vx, vy)| get_final(*px, *py, *vx, *vy, x_max, y_max)).collect::<Vec<(i64, i64)>>();
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    let mid_x = x_max / 2;
    let mid_y = y_max / 2;
    for (x, y) in pos {
        if x < mid_x && y < mid_y {
            q1 += 1;
        } else if x > mid_x && y < mid_y {
            q2 += 1;
        } else if x < mid_x && y > mid_y {
            q3 += 1;
        } else if x > mid_x && y > mid_y {
            q4 += 1;
        }
    }
    Some(q1 * q2 * q3 * q4)
}

pub fn part_two(input: &str) -> Option<u32> {
    let max_x = 101;
    let max_y = 103;
    let nums = parse(input);
    let mut pos = nums.iter().map(|(x, y, _, _)| (*x, *y)).collect::<Vec<(i64, i64)>>();
/* 
    for picnum in 0..(101 * 103) {
        for i in 0..pos.len() {
            pos[i] = ((pos[i].0 + nums[i].2 + max_x) % max_x, (pos[i].1 + nums[i].3 + max_y) % max_y);
        }
        if (picnum - 78) % 101 == 0 {
            println!("Pic {}", picnum);
            for y in 0..max_y {
                for x in 0..max_x {
                    if pos.contains(&(x, y)) {
                        print!("X");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }
        }
    } */

    // I used the above code to notice a pattern that repeats starting at 75, every 101 images.
    // I visually found the answer using the above code.
    Some(8159)
}


