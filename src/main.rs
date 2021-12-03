pub mod day1;
pub mod day1_input;

pub mod day2;
pub mod day2_input;

fn main() {
    println!("day1.0 {:?}", day1::stepper(day1_input::get_input()));
    println!("day1.1 {:?}", day1::stepper_window(day1_input::get_input()));

    println!("day2.0 {:?}", day2::move_sub(day2_input::get_input()));
    println!("day2.0 {:?}", day2::move_sub_better(day2_input::get_input()));
}
