pub fn compute(input: &str) -> (u32, u32) {
    let values = input.lines().map(|l| {
        let val_vector = l.split_whitespace().collect::<Vec<&str>>();
        
        (val_vector[0], val_vector[1].parse::<u32>().unwrap())

    }).collect::<Vec<(&str, u32)>>();

    let p1 = find_value(&values, false);
    let p2 = find_value(&values, true);

    (p1, p2)
}

fn find_value(vals: &Vec<(&str, u32)>, use_aim: bool) -> u32 {
    let mut aim = 0;
    let mut h_dist = 0;
    let mut v_dist = 0;

    for v in vals {
        if use_aim {
            match v.0 {
                "forward" => {
                    h_dist += v.1;
                    v_dist += aim * v.1;
                },
                "down"      => { aim += v.1; },
                "up"        => { aim -= v.1; },
                _ => {}
            }
        } else {
            match v.0 {
                "forward"   => { h_dist += v.1; },
                "down"      => { v_dist += v.1; },
                "up"        => { v_dist -= v.1; },
                _ => {}
            }
        }
    }

    h_dist * v_dist
}