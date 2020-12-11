pub mod day6 {
    pub fn run() {
        use lazy_static::lazy_static;
        use std::collections::HashMap;

        lazy_static! {
            static ref INPUT: String = 
                std::fs::read_to_string("data/input-day-6.txt").unwrap().strip_suffix("\n").unwrap().to_string();
        }

        let answers: Vec<Vec<&str>> = INPUT.split("\n\n").map(|s| s.split("\n").collect() ).collect();

        let mut inclusive_count = 0;
        let mut exclusive_count = 0;

        for answer in answers {
            let mut set: HashMap<char, i32> = HashMap::new();

            for list in answer.clone() {
                for c in list.chars() {
                    let i = match set.get_mut(&c) {
                        Some(a) => *a + 1,
                        None => 1
                    };

                    *set.entry(c).or_insert(i) = i;
                }
            }

            inclusive_count = inclusive_count + set.len();

            for (_key, val) in set.iter() {
                if *val == answer.len() as i32 {
                    exclusive_count += 1;
                }
            }
        }

        println!("total inclusive answers: {}", inclusive_count);
        println!("total exclusive answers: {}", exclusive_count);
    }
}

pub mod day5 {
    pub fn run() {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref INPUT: String = 
                std::fs::read_to_string("data/input-day-5.txt").unwrap();
        }

        let mut ids = INPUT.lines()
            .map(get_seat)
            .collect::<Vec<i32>>();

        ids.sort();

        let highest_id = ids.last().unwrap();

        let missing_seat = ids
            .windows(2)
            .find(|w| w[0] + 1 != w[1])
            .unwrap()[0] + 1;

        println!("highest seat id: {:?}", highest_id);
        println!("missing seat id: {}", missing_seat);
    }

    fn get_seat(id: &str) -> i32 {
        id.chars()
            .map(|c| match c {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                _ => panic!("Invalid char"),
            })
            .fold(0, |a, b| (a << 1) + b)
    }
}

pub mod day4 {
    use std::str::FromStr;
    use std::string::ParseError;
    use std::collections::HashMap;
    use regex::Regex;

    #[derive(Debug, Clone)]
    struct Passport {
        byr: i32, 
        iyr: i32,
        eyr: i32,
        hgt: String,
        hcl: String,
        ecl: String,
        pid: String,
        cid: String,
    }

    impl FromStr for Passport {
        type Err = ParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut map = HashMap::new();

            let items = s.split_whitespace();

            for item in items {
                let pair: Vec<&str> = item.split(":").collect();
                map.insert(pair[0].to_string(), pair[1].to_string());
            }

            let passport = Passport {
                byr: map.get("byr").unwrap_or(&String::new()).parse::<i32>().unwrap_or(0).clone(),
                iyr: map.get("iyr").unwrap_or(&String::new()).parse::<i32>().unwrap_or(0).clone(),
                eyr: map.get("eyr").unwrap_or(&String::new()).parse::<i32>().unwrap_or(0).clone(),
                hgt: map.get("hgt").unwrap_or(&String::new()).clone(),
                hcl: map.get("hcl").unwrap_or(&String::new()).clone(),
                ecl: map.get("ecl").unwrap_or(&String::new()).clone(),
                pid: map.get("pid").unwrap_or(&String::new()).clone(),
                cid: map.get("cid").unwrap_or(&String::new()).clone(),
            };
            
            return Ok(passport)
        }
    }

    fn valid_pass(pass: Passport) -> bool {
        if pass.byr >= 1920 && pass.byr <= 2002 &&
            pass.iyr >= 2010 && pass.iyr <= 2020 &&
            pass.eyr >= 2020 && pass.eyr <= 2030 &&
            (
                (pass.hgt.ends_with("in") &&
                    pass.hgt.strip_suffix("in").unwrap().parse::<i32>().unwrap() >= 59 &&
                    pass.hgt.strip_suffix("in").unwrap().parse::<i32>().unwrap() <= 76
                ) ||
                (pass.hgt.ends_with("cm") &&
                    pass.hgt.strip_suffix("cm").unwrap().parse::<i32>().unwrap() >= 150 &&
                    pass.hgt.strip_suffix("cm").unwrap().parse::<i32>().unwrap() <= 193
                )
            ) &&
            Regex::new(r"#[0-9a-f]{6}\b").unwrap().is_match(&pass.hcl) &&
            ["amb","blu","brn","gry","grn","hzl","oth"].contains(&pass.ecl.as_str()) &&
            Regex::new(r"^[0-9]{9}\b").unwrap().is_match(&pass.pid)
        {
            return true
        } else {
            return false
        }
    }

    pub fn run() {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref INPUT: String = 
                std::fs::read_to_string("data/input-day-4.txt").unwrap();
        }

        let passports: Vec<Passport> = INPUT.split("\n\n").map(|s| s.replace("\n", " ").parse().unwrap()).collect();

        let mut result = 0;

        for pass in passports {
            let valid = valid_pass(pass.clone());
            if valid {
                result = result + 1;
            }
        }
        
        println!("{} valid passports", result);
    }
}

