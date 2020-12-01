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
}
