fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut str_values = input.split_whitespace();

    let map_constraints = (
        str_values.next().unwrap().parse::<usize>().unwrap(),
        str_values.next().unwrap().parse::<usize>().unwrap(),
        str_values.next().unwrap().parse::<usize>().unwrap(),
    );

    let mut map: Vec<Vec<usize>> = vec![];
    for _ in 0..(map_constraints.0) {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        str_values = input.split_whitespace();

        map.push(str_values.map(|value| value.parse::<usize>().unwrap()).collect());
    }
    
    println!("{:?}", map)
}