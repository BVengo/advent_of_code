use std::fs;

pub mod day16;

fn main() {
    let input  = get_input(16);
    let output: (_, _) = day16::compute(&input);

    println!("Part1: {:?}", output.0);
    println!("Part2: {:?}", output.1);
}

fn get_input(num: u32) -> String {
    let file = format!("input/day{}.txt", num);
    fs::read_to_string(file).expect("Something went wrong reading the file")
}