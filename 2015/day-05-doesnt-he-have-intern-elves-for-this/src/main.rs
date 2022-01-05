use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("input file no found");
    let now = std::time::Instant::now();
    println!("Part 1 = {}", check_string_part1(&input));
    println!("Part 2 = {}", check_string_part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}

fn check_string_part1(input: &str) -> usize {
    let mut counter = 0;

    for line in input.lines() {
        if !line.contains("ab") && !line.contains("cd") && !line.contains("pq") && !line.contains("xy") {
            let mut vowels = 0;
            let mut double = 0;
            let mut previous_c: char = '\0';
            for c in line.chars() {
                if "aeiou".contains(c) {
                    vowels += 1;
                }
                if c == previous_c {
                    double += 1;
                }
                previous_c = c;
            }
            if vowels >= 3 && double > 0 {
                counter += 1;
            }
        }
    }
    return counter;
}

fn check_string_part2(input: &str) -> usize {
    let mut counter = 0;

    for line in input.lines() {
        let mut pair = false;
        for m in 0..line.len()-2 {
            let n = m + 2;
            if line.matches(&line[m..n]).count() > 1 {
                pair = true;
                break;
            }
        }

        let mut at_least_one_letter_repeat = false;
        for m in 0..line.len()-2 {
            if line.chars().nth(m) == line.chars().nth(m+2) {
                at_least_one_letter_repeat = true;
            }
        }

        if pair && at_least_one_letter_repeat {
            counter += 1;
        }
    }
    return counter;
}