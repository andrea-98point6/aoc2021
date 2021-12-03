pub mod day1;

pub mod day1_input;
fn main() {
    println!("day1.0 {:?}", day1::stepper(day1_input::get_input()));
    println!("day1.1 {:?}", day1::stepper_window(day1_input::get_input()));
}
