use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;
use day5::Board;
pub mod day1;
pub mod day5;
// pub mod day1_input;

pub mod day2;
// pub mod day2_input;

pub mod day3;

pub mod day4;

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

    let input4 = read_lines_4_into("files/day4.txt".to_string());
    // println!("{:?}", input4.len());

    let called = vec!{
        31,50,79,59,39,53,58,95,92,55,40,97,81,22,69,26,6,23,3,29,83,48,18,75,47,49,62,45,35,34,1,88,54,16,56,77,28,94,52,15,0,87,93,90,60,67,68,85,80,51,20,96,61,66,63,91,8,99,70,13,71,17,7,38,44,43,5,25,72,2,57,33,82,78,89,21,30,11,73,84,4,46,14,19,12,10,42,32,64,98,9,74,86,27,24,65,37,41,76,36
    };
    let mut win_result = -1;
    let mut win_moves = 700;
    
    let mut loose_result = -1;
    let mut loose_moves = 1;

    for ch in &input4.iter().chunks(25) {
        if let Some((moves_t, result_t)) = day4::find_bingo(ch.map(|n| *n).collect_vec(), called.clone()){
            if moves_t <= win_moves {
                win_moves = moves_t;
                win_result = result_t;
            }
            if moves_t > loose_moves {
                loose_moves = moves_t;
                loose_result = result_t;
            }
        }
    }
    println!("day4 winner {:?}", win_result);
    println!("day4 looser {:?}", loose_result);

    // this main is starting to be pretty sad, but it is Xmas
    let input_5 = read_lines_into("./files/day5.txt".to_string(), |line| {
        let dis = line.split(",").map(|s| s.parse().unwrap_or(0)).collect::<Vec<i32>>();
        (dis[0], dis[1], dis[2], dis[3])
    });

    let mut board = Board::new();
    input_5.clone().into_iter().for_each(|dis| {
        board.mark_this_route(dis.0, dis.1, dis.2, dis.3)
    });

    println!("day5 {}", board.count());

    let mut board = Board::new();
    input_5.clone().into_iter().for_each(|dis| {
        board.mark_this_route_diag(dis.0, dis.1, dis.2, dis.3)
    });

    println!("day5 {}", board.count());

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

fn read_lines_4_into(name_file: String) -> Vec<i32> {
    let mut accumulator: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(name_file) {
        for line in lines {
            if let Ok(numbers) = line {
                numbers
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .for_each(|n| accumulator.push(n))
            }
        }
    };
    accumulator
}
