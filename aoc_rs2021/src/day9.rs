pub fn compute(input: &str) -> (u32, usize) {

    let values = input.lines().map(|l| {
        l.trim().chars().map(|c| {
            c.to_digit(10).unwrap()
        }).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    let basins = get_basins(&values);


    let p1 = part1(&basins);
    let p2 = part2(&values, &basins);

    (p1, p2)
}

struct Pos {
    row: usize,
    col: usize,
    val: u32,
}

fn get_basins(values: &Vec<Vec<u32>>) -> Vec<Pos> {   
    let size = values.len();
    let mut basins = Vec::<Pos>::new();

    for row in 0..size {
        for col in 0..size {
            if check_adjacent(&values, size, row, col) {
                let pos = Pos {
                    row: row,
                    col: col,
                    val: values[row][col]
                };

                basins.push(pos);
            }
        }
    }

    basins
}

fn part1(basins: &Vec<Pos>) -> u32 {
    basins.iter().map(|p| p.val + 1).sum()
}

fn part2(values: &Vec<Vec<u32>>, basins: &Vec<Pos>) -> usize {   
    0
}

fn check_adjacent(values: &Vec<Vec<u32>>, size: usize, row: usize, col: usize) -> bool {
    let num = values[row as usize][col as usize];
    let mut adjacent = vec![10, 10, 10, 10];

    // up
    if col as i32 - 1 >= 0 {
        adjacent[0] = values[row as usize][col as usize - 1];
    }

    // down
    if col + 1 < size {
        adjacent[1] = values[row as usize][col as usize +1];
    }

    // left
    if row as i32 - 1 >= 0 {
        adjacent[2] = values[row as usize - 1][col as usize];
    }

    // right
    if row + 1 < size {
        adjacent[3] = values[row as usize + 1][col as usize];
    }

    adjacent.iter().filter(|&a| a > &num).count() == 4
}