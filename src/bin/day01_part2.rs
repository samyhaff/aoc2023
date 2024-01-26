use aoc2023::utils::read_input;
use std::collections::HashMap;

fn main() {
    let input = read_input().unwrap();
    let sum = solve(input);
    println!("Sum: {}", sum);
}

fn solve(contents: String) -> u32 {
    let nums_dict = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut sum = 0;
    for line in contents.lines() {
        let mut forward = line;
        let mut backward = line;

        let first_num = 'fowrd_loop: loop {
            for (key, value) in &nums_dict {
                if forward.starts_with(key) {
                    break 'fowrd_loop value;
                }
            }
            forward = &forward[1..];
        };

        let last_num = 'back_loop: loop {
            for (key, value) in &nums_dict {
                if backward.ends_with(key) {
                    break 'back_loop value;
                }
            }
            backward = &backward[..backward.len() - 1];
        };

        sum += 10 * first_num + last_num;
    }
    sum
}
