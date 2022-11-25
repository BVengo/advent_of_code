pub fn compute(input: &str) -> (i64, i64) {

    let mut values = input.trim().split(",").map(|v| { v.parse::<i64>().unwrap() }).collect::<Vec<i64>>();
    values.sort();

    let p1 = part1(&values);
    let p2 = part2(&values);

    (p1, p2)
}

fn part1(values: &Vec<i64>) -> i64 {
    let vals = values.to_vec();
    let median = values[values.len()/2];

    vals.iter().map(|&v| {
        (median - v).abs()
    }).sum()
}

fn part2(values: &Vec<i64>) -> i64 {
    let vals = values.to_vec();
    let mean = (vals.iter().sum::<i64>() as f64 / vals.len() as f64);

    let lower_mean = mean.floor();
    let upper_mean = mean.ceil();

    let sum1 = vals.iter().map(|&v| {
        let moves = (lower_mean - v as f64).abs() as f64;
        ((moves / 2.0) * (moves + 1.0)) as i64
    }).sum();

    let sum2 = vals.iter().map(|&v| {
        let moves = (upper_mean - v as f64).abs() as f64;
        ((moves / 2.0) * (moves + 1.0)) as i64
    }).sum();

    if sum1 > sum2 {
        sum2
    } else {
        sum1
    }
}