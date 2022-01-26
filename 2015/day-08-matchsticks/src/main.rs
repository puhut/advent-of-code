use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("input file no found");
    let now = std::time::Instant::now();
    println!("Part 1 = {}", input.len() - decode(&input));
    println!("Part 2 = {}", solve_part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}

fn solve_part2(input: &str) -> usize {
    input.lines()
        .map(|line| encode(line) - line.len())
        .sum()
}

fn decode(input: &str) -> usize {
    let mut counts = 0;

    let mut characters = input.chars().peekable();
    while characters.peek().is_some() {
        let c = characters.next().unwrap();

        if c == '"' {
            continue;
        } else if c == '\\' {
            let next = characters.next().unwrap();
            match next {
                '\\' => {}
                '"' => {}
                'x' => {
                    assert!(characters.next().unwrap().is_ascii_hexdigit());
                    assert!(characters.next().unwrap().is_ascii_hexdigit());
                }
                _ => {}
            }
        }
        counts += 1;
    }
    return counts;
}

fn encode(input: &str) -> usize {
    let mut encoded_string = String::new();
    encoded_string.push('"');
    for c in input.chars() {
        match c {
            '"'  => encoded_string.push_str("\\\""),
            '\\' => encoded_string.push_str("\\\\"),
            _    => encoded_string.push(c),
        }
    }
    encoded_string.push('"');
    return encoded_string.len();
}
