use itertools::Itertools;
use petgraph::{Graph, algo, graph::NodeIndex, data::Build};

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
    value: u8,
}

impl Point {
    fn new(x: usize, y: usize, value: u8) -> Self {
        Point { x: x, y: y, value: value }
    }
}

fn get_neighbours<T>(x: usize, y: usize, vecs: &Vec<Vec<T>>, values: &Vec<Vec<char>>) -> (Vec<T>, Vec<u8>)
where
    T: Copy 
{
    let mut neighbours = Vec::new();
    let mut nb_values = Vec::new();

    let y_len = vecs.len();
    let x_len = vecs[0].len();

    if x > 0 {
        neighbours.push(vecs[y][x-1]);
        nb_values.push(values[y][x-1] as u8);
    }
    if x < x_len - 1 {
        neighbours.push(vecs[y][x+1]);
        nb_values.push(values[y][x+1] as u8);
    }
    if y > 0 {
        neighbours.push(vecs[y-1][x]);
        nb_values.push(values[y-1][x] as u8);
    }
    if y < y_len - 1 {
        neighbours.push(vecs[y+1][x]);
        nb_values.push(values[y+1][x] as u8);
    }

    (neighbours, nb_values)
}

pub fn part1(input: String) {
    let elevation_data = input
        .trim()
        .split('\n')
        .map(|x| x  
            .trim() 
            .chars()
            .collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();
        
    let mut elevation_data2 = input
    .trim()
    .split('\n')
    .map(|x| x  
        .trim() 
        .chars()
        .collect::<Vec<char>>()
    )
    .collect::<Vec<Vec<char>>>();

    let mut graph: Graph<Point, i32> = Graph::new();

    let y_len = elevation_data.len();
    let x_len = elevation_data[0].len();

    let nodes = elevation_data
        .into_iter()
        .enumerate()
        .map(|(y, outer)| {
            outer
                .into_iter()
                .enumerate()
                .map(|(x, inner)| {
                    let point = Point::new(x, y, inner as u8);
                    graph.add_node(point)
                })
                .collect_vec()
        })
        .collect_vec();

    let mut start = NodeIndex::new(0);
    let mut end = NodeIndex::new(0);

    // Find start and end, and change them to a and z
    for y in 0..y_len {
        for x in 0..x_len {
            let node = nodes[y][x];
            let mut value = elevation_data2[y][x] as u8;
            
            if value == 83 { // S
                start = node;
                value = 97; // a
                elevation_data2[y][x] = 97 as char;
                // println!("Found start at ({:?}, {:?})", x, y);
            }
            if value == 69 {  // E
                end = node;
                value = 122; // z
                elevation_data2[y][x] = 122 as char;
                // println!("Found end at ({:?}, {:?})", x, y);
            }
        }
    }

    for y in 0..y_len {
        for x in 0..x_len {
            let node = nodes[y][x];
            let mut value = elevation_data2[y][x] as u8;

            let (neighbours, nb_values) = get_neighbours(x, y, &nodes, &elevation_data2);

            for (nb, nb_value) in neighbours.into_iter().zip(nb_values.into_iter()) {
                // Can only go one up at max
                if value + 1 >= nb_value {
                    // println!("Adding edge from ({:?}, {:?}) {:?} to {:?}", x, y, value as char, nb_value as char);
                    graph.add_edge(node, nb, 1);
                }
            }

        }
    }

    let shortest_path = algo::dijkstra(&graph, start, Some(end), |_| 1); 
    let answer = shortest_path.get(&end).unwrap();

    println!("Part 1: {:?}", answer);
}

pub fn part2(input: String) {
    let elevation_data = input
        .trim()
        .split('\n')
        .map(|x| x  
            .trim() 
            .chars()
            .collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();
        
    let mut elevation_data2 = input
    .trim()
    .split('\n')
    .map(|x| x  
        .trim() 
        .chars()
        .collect::<Vec<char>>()
    )
    .collect::<Vec<Vec<char>>>();

    let mut graph: Graph<Point, i32> = Graph::new();

    let y_len = elevation_data.len();
    let x_len = elevation_data[0].len();

    let nodes = elevation_data
        .into_iter()
        .enumerate()
        .map(|(y, outer)| {
            outer
                .into_iter()
                .enumerate()
                .map(|(x, inner)| {
                    let point = Point::new(x, y, inner as u8);
                    graph.add_node(point)
                })
                .collect_vec()
        })
        .collect_vec();

    let mut start = NodeIndex::new(0);
    let mut end = NodeIndex::new(0);

    // Find start and end, and change them to a and z
    for y in 0..y_len {
        for x in 0..x_len {
            let node = nodes[y][x];
            let mut value = elevation_data2[y][x] as u8;
            
            if value == 83 { // S
                start = node;
                value = 97; // a
                elevation_data2[y][x] = 97 as char;
                // println!("Found start at ({:?}, {:?})", x, y);
            }
            if value == 69 {  // E
                end = node;
                value = 122; // z
                elevation_data2[y][x] = 122 as char;
                // println!("Found end at ({:?}, {:?})", x, y);
            }
        }
    }

    let mut start_nodes = Vec::new();

    for y in 0..y_len {
        for x in 0..x_len {
            let node = nodes[y][x];
            let mut value = elevation_data2[y][x] as u8;

            if value == 97 {
                start_nodes.push(node);
            }

            let (neighbours, nb_values) = get_neighbours(x, y, &nodes, &elevation_data2);

            for (nb, nb_value) in neighbours.into_iter().zip(nb_values.into_iter()) {
                // Can only go one up at max
                if value + 1 >= nb_value {
                    // println!("Adding edge from ({:?}, {:?}) {:?} to {:?}", x, y, value as char, nb_value as char);
                    graph.add_edge(node, nb, 1);
                }
            }

        }
    }

    let mut paths = Vec::new();
    for some_start in start_nodes {
        let shortest_path = algo::dijkstra(&graph, some_start, Some(end), |_| 1);
        let distance = match shortest_path.get(&end) {
            Some(v) => v.clone(),
            None => continue,
        };
        paths.push(distance);
    }

    let answer = paths.into_iter().min().unwrap();

    println!("Part 2: {:?}", answer);
}
