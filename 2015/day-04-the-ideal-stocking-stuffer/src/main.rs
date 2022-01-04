extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let now = std::time::Instant::now();
    let input = "ckczppom".as_bytes();
    let mut count: u64 = 1;
    let mut hasher = Md5::new();

    loop {
        hasher.input(input);
        hasher.input(count.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        let first_five = output[0] as i32 + output[1] as i32 + (output[2]>>4) as i32;
        if first_five == 0 {
            println!("Part 1 = {}", count);
            hasher.reset();
            break;
        }
        hasher.reset();
        count += 1;
    }

    count = 1;
    loop {
        hasher.input(input);
        hasher.input(count.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        let first_six = output[0] as i32 + output[1] as i32 + output[2] as i32;
        if first_six == 0 {
            println!("Part 2 = {}", count);
            break;
        }
        hasher.reset();
        count += 1;
    }
    println!("Execution time: {:?}", now.elapsed());
}
