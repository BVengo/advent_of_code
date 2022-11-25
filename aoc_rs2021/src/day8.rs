pub fn compute(input: &str) -> (usize, u64) {

    let values = input.lines().map(|l| {
        let inputs = &l[..58];
        let outputs = &l[61..l.len()];

        let inputs = inputs.split_whitespace().map(|i| {String::from(i)}).collect::<Vec<String>>();
        let outputs = outputs.split_whitespace().map(|o| {String::from(o)}).collect::<Vec<String>>();

        (inputs, outputs)
    }).collect::<Vec<(Vec<String>, Vec<String>)>>();

    let p1 = part1(&values);
    let p2 = part2(&values);

    (p1, p2)
}

fn part1(values: &Vec<(Vec<String>, Vec<String>)>) -> usize {   
    values.iter().map(|v| {
        v.1.iter().filter(|o| {
            [2,3,4,7].contains(&o.len())
        }).count()
    }).sum()
}

fn part2(values: &Vec<(Vec<String>, Vec<String>)>) -> u64 {    
    let mut sum = 0;

    for v in values {
        let mut displays: Vec<String> = vec![String::new(); 10];

        while displays.contains(&String::from("")) {
            for i in &v.0 {
                if displays.contains(&i) { 
                    continue;
                }

                let length = i.len();               

                if length == 2 {
                    displays[1] = String::from(i);
                } else if length == 3 {
                    displays[7] = String::from(i);
                } else if length == 4 {
                    displays[4] = String::from(i);
                } else if length == 7 {
                    displays[8] = String::from(i);
                }

                if displays[1] == "" || displays[7] == "" || displays[4] == "" || displays[8] == "" {
                    continue;
                }
                
                if length == 6 && substr_chars(&displays[4], &i) == displays[4].len() {
                    displays[9] = String::from(i);
                } else if length == 5 && substr_chars(&displays[4], &i) == 2 {
                    displays[2] = String::from(i);
                } else if length == 6 && substr_chars(&displays[7], &i) == displays[7].len() {
                    displays[0] = String::from(i);
                } else if length == 6 {
                    displays[6] = String::from(i);
                } else if substr_chars(&i, &displays[6]) == length {
                    displays[5] = String::from(i);
                } else if length == 5 && displays[2] != "" && displays[5] != "" {
                    displays[3] = String::from(i);
                }
            }
        }

        for output in 0..4 {
            for d in 0..10 {
                // Both strings have the same chars
                if v.1[output].len() == displays[d].len() && substr_chars(&v.1[output], &displays[d]) == v.1[output].len() {
                    sum += d as u64 * 10_u64.pow((3-output) as u32);
                }
            }
        }
    }

    sum
}

fn substr_chars(substr: &String, container: &String) -> usize {
    substr.chars().filter(|&c| container.contains(c)).count()
}