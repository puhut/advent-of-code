use std::fs::read_to_string;
use std::collections::HashSet;

type Point = (i16, i16);

fn main() {
    let input = read_to_string("input.txt").expect("input file not found");
    let now = std::time::Instant::now();
    println!("Part 1 = {}", calculate_delivered_house(&input));
    println!("Part 2 = {}", calculate_delivered_house_with_robot(&input));
    println!("Execution time: {:?}", now.elapsed());
}

fn calculate_delivered_house(input: &str) -> usize {
    let mut delivered_houses = HashSet::<Point>::new();
    let mut x = 0;
    let mut y = 0;

    delivered_houses.insert((x,y));

    for c in input.chars() {
        match c {
            '>' => x += 1,
            '^' => y += 1,
            '<' => x -= 1,
            'v' => y -= 1,
            _   => ()
        }
        delivered_houses.insert((x,y));
    }
    return delivered_houses.len();
}

fn calculate_delivered_house_with_robot(input: &str) -> usize {
    let mut delivered_houses = HashSet::<Point>::new();
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robot_x = 0;
    let mut robot_y = 0;
    let mut even_or_odd = 0;

    delivered_houses.insert((santa_x, santa_y));

    for c in input.chars() {
        if even_or_odd % 2 == 0 {
            match c {
                '>' => santa_x += 1,
                '^' => santa_y += 1,
                '<' => santa_x -= 1,
                'v' => santa_y -= 1,
                _   => ()
            }
            delivered_houses.insert((santa_x,santa_y));
        } else {
            match c {
                '>' => robot_x += 1,
                '^' => robot_y += 1,
                '<' => robot_x -= 1,
                'v' => robot_y -= 1,
                _   => ()
            }
            delivered_houses.insert((robot_x,robot_y));
        }
        even_or_odd += 1;
    }
    return delivered_houses.len();
}