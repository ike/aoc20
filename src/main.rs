pub mod day1 {
    pub fn run() {
        use lazy_static::lazy_static;
        // Parse input
        lazy_static! {
            static ref INPUT: String =
                std::fs::read_to_string("data/input-day-1.txt").unwrap();
        }

        let input_lines: Vec<i32> = INPUT.lines().map(|s| s.parse().unwrap()).collect();
        let result = subsetsum(input_lines, 2020);

        println!("{:?}", result)
    }
    
    fn subsetsum(arr:Vec<i32>, sum:i32) -> Option<Vec<i32>> {
        if arr.iter().sum() == sum {
            Some(arr)
        } else if arr.len() > 1 {
            let end = arr.len() - 1;
            let mut slice = arr.as_slice();
            slice = &slice[1..end];

            for sub in slice {
                let result = subsetsum(sub, sum)
                match result {
                    Some => Some(result)
               }
            }
        } else {
            None
        }
    }
}

fn main() {
    day1::run()
}
