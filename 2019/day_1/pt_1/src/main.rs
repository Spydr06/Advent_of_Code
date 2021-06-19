use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_name = "input.txt";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut total_fuel = 0;

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mass: i32 = line.parse().unwrap();
        let fuel = ((mass / 3) as f32).floor() as i32 - 2;

        total_fuel += fuel 
    }

    println!("Total fuel: {}", total_fuel);
}
