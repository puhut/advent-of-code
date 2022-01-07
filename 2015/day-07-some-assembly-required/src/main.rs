use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let input = read_to_string("input.txt").expect("input file no found");
    let now = std::time::Instant::now();
    println!("Part 1 = {}", process_circuit(&input));
    println!("Execution time: {:?}", now.elapsed());
}

fn process_circuit(input: &str) -> usize {
    let mut signals = HashMap::<&str, usize>::new();
    let mut circuit = HashMap::<&str, Vec<&str>>::new();

    for line in input.lines() {
        let instruction: Vec<&str> = line.split(" ").collect();

        match instruction.len() {
            3 => { //a -> b
                circuit.insert(instruction[2], vec![instruction[0]]);
            },
            4 => { // not a -> b
                circuit.insert(instruction[3], vec![instruction[0], instruction[1]]);
            },
            5 => { // a & b -> c or a || b -> c
                circuit.insert(instruction[4], vec![instruction[1], instruction[0], instruction[2]]);
            },
            _ => {
                println!("something wrong!");
            }
        }
        println!("{:?}", circuit);
    }
    return 0;
}
