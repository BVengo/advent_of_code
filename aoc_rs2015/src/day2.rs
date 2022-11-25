pub fn compute(input: &str) {
    let boxes = input.lines().map(|l| {
        l.split("x").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();
   

    let mut surface_area = 0;
    let mut ribbon_area = 0;

    for b in boxes {
        let faces = vec![b[0]*b[1], b[1]*b[2], b[0]*b[2]];

        surface_area += 2 * faces.iter().sum::<u32>() + faces.iter().min().unwrap();

        let mut min1 = b[0];
        let mut min2 = b[1];

        if min2 < min1 {
            min1 = b[1];
            min2 = b[0];
        }

        if b[2] < min1 {
            min2 = min1;
            min1 = b[2];
        } else if b[2] < min2 {
            min2 = b[2];
        }

        ribbon_area += 2 * (min1 + min2) + b[0] * b[1] * b[2];
    }

    println!("{}", surface_area);
    println!("{}", ribbon_area);
}