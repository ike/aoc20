use lazy_static::lazy_static;

pub fn run(old: bool) {
    lazy_static! {
        static ref INPUT: String = 
            std::fs::read_to_string("data/input-day-2.txt").unwrap();
    }

    let input_lines: Vec<String> = INPUT.lines().map(|s| s.parse().unwrap()).collect();

    let mut result: i32 = 0;

    for line in input_lines {
        
        let min: usize = line.split_whitespace()
                                .nth(0)
                                .unwrap()
                                .split("-")
                                .nth(0)
                                .unwrap()
                                .parse()
                                .unwrap();

        let max: usize = line.split_whitespace()
                                .nth(0)
                                .unwrap()
                                .split("-")
                                .nth(1)
                                .unwrap()
                                .parse()
                                .unwrap();

        let rule: char = line.split_whitespace()
                                .nth(1)
                                .unwrap()
                                .chars()
                                .nth(0)
                                .unwrap()
                                .clone();

        let pass: String = line.split_whitespace()
                                .nth(2)
                                .unwrap()
                                .to_ascii_lowercase();

        let count: usize = pass.matches(rule)
                                .count();

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
