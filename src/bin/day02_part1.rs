use aoc2023::utils::read_input;

struct Record {
    red: u32,
    green: u32,
    blue: u32
}

impl Record {
    fn is_possible(&self, bag_content: &Record) -> bool {
        self.red <= bag_content.red &&
        self.green <= bag_content.green &&
        self.blue <= bag_content.blue
    }

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
    let bag_content = Record { red: 12, green: 13, blue: 14 };
    let result = solve(&input, &bag_content);
    println!("{}", result);
}

fn solve(input: &str, bag_content: &Record) -> u32  {
    let lines = input.lines();
    let mut sum = 0;

    'outer: for line in lines {
        let mut line = line.split(":");
        let game_id = line.nth(0).unwrap().split(" ").nth(1).unwrap().parse::<u32>().unwrap();
        let records = line.next().unwrap().split(";");
        for record in records {
            let record = record.trim();
            let record = Record::from_str(record);
            if !record.is_possible(bag_content) {
                continue 'outer;
            }
        }
        sum += game_id;
    }

    sum
}
