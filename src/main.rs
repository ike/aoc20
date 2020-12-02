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

fn main() {
    day1::run(2, 2020);
    day1::run(3, 2020);
    day2::run(true);
    day2::run(false);
}
