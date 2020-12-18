use lazy_static::lazy_static;
use std::collections::HashMap;

pub fn run() {
    lazy_static! {
        static ref INPUT: String = 
            std::fs::read_to_string("data/input-day-6.txt")
                .unwrap()
                .strip_suffix("\n")
                .unwrap()
                .to_string();
    }

    let answers: Vec<Vec<&str>> = INPUT.split("\n\n")
                                        .map(|s| 
                                            s.split("\n")
                                            .collect()
                                        ).collect();

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
