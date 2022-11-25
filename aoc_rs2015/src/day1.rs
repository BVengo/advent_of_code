pub fn compute(input: &str) {
    let mut floor = 0;
    
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => ()
        }

        if floor == -1 {
            println!("Position {} sent Santa to the basement", i + 1);
            break;
        }
    }

    println!("Santa has finished on floor {}", floor);
}