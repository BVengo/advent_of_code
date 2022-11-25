pub fn compute(input: &str) -> (usize, usize) {
    let values = input.lines().map(|l| {
        l.parse::<u32>().unwrap()
    }).collect::<Vec<u32>>();

    let p1 = count_increasing(&values, 1);
    let p2 = count_increasing(&values, 3);

    (p1, p2)
}

fn count_increasing(values: &Vec<u32>, roll_amount: usize) -> usize {
    values.windows(roll_amount + 1)
        .filter(|w| {
            w[0] < w[roll_amount]
        }).count()
}