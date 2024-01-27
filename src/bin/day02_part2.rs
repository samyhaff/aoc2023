use aoc2023::utils::read_input;

struct Record {
    red: u32,
    green: u32,
    blue: u32
}

impl Record {
    fn from_str(record: &str) -> Record {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        let record = record.split(",");
        for content in record {
            let content = content.trim();
            let mut content = content.split(" ");
            let amount = content.nth(0).unwrap().parse::<u32>().unwrap();
            let color = content.nth(0).unwrap();
            match color {
                "red" => red = amount,
                "green" => green = amount,
                "blue" => blue = amount,
                _ => panic!("Invalid color")
            }
        }
        Record { red, green, blue }
    }
}

fn main() {
    let input = read_input().unwrap();
    let result = solve(&input);
    println!("{}", result);
}

fn solve(input: &str) -> u32  {
    let lines = input.lines();
    let mut sum = 0;

    for line in lines {
        let mut line = line.split(":");
        let _ = line.nth(0).unwrap().split(" ").nth(1).unwrap().parse::<u32>().unwrap();
        let records = line.next().unwrap().split(";");
        let mut min_content = Record { red: 0, green: 0, blue: 0 };
        for record in records {
            let record = record.trim();
            let record = Record::from_str(record);
            min_content.red = std::cmp::max(min_content.red, record.red);
            min_content.green = std::cmp::max(min_content.green, record.green);
            min_content.blue = std::cmp::max(min_content.blue, record.blue);
        }
        sum += min_content.red * min_content.green * min_content.blue;
    }

    sum
}
