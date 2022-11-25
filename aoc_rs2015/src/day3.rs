pub fn compute(input: &str) {
    let rows: usize = input.chars().filter(|&c| c == '^' || c == 'v').count();
    let cols: usize = input.chars().filter(|&c| c == '<' || c == '>').count();

    let mut world = vec![vec![false; rows]; cols];

    
}