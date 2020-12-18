use lazy_static::lazy_static;

pub fn run() {
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

