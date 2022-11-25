pub fn compute(input: &str) -> (u32, u32) {
    let binary = input.lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    let p1 = power_consumption(&binary);
    let p2 = air_rating(&binary, false) * air_rating(&binary, true);

    (p1, p2)
}

fn power_consumption(vals: &[u32]) -> u32 {
    let gamma_val = (0..12).rev()
        .map(|p| {
            max_bit(vals, p) << p
        }).sum::<u32>();

    let epsilon_val = !gamma_val & 0xfff;

    gamma_val * epsilon_val
}

fn air_rating(vals: &Vec<u32>, is_oxy: bool) -> u32 {
    let mut local_vals = vals.to_vec();

    for i in 0..12 {
        if local_vals.len() < 2 {
            break;
        }

        let count = local_vals
            .iter()
            .filter(|v| **v >> (11 - i) & 0x01 == 0x01)
            .count();

        let common = (count >= local_vals.len() - count) as u32;
        let needle = if is_oxy { 1 - common } else { common };
        local_vals.retain(|v| *v >> (12 - 1 - i) & 0x01 == needle);
    }

    local_vals[0] as u32
}


fn max_bit(vals: &[u32], pos: usize) -> u32 {
    let (ones, zeroes): (Vec<u32>, Vec<u32>) = vals.iter().partition(|&v| (v & 1 << pos) > 0);
    (ones.len() > zeroes.len()) as u32
}