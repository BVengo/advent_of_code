#[derive(Debug, PartialEq)]
enum Axis {
    X, Y
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    index: usize
}

pub fn compute(input: &str) -> (usize, Vec<Vec<char>>) {

    let lines = input.lines().collect::<Vec<&str>>();

    let mut folds = Vec::<Fold>::new();
    let mut points = Vec::<(usize, usize)>::new();
    let mut paper_size = (0, 0);
    
    let mut fold_inputs = false;
    for l in lines {
        if l == "" {
            fold_inputs = true;
            continue;
        }

        if fold_inputs {
            let axis: Axis;

            if &l[11..12] == "x" {
                axis = Axis::X;
            } else {
                axis = Axis::Y;
            }

            folds.push( Fold{ 
                axis: axis,
                index: (&l[13..l.len()]).parse::<usize>().unwrap()
            });

        } else {
            let coords = l.split(",").map(|c| { c.parse::<usize>().unwrap() }).collect::<Vec<usize>>();
            points.push((coords[0], coords[1]));

            if paper_size.0 < coords[0] {
                paper_size.0 = coords[0]
            }

            if paper_size.1 < coords[1] {
                paper_size.1 = coords[1];
            }
        }
    }

    fold(&mut points, &folds[0], &mut paper_size);
    let p1 = points.len();

    (1..folds.len()).for_each(|i| {
        fold(&mut points, &folds[i], &mut paper_size) 
    });

    println!("{:?}", points);
    
    let mut grid = vec![vec![' '; paper_size.0 + 1]; paper_size.1 + 1];
    
    points.iter().for_each(|p| {
        grid[p.1][p.0] = '█';
    });

    (p1, grid)
}

fn fold(points: &mut Vec<(usize, usize)>, fold: &Fold, paper_size: &mut (usize, usize)) {
    let curr_points = points.to_vec();

    if fold.axis == Axis::X {
        curr_points.iter().filter(|p| {
            p.0 > fold.index
        }).for_each(|p| {
            points.push((paper_size.0 - p.0, p.1));
        });

        paper_size.0 = fold.index;
    } else {
        curr_points.iter().filter(|p| {
            p.1 > fold.index
        }).for_each(|p| {
            points.push((p.0, paper_size.1 - p.1));
        });

        paper_size.1 = fold.index;
    }

    points.retain(|p| p.0  <= paper_size.0 && p.1 <= paper_size.1);

    points.sort_by(|a, b| { 
        if a.0 != b.0 {
            a.0.partial_cmp(&b.0).unwrap()
        } else {
            a.1.partial_cmp(&b.1).unwrap()
        }
    });

    points.dedup();
}