use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("input file not found");
    let now = std::time::Instant::now();
    println!("Part 1 = {}", calculate_wrapping_paper(&input));
    println!("Part 2 = {}", calculate_ribbon(&input));
    println!("Execution time: {:?}", now.elapsed());
}

fn calculate_wrapping_paper(input: &str) -> u32 {
    let mut total_wrap = 0;
    for line in input.lines() {
        let mut numbers: Vec<u32> = line.split('x').map(|x| x.parse::<u32>().unwrap()).collect();
        numbers.sort();
        let wrap = 2*(numbers[0]*numbers[1] + numbers[1]*numbers[2] + numbers[2]*numbers[0]) + numbers[0]*numbers[1];
        total_wrap += wrap;
    }
    return total_wrap;
}

fn calculate_ribbon(input: &str) -> u32 {
    let mut total_ribbon = 0;
    for line in input.lines() {
        let mut numbers: Vec<u32> = line.split('x').map(|x| x.parse::<u32>().unwrap()).collect();
        numbers.sort();
        let ribbon = 2 * (numbers[0] + numbers[1]) + numbers[0] * numbers[1] * numbers[2];
        total_ribbon += ribbon;
    }
    return total_ribbon;
}