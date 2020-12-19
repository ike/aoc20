use lazy_static::lazy_static;
use std::collections::HashSet;

pub fn run() {
    lazy_static! {
        static ref INPUT: String = 
            std::fs::read_to_string("data/input-day-8.txt")
                .unwrap()
                .strip_suffix("\n")
                .unwrap()
                .to_string();
    }

    run_part_1(INPUT.to_string());
    run_part_2(INPUT.to_string());
}

fn run_part_1(input: String) {
    let instructions: Vec<&str> = input.lines().collect();
    let mut history: HashSet<usize> = HashSet::new();
    let mut stack = Vec::<&str>::new();
    let mut acc: f32 = 0f32;
    let mut next_inst: usize = 0;

    stack.push(instructions[0]);

    while !stack.is_empty() {
        let instruction: Vec<&str> = stack.pop().unwrap().split(" ").collect();

        let result = run_instruction(next_inst, instruction[0], instruction[1]);
        next_inst = result.0;

        if history.insert(next_inst) {
            acc += result.1;
            if instructions.len() > next_inst {
                stack.push(instructions[next_inst]);
            }
        }
    }

    println!("{}", acc)
}

fn run_part_2(input: String) {
    let instructions: Vec<&str> = input.lines().collect();
    let mut history: HashSet<usize> = HashSet::new();
    let mut stack = Vec::<&str>::new();
    let mut acc: f32 = 0f32;
    let mut next_inst: usize = 0;

    stack.push(instructions[0]);

    // keep track of changes, and whether the
    // current run already has a changed 'jmp'
    // or 'nop'
    let mut changes: HashSet<usize> = HashSet::new();
    let mut unchanged = true;

    while !stack.is_empty() {
        let mut instruction: Vec<&str> = stack.pop().unwrap().split(" ").collect();

        if instruction[0] == "nop" || instruction[0] == "jmp" {
            if unchanged && changes.insert(next_inst) {
                instruction[0] = if instruction[0] == "nop" {
                    "jmp" 
                } else { 
                    "nop" 
                };

                unchanged = false;
            }
        }

        let result = run_instruction(next_inst, instruction[0], instruction[1]);
        next_inst = result.0;

        if history.insert(next_inst) {
            acc += result.1;
            if instructions.len() > next_inst {
                stack.push(instructions[next_inst]);
            }
        } else if unchanged == false { 
            // if we did change a value on this run
            // then we need to start over, since 
            // we are in an infinite loop
            next_inst = 0;
            acc = 0f32;
            stack.clear();
            stack.push(instructions[0]);
            history.clear();
            unchanged = true;
        }
    }

    println!("{}", acc)
}

fn run_instruction(line: usize, instruction: &str, value: &str) -> (usize, f32) {
    match instruction {
        "acc" => {
            (line + 1, value.parse().unwrap())
        },
        "jmp" => {
            let (sign, int) = value.split_at(1);
            let next_line: usize;
            if sign == "-" {
                next_line = line - usize::from_str_radix(int, 10).unwrap()
            } else {
                next_line = line + usize::from_str_radix(int, 10).unwrap()
            }
            (next_line, 0f32)
        },
        "nop" => {
            (line + 1, 0f32)
        },
        _ => (0, 0f32)
    }
}
