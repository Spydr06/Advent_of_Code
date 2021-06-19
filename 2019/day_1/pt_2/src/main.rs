use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_fuel(mass: i32) -> i32 {
    let mut fuel: i32 = ((mass / 3) as f32).floor() as i32 - 2;

    if fuel <= 0 {
        return 0;
    }

    fuel += get_fuel(fuel);
    fuel
}

fn main() {
    let file_name = "input.txt";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut total_fuel = 0;

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mass: i32 = line.parse().unwrap();

        total_fuel += get_fuel(mass);
    }

    println!("Total fuel: {}", total_fuel);
}
