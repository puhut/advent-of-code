use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);
    let mut elf_number = 0;
    let mut calories_by_elf_number = HashMap::new();

    if let Ok(lines) = read_lines(filename){
        for line in lines {
            if let Ok(number) = line {
                if number.is_empty() {
                    elf_number += 1;
                } else {
                    match calories_by_elf_number.get(&elf_number) {
                        Some(calories) => { calories_by_elf_number.insert(elf_number, calories +  i32::from_str(&number).unwrap())}
                        None => calories_by_elf_number.insert(elf_number, i32::from_str(&number).unwrap())
                    };
                }
            }
        }
    }
    println!("{:?}", calories_by_elf_number);
    let key_with_max_value = calories_by_elf_number.iter().max_by_key(|entry| entry.1).unwrap();
    println!(" max value key: {} value: {}", key_with_max_value.0, key_with_max_value.1);
    
    //part II
    let mut calories_vec: Vec<(&i32, &i32)> = calories_by_elf_number.iter().collect();
    calories_vec.sort_by(|a,b| b.1.cmp(a.1));
    println!("{:?}", calories_vec[0].1 + calories_vec[1].1 + calories_vec[2].1);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}