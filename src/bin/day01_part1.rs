use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide a filename");
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let sum = solve(contents);
    println!("Sum: {}", sum);
}

fn solve(contents: String) -> u32 {
    let mut sum = 0;
    for line in contents.lines() {
        let first = line.chars().find(|c| c.is_digit(10)).unwrap();
        let firdt: u32 = first.to_digit(10).unwrap();
        let last = line.chars().rev().find(|c| c.is_digit(10)).unwrap();
        let last: u32 = last.to_digit(10).unwrap();
        sum += 10 * firdt + last;
    }
    sum
}
