use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let input = read_to_string("input.txt").expect("input file not found");
    let now = std::time::Instant::now();
    println!("Part 1 = {}", calculate_floor(&input));
    println!("Part 2 = {}", calculate_floor_position(&input));
    println!("Execution time: {:?}", now.elapsed());
}

fn calculate_floor(input: &str) -> i32 {
    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in input.chars() {
        counts.insert(c, counts.get(&c).or_else(|| Some(&0)).unwrap() + 1);
    }
    return counts.get(&'(').unwrap() - counts.get(&')').unwrap()
}

fn calculate_floor_position(input: &str) -> i32 {
    let mut counts: HashMap<char, i32> = HashMap::from([('(', 0) , (')', 0)]);
    let mut position_count = 0;
    for c in input.chars() {
        position_count += 1;
        counts.insert(c, counts.get(&c).or_else(|| Some(&0)).unwrap() +1);
        if counts.get(&'(').unwrap() - counts.get(&')').unwrap() == -1 {
            return position_count;
        }
    }
    return 0;
}