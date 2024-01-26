use aoc2023::utils::read_input;

fn main() {
    let input = read_input().unwrap();
    let sum = solve(input);
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
