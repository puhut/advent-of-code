use std::fs::read_to_string;
use std::collections::HashSet;
use std::collections::HashMap;

type Point = (usize, usize);

fn main() {
    let input = read_to_string("input.txt").expect("input file no found");
    let now = std::time::Instant::now();
    println!("Part 1 = {}", check_light(&input));
    println!("Part 2 = {}", check_light_part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}

fn check_light(input: &str) -> usize {
    let mut grid = HashSet::<Point>::new();
    for line in input.lines() {
        let mut tokens = line.split(|c| c == ' ' || c== ',');
        let mut keyword = tokens.next().unwrap();
        if keyword == "turn" {
            keyword = tokens.next().unwrap();
        }
        let x_min = tokens.next().unwrap().parse::<usize>().unwrap();
        let y_min = tokens.next().unwrap().parse::<usize>().unwrap();
        tokens.next();
        let x_max = tokens.next().unwrap().parse::<usize>().unwrap();
        let y_max = tokens.next().unwrap().parse::<usize>().unwrap();

        match keyword {
            "on" => {
                for x in x_min..x_max+1 {
                    for y in y_min..y_max+1 {
                        grid.insert((x, y));
                    }
                }
            },
            "off" => {
                for x in x_min..x_max+1 {
                    for y in y_min..y_max+1 {
                        grid.remove(&(x, y));
                    }
                }
            },
            "toggle" => {
                for x in x_min..x_max+1 {
                    for y in y_min..y_max+1 {
                        if grid.contains(&(x, y)) {
                            grid.remove(&(x, y));
                        } else {
                            grid.insert((x, y));
                        }
                    }
                }
            },
            _ => println!("wrong!"),
        }
    }

    return grid.len();
}

fn check_light_part2(input: &str) -> usize {
    let mut grid = HashMap::<Point, usize>::new();
    for line in input.lines() {
        let mut tokens = line.split(|c| c == ' ' || c== ',');
        let mut keyword = tokens.next().unwrap();
        if keyword == "turn" {
            keyword = tokens.next().unwrap();
        }
        let x_min = tokens.next().unwrap().parse::<usize>().unwrap();
        let y_min = tokens.next().unwrap().parse::<usize>().unwrap();
        tokens.next();
        let x_max = tokens.next().unwrap().parse::<usize>().unwrap();
        let y_max = tokens.next().unwrap().parse::<usize>().unwrap();

        match keyword {
            "on" => {
                for x in x_min..x_max+1 {
                    for y in y_min..y_max+1 {
                        *grid.entry((x, y)).or_insert(0) += 1;
                    }
                }
            },
            "off" => {
                for x in x_min..x_max+1 {
                    for y in y_min..y_max+1 {
                        if !grid.contains_key(&(x, y)) {
                            grid.insert((x, y), 0);
                        } else if grid[&(x, y)] > 0 {
                            *grid.get_mut(&(x,y)).unwrap() -= 1;
                        }
                    }
                }
            },
            "toggle" => {
                for x in x_min..x_max+1 {
                    for y in y_min..y_max+1 {
                        *grid.entry((x, y)).or_insert(0) += 2;
                    }
                }
            },
            _ => println!("wrong!"),
        }
    }

    return grid.values().sum();
}