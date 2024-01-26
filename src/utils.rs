use std::env;
use std::fs;

pub fn read_input() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let filename = &args[1];
            let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
            Some(contents)
        },
        _ => None
    }
}
