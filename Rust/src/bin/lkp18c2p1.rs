use std::{cmp::Reverse, collections::BinaryHeap, io::{self, Write}};

fn main() {
    let mut writer = io::BufWriter::new(io::stdout().lock());

    let mut first_input = String::new();
    std::io::stdin().read_line(&mut first_input).unwrap();
    let count_of_people = first_input.split_whitespace().nth(1).unwrap().trim().parse::<usize>().unwrap();
    
    let mut second_input = String::new();
    std::io::stdin().read_line(&mut second_input).unwrap();
    let mut operating_lines: BinaryHeap<Reverse<usize>> = BinaryHeap::from(second_input.split_whitespace().into_iter().map(|value| Reverse(value.trim().parse::<usize>().unwrap())).collect::<Vec<Reverse<usize>>>());

    for _ in 0..count_of_people {
        let mut update_line = operating_lines.pop().unwrap();
        writeln!(writer, "{}", update_line.0).unwrap();
        update_line.0 += 1;
        operating_lines.push(update_line);
    }
}