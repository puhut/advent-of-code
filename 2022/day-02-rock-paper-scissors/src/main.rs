use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    /*
        *Opponent
        A - rock - 1
        B - paper - 2
        C - scissor - 3

        *Me
        X - rock 1
        Y - paper 2
        Z - scissor 3

        winner 6
        draw 3
        lost 0
    */
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);
    let mut total_score = 0;

    if let Ok(lines) = read_lines(filename){
        for line in lines {
            if let Ok(number) = line {
                match number.as_ref() {
                    "A X" => { total_score += 4 }, //1+3
                    "A Y" => { total_score += 8 }, //2+6
                    "A Z" => { total_score += 3 }, //3+0
                    "B X" => { total_score += 1 }, //1 + 0
                    "B Y" => { total_score += 5 }, //2 + 3
                    "B Z" => { total_score += 9 }, //3 + 6
                    "C X" => { total_score += 7 }, //1 + 6
                    "C Y" => { total_score += 2 }, //2 + 0
                    "C Z" => { total_score += 6 }, //3 + 3
                    _ => println!("doesn't match")
                };
            }
        }
    }
    println!("{:?}", total_score);
    
    //part II
    /*
        X - lose
        Y - draw
        Z - win

        *Opponent
        A - rock - 1
        B - paper - 2
        C - scissor - 3
     */
    total_score = 0;
    if let Ok(lines) = read_lines(filename){
        for line in lines {
            if let Ok(number) = line {
                match number.as_ref() {
                    "A X" => { total_score += 3 }, //lose scissor 0 + 3
                    "A Y" => { total_score += 4 }, //draw rock 3 + 1
                    "A Z" => { total_score += 8 }, //win paper 6 + 2
                    "B X" => { total_score += 1 }, //lose rock 0 + 1
                    "B Y" => { total_score += 5 }, //draw paper 3 + 2
                    "B Z" => { total_score += 9 }, //win scissor 6 + 3
                    "C X" => { total_score += 2 }, //lose paper 0 + 2
                    "C Y" => { total_score += 6 }, //draw scissor 3 + 3
                    "C Z" => { total_score += 7 }, //win rock 6 + 1
                    _ => println!("doesn't match")
                };
            }
        }
    }
    println!("{:?}", total_score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}