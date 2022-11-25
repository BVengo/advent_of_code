pub fn compute(input: &str) -> (u32, u64) {

    let lines = input.lines().collect::<Vec<&str>>();

    let mut closing_points = Vec::<u64>::new();
    let mut corrupted_points: u32 = 0;

    'line_loop: for l in lines.iter() {
        let mut stack = Vec::<char>::new();

        let mut line_points: u64 = 0;
        let prev_points = corrupted_points;

        for c in l.chars() {
            corrupted_points +=  match c {
                '(' | '[' | '{' | '<' => { 
                    stack.push(c); 
                    0
                },
                ')' => check_escape(&mut stack, c, 3      ),
                ']' => check_escape(&mut stack, c, 57     ),
                '}' => check_escape(&mut stack, c, 1197   ),
                '>' => check_escape(&mut stack, c, 25137  ),
                _ => {0}
            };

            if prev_points != corrupted_points {
                continue 'line_loop;
            }
        }

        while let Some(entry) = stack.pop() {
            line_points = (line_points * 5) + match entry {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _   => 0
            };
        }

        closing_points.push(line_points);
    }
    
    closing_points.sort();

    let p1 = corrupted_points;
    let p2 = closing_points[closing_points.len()/2];

    (p1, p2)
}

fn check_escape(stack: &mut Vec<char>, escape: char, error_points: u32) -> u32 {
    let enter: char;

    match stack.pop() {
        Some(c) => { enter = c; },
        None => return error_points
    }

    let correct_escape = match escape {
        ')' => enter == '(',
        ']' => enter == '[',
        '}' => enter == '{',
        '>' => enter == '<',
        _ => {false}
    };

    if correct_escape {
        0
    } else {
        error_points
    }
}