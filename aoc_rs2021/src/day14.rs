use std::collections::HashMap;

pub fn compute(input: &str) -> (u64, u64) {
    let lines = input.lines().collect::<Vec<&str>>();
    
    let mut chars = HashMap::<char, u64>::new();
    lines[0].trim().chars().for_each(|c| {
        *chars.entry(c).or_insert(0) += 1;
    });

    let mut pairs = extract_pairs(&lines);
    let mut conditions = extract_conditions(&lines);
    

    (0..10).for_each(|_i| {
        step(&mut pairs, &mut conditions, &mut chars);
    });

    let char_counts = chars.values().cloned().collect::<Vec<u64>>();
    let p1 = char_counts.iter().max().unwrap() - char_counts.iter().min().unwrap();

    (10..40).for_each(|_i| {
        step(&mut pairs, &mut conditions, &mut chars);
    });

    let char_counts = chars.values().cloned().collect::<Vec<u64>>();
    let p2 = char_counts.iter().max().unwrap() - char_counts.iter().min().unwrap();

    (p1, p2)
}

fn extract_pairs(lines: &Vec<&str>) -> HashMap<String, u64> {
    let mut pairs = HashMap::<String, u64>::new();

    lines[0].trim().chars().collect::<Vec<char>>()
        .windows(2).for_each(|w| {
            let mut pair_value = String::from(w[0]);
            pair_value.push(w[1]);

            *pairs.entry(pair_value).or_insert(0) += 1;
        });

    pairs
}

fn extract_conditions(lines: &Vec<&str>) -> HashMap<String, char> {
    let mut conditions = HashMap::<String, char>::new();

    lines[2..lines.len()].iter().for_each(|l| {
        let pair_val = l.split(" -> ").collect::<Vec<&str>>();
        conditions.insert(String::from(pair_val[0]), pair_val[1].chars().next().expect("Character insert missing!"));
    });

    conditions
}

fn step(pairs: &mut HashMap<String, u64>, conditions: &mut HashMap<String, char>, chars: &mut HashMap<char, u64>) {
    let current_pairs = pairs.clone();

    current_pairs.iter().for_each(|(k, v)| {
        if let Some(c) = conditions.get(k) {
            let mut pair_a = String::from(&k[0..1]);
            let mut pair_b = String::from(*c);

            pair_a.push(*c);
            pair_b.push_str(&k[1..2]);

            *pairs.entry(pair_a).or_insert(0) += v;
            *pairs.entry(pair_b).or_insert(0) += v;
            *pairs.entry(k.clone()).or_insert(0) -= v;

            *chars.entry(*c).or_insert(0) += v;
        }
    });
}