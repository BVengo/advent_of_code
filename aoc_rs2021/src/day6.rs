pub fn compute(input: &str) -> (u64, u64) {

    let mut values: [u64; 9] = [0; 9];
    input.trim().split(",").for_each(|f| { values[f.parse::<usize>().unwrap()] += 1; });

    let p1 = get_day(&values, 80);
    let p2 = get_day(&values, 256);

    (p1, p2)
}

fn get_day(values: &[u64], days: u32) -> u64 {

    let mut vals = values.to_vec();

    for _ in 0..days {
        let temp = vals[0];
        
        for life in 1..vals.len() {
            vals[life-1] = vals[life];
        }

        vals[8] = temp;
        vals[6] += temp;
    }

    vals.iter().fold(0, |a, b| a + b)
}