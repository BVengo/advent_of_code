pub fn compute(input: &str) -> (u32, u32) {
    let values = input.lines()
        .filter(|&l| l != "")
        .map(|l| {
            String::from(l)
        }).collect::<Vec<String>>();

    let input = String::from(&values[0]).split(",").map(|n| {
        n.parse::<u32>().unwrap()
    }).collect::<Vec<u32>>();
    
    let mut boards = values;
    boards.remove(0);

    let boards = boards.iter()
        .map(|l| {
            l.split_whitespace().map(|n| {
                n.parse::<u32>().unwrap()
            }).collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>();
    
    let mut scores:Vec<(usize, u32)> = Vec::new();

    for drawn in 0..(input.len()) {
        for board in (0..boards.len()).step_by(5) {
            let completed_boards = scores.iter().map(|s| { s.0 }).collect::<Vec<usize>>();
            if completed_boards.contains(&board) {
                continue;
            }

            if check_rows(&boards[board..board+5], &input[0..=drawn]) || 
                check_cols(&boards[board..board+5], &input[0..=drawn]) {

                let score = sum_board(&boards[board..board+5], &input[0..=drawn]) * input[drawn];
                scores.push((board, score));
            }
        }
    }

    let p1 = scores[0].1;
    let p2 = scores[scores.len() - 1].1;

    (p1, p2)
}

fn check_rows(board: &[Vec<u32>], called: &[u32]) -> bool {
    (0..5).filter(|&r| {
        (0..5).filter(|&i| {
            called.contains(&board[r][i])
        }).collect::<Vec<usize>>().len() == 5
    }).count() > 0
}

fn check_cols(board: &[Vec<u32>], called: &[u32]) -> bool {
    (0..5).filter(|&c| {
        (0..5).filter(|&i| {
            called.contains(&board[i][c])
        }).collect::<Vec<usize>>().len() == 5
    }).count() > 0
}

fn sum_board(board: &[Vec<u32>], called: &[u32]) -> u32 {
    board.iter().map(|r| {
        r.iter().filter(|n| !called.contains(n)).sum::<u32>()
    }).sum::<u32>()
}