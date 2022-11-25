const GRID_SIZE: usize = 10;

pub fn compute(input: &str) -> (u32, usize) {

    let grid = input.lines().map(|l| {
        l.chars().map(|c| {
            c.to_digit(10).unwrap()
        }).collect()
    }).collect::<Vec<Vec<u32>>>();

    let mut grid_p1 = grid.to_vec();
    let mut grid_p2 = grid.to_vec();

    let p1 = (0..GRID_SIZE * GRID_SIZE).map(|_i| flash_grid(&mut grid_p1).0).sum();
    let mut p2 = 1;

    while !flash_grid(&mut grid_p2).1 {
        p2 += 1;
    }
    
    (p1, p2)
}

fn flash_grid(grid: &mut Vec<Vec<u32>>) -> (u32, bool) {
    let mut num_flashed = 0;

    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            num_flashed += flash_octopus(grid, row as isize, col as isize);
        }
    }

    for flashed_octopus in grid.iter_mut().flat_map(|r| r.iter_mut().filter(|o| o > &&mut 9)) {
        *flashed_octopus = 0;
    }

    (num_flashed, num_flashed as usize == (GRID_SIZE * GRID_SIZE))
}

fn flash_octopus(grid: &mut Vec<Vec<u32>>, row: isize, col: isize) -> u32 {

    if row < 0 || row >= GRID_SIZE as isize || col < 0 || col >= GRID_SIZE as isize {
        return 0;
    }

    grid[row as usize][col as usize] += 1;

    if grid[row as usize][col as usize]  != 10 {
        return 0;
    }

    flash_octopus(grid, row - 1, col) +
        flash_octopus(grid, row - 1, col - 1) +
        flash_octopus(grid, row - 1, col + 1) +
        flash_octopus(grid, row + 1, col) +
        flash_octopus(grid, row + 1, col - 1) +
        flash_octopus(grid, row + 1, col + 1) +
        flash_octopus(grid, row, col - 1) +
        flash_octopus(grid, row, col + 1)
}