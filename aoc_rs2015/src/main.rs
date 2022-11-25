use std::fs;

pub mod day3;

fn main() {
    let input = get_input(3);
    day3::compute(&input);
}

fn get_input(num: u32) -> String {
    fs::read_to_string(format!("input/day{}.txt", &num.to_string())).expect("Something went wrong reading the file")
}