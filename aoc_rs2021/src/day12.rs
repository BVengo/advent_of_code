use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Cave {
    name: String,
    large: bool,
    visited: u32,
    connections: Vec<String>
}

pub fn compute(input: &str) -> (u32, u32) {

    let mut caves = HashMap::<String, Cave>::new(); 
    
    input.lines().for_each(|l| {
        let edge = l.split("-").collect::<Vec<&str>>();

        update_cave(&mut caves, edge[0], edge[1]);
        update_cave(&mut caves, edge[1], edge[0]);
    });


    let p1 = traverse(&mut caves, 1, "start");
    let p2 = traverse(&mut caves, 2, "start");

    (p1, p2)
}

fn traverse(caves: &mut HashMap::<String, Cave>, mut small_limit: u32, root_name: &str) -> u32 {
    let root = caves.get_mut(root_name).unwrap();

    if root.name == "end" {
        return 1;
    }

    if root.name == "start" && root.visited != 0 {
        return 0;
    }
    
    if !root.large && root.visited >= small_limit {
        return 0;
    }

    root.visited += 1;
    
    if small_limit != 1 && root.visited == small_limit {
        small_limit -= 1;
    }

    let connections = root.connections.to_vec();
    drop(root);

    let count: u32 = connections.iter().map(|c| {
        traverse(caves, small_limit, c)
    }).sum();
    
    caves.get_mut(root_name).unwrap().visited -= 1;

    count
}

fn update_cave(caves: &mut HashMap::<String, Cave>, key: &str, end: &str) {
    if !caves.contains_key(key) {
        let root = Cave {
            name: key.to_string(),
            large: key.to_uppercase() == key,
            visited: 0,
            connections: vec![end.to_string()]
        };

        caves.insert(key.to_string(), root);

    } else {
        let root = caves.get_mut(key).unwrap();
        root.connections.push(end.to_string());
    }
}