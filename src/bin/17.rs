advent_of_code::solution!(17);

fn parse(input: &str) -> (i64, i64, i64, Vec<i64>) {
    let spl = input.split("\n").collect::<Vec<&str>>();
    let sp = spl[0].split(": ").collect::<Vec<&str>>();
    let a = sp[1].parse::<i64>().unwrap();
    let sp = spl[1].split(": ").collect::<Vec<&str>>();
    let b = sp[1].parse::<i64>().unwrap();
    let sp = spl[2].split(": ").collect::<Vec<&str>>();
    let c = sp[1].parse::<i64>().unwrap();
    let sp = spl[4].split(": ").collect::<Vec<&str>>();
    let prog = sp[1].split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    (a, b, c, prog)
}

fn get_combo(val: i64, a: i64, b: i64, c: i64) -> i64 {
    if val < 4 {
        return val;
    }
    if val == 4 {
        return a;
    }
    if val == 5 {
        return b;
    }
    if val == 6 {
        return c;
    }
    println!("Bad combo value");
    return 0;
}

pub fn part_one(input: &str) -> Option<String> {
    let params = parse(input);
    let mut a = params.0;
    let mut b = params.1;
    let mut c = params.2;
    let prog = params.3;
    let mut ip = 0;
    let mut out = String::new();
    while ip < prog.len() {
        if prog[ip] == 0 {
            // adv
            a /= 2_i64.pow(get_combo(prog[ip + 1], a, b, c) as u32);
            ip += 2;
        } else if prog[ip] == 1 {
            b ^= prog[ip + 1];
            ip += 2;
        } else if prog[ip] == 2 {
            b = get_combo(prog[ip + 1], a, b, c) % 8;
            ip += 2;
        } else if prog[ip] == 3 {
            if a == 0 {
                ip += 2;
            } else {
                ip = prog[ip + 1] as usize;
            }
        } else if prog[ip] == 4 {
            b ^= c;
            ip += 2;
        } else if prog[ip] == 5 {
            if out == String::new() {
                out = (get_combo(prog[ip + 1], a, b, c) % 8).to_string();
            } else {
                out += ",";
                out += &(get_combo(prog[ip + 1], a, b, c) % 8).to_string();
            }
            ip += 2;
        } else if prog[ip] == 6 {
            b = a / 2_i64.pow(get_combo(prog[ip + 1], a, b, c) as u32);
            ip += 2;
        } else if prog[ip] == 7 {
            c = a / 2_i64.pow(get_combo(prog[ip + 1], a, b, c) as u32);
            ip += 2;
        }
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<u32> {
    // I'm not proud of this... but it got me on the leaderboard
    for i in 0..8 {
        println!("i: {}", i);
        let mut a: i64 = 7;
        a = a * 8 + 2; // 4
        a = a * 8 + 6;
        a = a * 8 + 0; // 6
        a = a * 8 + 1;
        a = a * 8 + 1; // 6
        a = a * 8 + 0; // 1, 2, 7
        a = a * 8 + 5; // 5
        a = a * 8 + 2;
        a = a * 8 + 2; // 5, 7
        a = a * 8 + 6;
        a = a * 8 + 2;
        a = a * 8 + 1;
        a = a * 8 + 6;
        a = a * 8 + 3;
        a = a * 8 + 3;
        println!("{}", a);
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        while a != 0 {
            b = a % 8;
            b = b ^ 7;
            c = a / 2_i64.pow(b as u32);
            a /= 8;
            b = b ^ c;
            b = b ^ 7;
            println!("{}", b % 8);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }
}
