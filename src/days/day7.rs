use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use regex::Regex;

#[derive(Debug)]
struct Bag {
    count: i32,
    color: String 
}

impl Bag {
    fn new(count: i32, color: String) -> Bag {
        Self {count, color}
    }
}

pub fn run() {
    lazy_static! {
        static ref TARGET_COLOR: String = "shiny gold".to_string();
        static ref INPUT: String = 
            std::fs::read_to_string("data/input-day-7.txt")
                .unwrap()
                .strip_suffix("\n")
                .unwrap()
                .to_string();
    }

    let graph = parse_input(&INPUT);

    let part1 = count_contains(
        &reverse_graph(
            &graph 
        ), TARGET_COLOR.to_string()
    );

    let part2 = count_contains(
            &graph,
            TARGET_COLOR.to_string()
    );

    println!("total: {:?}, weighted: {:?}", part1.0, part1.1);
    println!("total: {:?}, weighted: {:?}", part2.0, part2.1);
}

fn count_contains(graph: &HashMap<String, Vec<Bag>>, target: String) -> (usize, i64) {
    let mut targets = Vec::new();
    let mut result = HashSet::new();
    let mut total_weights: i64 = 0;

    targets.push(target);

    while !targets.is_empty() {
        if let Some(value) = graph.get(&targets.pop().unwrap()) {
            value
                .iter()
                .map(|item| {
                    // Want to find faster way to do this, essentially
                    // only need to count all children of item n times
                    for _n in 0..item.count {
                        targets.push(item.color.to_string());
                    }
                    result.insert(item.color.to_string());
                    total_weights += i64::try_from(item.count).unwrap();
                })
                .for_each(drop);
        }
    }
    (result.len(), total_weights)
}

fn parse_input(input: &str) -> HashMap<String, Vec<Bag>> {
    let parse_parent_children = Regex::new(r"([\s\w]+?) bags contain ([\s\w,]+?)\.\n?").unwrap();
    let parse_child = Regex::new(r"(\d+) ([\s\w]+?) bag").unwrap();

    let mut graph = HashMap::<String, Vec<Bag>>::new();
    parse_parent_children
        .captures_iter(input)
        .map(|capture| {
            capture[2]
                .split(",")
                .map(|child| {
                    parse_child
                        .captures_iter(child)
                        .map(|edge| {
                            graph
                                .entry(capture[1].to_string())
                                .or_default()
                                .push(Bag::new(edge[1].parse().unwrap(), edge[2].to_string()))
                        })
                        .for_each(drop)
                })
                .for_each(drop)
        })
        .for_each(drop);
    graph
}

fn reverse_graph(graph: &HashMap<String, Vec<Bag>>) -> HashMap<String, Vec<Bag>> {
    let mut new_graph = HashMap::<String, Vec<Bag>>::new();
    graph
        .iter()
        .map(|(parent_color, parent_bags)| {
            parent_bags
                .iter()
                .map(|bag| {
                    new_graph
                        .entry(bag.color.to_string())
                        .or_default()
                        .push(Bag::new(bag.count, parent_color.to_string()))
                })
                .for_each(drop)
        })
        .for_each(drop);
    new_graph
}

