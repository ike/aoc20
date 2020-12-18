use lazy_static::lazy_static;

pub fn run1() {
    lazy_static! {
        static ref INPUT: String = 
            std::fs::read_to_string("data/input-day-3.txt").unwrap();
    }

    let input_lines: Vec<Vec<char>> = INPUT.lines()
                                            .map(|s| 
                                                s.chars()
                                                .collect()
                                            ).collect();

    println!("You hit {} trees", calc_trees(&input_lines, 3, 1));
}

pub fn run2() {
    lazy_static! {
        static ref INPUT: String = 
            std::fs::read_to_string("data/input-day-3.txt").unwrap();
    }

    let input_lines: Vec<Vec<char>> = INPUT.lines()
                                            .map(|s| 
                                                s.chars()
                                                .collect()
                                            ).collect();

    let slopes: [i32; 5] = [
        calc_trees(&input_lines, 1, 1),
        calc_trees(&input_lines, 3, 1),
        calc_trees(&input_lines, 5, 1),
        calc_trees(&input_lines, 7, 1),
        calc_trees(&input_lines, 1, 2),
    ];

    println!("You hit {} trees", 
        slopes.iter()
        .fold(1i64, |acc, el| 
            acc * i64::from(el.clone())
        )
    );
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
