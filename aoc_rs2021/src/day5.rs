pub fn compute(input: &str) -> (usize, usize) {
    let values = input.lines().map(|l| {
        l.split(" -> ").map(|s| {
            s.split(",").map(|n| {
                n.parse::<usize>().unwrap()
            }).collect::<Vec<usize>>()
        }).collect::<Vec<Vec<usize>>>()
    }).collect::<Vec<Vec<Vec<usize>>>>();

    
    (part1(&values), part2(&values))
}

fn part1(values: &Vec<Vec<Vec<usize>>>) -> usize {
    let mut lines = [[0usize; 1000]; 1000];

    for v in values {
        let mut x1 = v[0][0];
        let mut y1 = v[0][1];
        let mut x2 = v[1][0];
        let mut y2 = v[1][1];

        if x1 == x2 {
            if y1 > y2 { 
                let temp = y1;
                y1 = y2;
                y2 = temp;
            }

            for y_pos in y1..=y2 {
                lines[x1][y_pos] += 1;
            }

        } else if y1 == y2 {
            if x1 > x2 { 
                let temp = x1;
                x1 = x2;
                x2 = temp;
            }

            for x_pos in x1..=x2 {
                lines[x_pos][y1] += 1;
            }
        }
    }

    let mut count = 0;
    for element in lines.iter_mut().flat_map(|r| r.iter_mut()) {
        if *element > 1 {
            count += 1;
        }
    }

    count
}

fn part2(values: &Vec<Vec<Vec<usize>>>) -> usize {
    let mut lines = [[0usize; 1000]; 1000];

    for v in values {
        let mut x1 = v[0][0];
        let mut y1 = v[0][1];
        let mut x2 = v[1][0];
        let mut y2 = v[1][1];

        let x_dir: usize = (x1 as isize - x2 as isize).abs() as usize;
        let y_dir: usize = (y1 as isize - y2 as isize).abs() as usize;

        if x_dir == 0 {
            // vert
            if y1 > y2 { 
                let temp = y1;
                y1 = y2;
                y2 = temp;
            }

            for y_pos in y1..=y2 {
                lines[x1][y_pos] += 1;
            }
        } else if y_dir == 0 {
            // horiz
            if x1 > x2 { 
                let temp = x1;
                x1 = x2;
                x2 = temp;
            }

            for x_pos in x1..=x2 {
                lines[x_pos][y1] += 1;
            }
        } else if x_dir == y_dir {           
            let dist = x_dir;

            if x1 > x2 && y1 > y2 {
                let temp = x1;
                x1 = x2;
                x2 = temp;

                let temp = y1;
                y1 = y2;
                y2 = temp;
            } 
            
            if x2 > x1 && y2 > y1 {
                for pos in 0..=dist {
                    lines[x1 + pos][y1 + pos] += 1;
                }       
            } else {
                if x2 > x1 {
                    for pos in 0..=dist {
                        lines[x1 + pos][y1 - pos] += 1
                    }
                } else {
                    for pos in 0..=dist {
                        lines[x1 - pos][y1 + pos] += 1
                    }
                }
            }
        }
    }

    let mut count = 0;
    for element in lines.iter_mut().flat_map(|r| r.iter_mut()) {
        if *element > 1 {
            count += 1;
        }
    }

    count
}