use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    /*
        Lowercase item types a through z have priorities 1 through 26.
        Uppercase item types A through Z have priorities 27 through 52.

        ASCII A = 65
        ASCII Z = 90
        ASCII a = 97
        ASCII z = 122
    */

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);
    let mut result = 0;

    if let Ok(lines) = read_lines(filename){
        for line in lines {
            if let Ok(items) = line {

                let (first, second) = items.split_at(items.len()/2);
                let mut first_vec = first.chars().collect::<Vec<char>>();
                let mut second_vec = second.chars().collect::<Vec<char>>();
                first_vec.sort();
                first_vec.dedup();
                second_vec.sort();
                second_vec.dedup();
                //println!("{:?} ==== {:?}", first_vec, second_vec);

                let mut checking_items = HashMap::new();
                for item in first_vec {
                    checking_items.insert(item, 1);
                }

                for item in second_vec {
                    match checking_items.get(&item) {
                        Some(num) => {checking_items.insert(item, num + 1);}
                        None => ()
                    };
                }

                for (k,v) in checking_items {
                    if v == 2 {                        
                        if (k as u32 )<91 {
                            result += (k as u32)-65+27;
                        } else {
                            result += (k as u32)-97+1;
                        }
                    }
                }
            }
        }
        println!("{}", result);
    }

    result = 0;
    if let Ok(lines) = read_lines(filename){
        let mut line_counter=0;
        let mut checking_items = HashMap::new();
        for line in lines {
            if let Ok(items) = line {
                let mut items_vec = items.chars().collect::<Vec<char>>();
                items_vec.sort();
                items_vec.dedup();
                //println!("{:?} ==== {:?}", first_vec, second_vec);

                if line_counter%3 == 0{
                    for item in items_vec {
                        checking_items.insert(item, 1);
                    }
                } else {
                    for item in items_vec {
                        match checking_items.get(&item) {
                            Some(num) => {checking_items.insert(item, num + 1);}
                            None => ()
                        };
                    }
                }
                
                for (k,v) in &checking_items {
                    if *v == 3 {                        
                        if (*k as u32 )<91 {
                            result += (*k as u32)-65+27;
                        } else {
                            result += (*k as u32)-97+1;
                        }
                    }
                }

                if line_counter%3 == 2 {
                    checking_items.clear();
                }
                line_counter += 1;
            }
        }
        println!("{}", result);
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}