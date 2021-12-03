use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
pub mod day1;
// pub mod day1_input;

pub mod day2;
// pub mod day2_input;

pub mod day3;

fn main() {
    // println!("day1.0 {:?}", day1::stepper(day1_input::get_input()));
    // println!("day1.1 {:?}", day1::stepper_window(day1_input::get_input()));

    // println!("day2.0 {:?}", day2::move_sub(day2_input::get_input()));
    // println!(
    //     "day2.0 {:?}",
    //     day2::move_sub_better(day2_input::get_input())
    // );

    let input1: Vec<i64> = read_lines_into("./files/day1.txt".to_string(), |number| {
        number
            .split_whitespace()
            .collect::<String>()
            .parse()
            .unwrap_or(0)
    });
    println!("day1.0 {:?}", day1::stepper(input1.clone()));
    println!("day1.1 {:?}", day1::stepper_window(input1));

    let input2: Vec<(String, i32)> = read_lines_into("./files/day2.txt".to_string(), |line| {
        let mut parts = line.split_whitespace();
        (
            parts.next().unwrap_or("none").to_string(),
            parts.next().unwrap_or("0").parse().unwrap_or(0),
        )
    });

    println!("day2.0 {:?}", day2::move_sub(input2.clone()));
    println!("day2.1 {:?}", day2::move_sub_better(input2));

    let input3: Vec<String> = read_lines_into("./files/day3.txt".to_string(), |number| number);

    println!("day3.0 {:?}", day3::get_consumption(input3.clone()));
    println!("day3. {:?}", day3::get_live(input3.clone()));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_lines_into<T>(name_file: String, converter: fn(String) -> T) -> Vec<T> {
    let mut accumulator: Vec<T> = Vec::new();
    if let Ok(lines) = read_lines(name_file) {
        for line in lines {
            if let Ok(number) = line {
                accumulator.push(converter(number));
            }
        }
    };
    accumulator
}