pub mod day3 {
    pub fn run1() {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref INPUT: String = 
                std::fs::read_to_string("data/input-day-3.txt").unwrap();
        }

        let input_lines: Vec<Vec<char>> = INPUT.lines().map(|s| s.chars().collect()).collect();

        println!("You hit {} trees", calc_trees(&input_lines, 3, 1));
    }

    pub fn run2() {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref INPUT: String = 
                std::fs::read_to_string("data/input-day-3.txt").unwrap();
        }

        let input_lines: Vec<Vec<char>> = INPUT.lines().map(|s| s.chars().collect()).collect();

        let slopes: [i32; 5] = [
            calc_trees(&input_lines, 1, 1),
            calc_trees(&input_lines, 3, 1),
            calc_trees(&input_lines, 5, 1),
            calc_trees(&input_lines, 7, 1),
            calc_trees(&input_lines, 1, 2),
        ];

        println!("You hit {} trees", slopes.iter().fold(1i64, |acc, el| acc * i64::from(el.clone())));
    }

    fn calc_trees(lines: &Vec<Vec<char>>, slope_x: usize, slope_y: usize) -> i32 {
        let mut result: i32 = 0;
        let mut current_x = 0;
        let mut current_y = 0;

        while current_y < lines.len() {
            let line = lines[current_y].clone();
            let spot = line.iter().cycle().nth(current_x).unwrap();
            if spot == &'#' {
                result = result + 1;
            }
            current_y = current_y + slope_y;
            current_x = current_x + slope_x;
        }
        return result
    }
}

pub mod day2 {
    pub fn run(old: bool) {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref INPUT: String = 
                std::fs::read_to_string("data/input-day-2.txt").unwrap();
        }

        let input_lines: Vec<String> = INPUT.lines().map(|s| s.parse().unwrap()).collect();

        let mut result: i32 = 0;

        for line in input_lines {
            
            let min: usize = line.split_whitespace().nth(0).unwrap().split("-").nth(0).unwrap().parse().unwrap();
            let max: usize = line.split_whitespace().nth(0).unwrap().split("-").nth(1).unwrap().parse().unwrap();
            let rule: char = line.split_whitespace().nth(1).unwrap().chars().nth(0).unwrap().clone();
            let pass: String = line.split_whitespace().nth(2).unwrap().to_ascii_lowercase();
            let count: usize = pass.matches(rule).count();

            if old {
                if count >= min && count <= max {
                    result = result + 1;
                } 
            } else {
                let pass_vec: Vec<char> = pass.chars().collect();
                if (pass_vec[min - 1] == rule) ^ (pass_vec[max - 1] == rule) {
                    result = result + 1;
                }
            } 
        }
        
        println!("Passing passwords: {}", result)
    }
}

pub mod day1 {
    pub fn run(kombinations: usize, target: i32) {
        use lazy_static::lazy_static;
        use itertools::Itertools;

        lazy_static! {
            static ref INPUT: String =
                std::fs::read_to_string("data/input-day-1.txt").unwrap();
        }

        let input_lines: Vec<i32> = INPUT.lines().map(|s| s.parse().unwrap()).collect();
        let result = input_lines.into_iter().combinations(kombinations).find(|v| v.iter().sum::<i32>() == target);

        let answer = result.clone().unwrap().into_iter().fold(1i32, |acc,val| acc * val);

        println!("For combination ({}): {:?}, and the answer is {}", kombinations, result, answer)
    }
}

fn main() {
    day1::run(2, 2020);
    day1::run(3, 2020);
    day2::run(true);
    day2::run(false);
    day3::run1();
    day3::run2();
    day4::run();
    day5::run();
    day6::run();
}
