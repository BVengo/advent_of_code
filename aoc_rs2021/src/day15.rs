use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Node {
    row: usize,
    col: usize,
    weight: usize
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
            .then_with(|| self.col.cmp(&other.col))
            .then_with(|| self.row.cmp(&other.row))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const SMALL_MAP_SIZE: usize = 100;
const LARGE_MAP_SIZE: usize = 500;

pub fn compute(input: &str) -> (usize, usize) {
    let small_map = input.lines().map(|l| {
        l.trim().chars().map(|c| {
            c.to_digit(10).unwrap() as usize
        }).collect::<Vec<usize>>()
    }).collect::<Vec<Vec<usize>>>();

    let mut large_map = vec![vec![0; LARGE_MAP_SIZE]; LARGE_MAP_SIZE];

    (0..5).for_each(|i| {
        (0..5).for_each(|j| {
            (0..SMALL_MAP_SIZE).for_each(|r| {
                (0..SMALL_MAP_SIZE).for_each(|c| {
                    let large_r = r + i * SMALL_MAP_SIZE;
                    let large_c = c + j * SMALL_MAP_SIZE;
                    let new_val = ((small_map[r][c] + i + j - 1) % 9) + 1;
    
                    large_map[large_r][large_c] = new_val;
                });
            });
        });
    });
    
    let p1 = shortest_risk(&small_map, (0, 0), (SMALL_MAP_SIZE - 1, SMALL_MAP_SIZE - 1)).unwrap();
    let p2 = shortest_risk(&large_map, (0, 0), (LARGE_MAP_SIZE - 1, LARGE_MAP_SIZE - 1)).unwrap();

    (p1, p2)
}

fn shortest_risk(grid: &Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut dist = vec![vec![usize::MAX; grid.len()]; grid.len()];
    
    dist[start.0][start.1] = 0;
    heap.push(Node{row: start.0, col: start.1, weight: 0});

    while let Some(Node{row, col, weight}) = heap.pop() {
        if row == end.0 && col == end.1 {
            return Some(weight);
        }

        if weight > dist[row][col] && (row, col) != start {
            continue;
        }

        let adjacent = get_adjacent(grid, row, col);

        for adj in adjacent {
            let new = Node{row: adj.row, col: adj.col, weight: weight + adj.weight};

            if new.weight < dist[new.row][new.col] {
                heap.push(new);
                dist[new.row][new.col] = new.weight;
            }
        }
    }

    Some(0)
}

fn get_adjacent(grid: &Vec<Vec<usize>>, row: usize, col: usize) -> Vec<Node> {
    let mut adj = Vec::<Node>::new();
    
    if let Some(weight) = grid[row].get(col + 1) {
        adj.push(Node { row: row, col: col + 1, weight: *weight });
    }

    if col > 0 {
        adj.push(Node { row: row, col: col - 1, weight: grid[row][col - 1]});
    }

    if let Some(r) = grid.get(row + 1) {
        adj.push(Node { row: row + 1, col: col, weight: r[col] });
    }

    if row > 0 {
        adj.push(Node { row: row - 1, col: col, weight: grid[row - 1][col]});
    }

    adj
}