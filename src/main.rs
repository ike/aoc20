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
}
