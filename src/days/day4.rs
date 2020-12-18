use std::str::FromStr;
use std::string::ParseError;
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

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
    lazy_static! {
        static ref INPUT: String = 
            std::fs::read_to_string("data/input-day-4.txt").unwrap();
    }

    let passports: Vec<Passport> = INPUT.split("\n\n")
                                        .map(|s| 
                                            s.replace("\n", " ")
                                            .parse()
                                            .unwrap()
                                        ).collect();

    let mut result = 0;

    for pass in passports {
        let valid = valid_pass(pass.clone());
        if valid {
            result = result + 1;
        }
    }
    
    println!("{} valid passports", result);
}

