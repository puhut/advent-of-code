use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);
    let mut pair= 0;
    let mut pair_part2=0;

    if let Ok(lines) = read_lines(filename){
        for line in lines {
            if let Ok(pairs) = line {
                let p: Vec<&str> = pairs.split(',').collect();
                let first_pair: Vec<&str> = p[0].split('-').collect();
                let second_pair: Vec<&str> = p[1].split('-').collect();

                if first_pair[0].parse::<i32>().unwrap() <= second_pair[0].parse::<i32>().unwrap() &&  first_pair[1].parse::<i32>().unwrap() >= second_pair[1].parse::<i32>().unwrap(){
                    pair += 1;
                } else if first_pair[0].parse::<i32>().unwrap() >= second_pair[0].parse::<i32>().unwrap() &&  first_pair[1].parse::<i32>().unwrap() <= second_pair[1].parse::<i32>().unwrap(){
                    pair += 1;
                }

                //part II
                let mut checking_range = HashMap::new();
                let mut checking_flag = false;
                for n in first_pair[0].parse::<i32>().unwrap()..first_pair[1].parse::<i32>().unwrap()+1 {
                    checking_range.insert(n, 1);
                }

                for n in second_pair[0].parse::<i32>().unwrap()..second_pair[1].parse::<i32>().unwrap()+1 {
                    match checking_range.get(&n) {
                        Some(num) => { checking_flag = true; }//checking_range.insert(n, num + 1);}
                        None => {checking_range.insert(n, 1);}
                    };
                }

                if checking_flag {
                    pair_part2 += 1;
                }
            }
        }
        println!("first part: {}", pair);
        println!("first part: {}", pair_part2);
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}