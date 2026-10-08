use std::collections::HashMap;

struct MapConstraints {
    height: usize,
    width: usize,
    block: usize,
}

struct Map {
    grid: Vec<Vec<usize>>,
    prefix_sum: Vec<Vec<usize>>,
    constraints: MapConstraints,
}

impl Map {
    fn new(constraints: MapConstraints) -> Self {
        Self { 
            grid: vec![
                vec![0; constraints.width];
                constraints.height
            ],
            prefix_sum: vec![
                vec![0; constraints.width];
                constraints.height
            ],
            constraints: constraints,
        }
    }

    fn calculate_prefix_sum(&self) {
        for (row_index, value) in self.grid.iter().enumerate() {
            for cell in value {
                
            }
        }
    }
}

fn main() {
    // Collect input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut str_values = input.split_whitespace();

    let map_constraints = MapConstraints {
        height: str_values.next().unwrap().parse::<usize>().unwrap(),
        width: str_values.next().unwrap().parse::<usize>().unwrap(),
        block: str_values.next().unwrap().parse::<usize>().unwrap(),
    };

    // Get map input, build it, then calculate prefix sum
    let mut map = Map::new(map_constraints);
    for index in 0..(map.constraints.height) {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        str_values = input.split_whitespace();

        map.grid[index] = str_values.map(|value| value.parse::<usize>().unwrap()).collect();
    }
    map.calculate_prefix_sum();

    println!("{:?}", map.grid);
    println!("{:?}", map.prefix_sum);
}