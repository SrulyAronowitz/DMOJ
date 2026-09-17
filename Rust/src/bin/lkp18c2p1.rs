use std::{cmp::Reverse, collections::BinaryHeap, io::{self, Write}};

fn main() {
    let mut writer = io::BufWriter::new(io::stdout().lock());

    let mut first_line = String::new();
    std::io::stdin().read_line(&mut first_line).unwrap();
    let count_of_people = first_line.split_whitespace().nth(1).unwrap().trim().parse::<usize>().unwrap();

    let mut second_line = String::new();
    std::io::stdin().read_line(&mut second_line).unwrap();
    let input_lines: Vec<usize> = second_line.split_whitespace().into_iter().map(|value| value.trim().parse::<usize>().unwrap()).collect();

    let mut operating_lines: BinaryHeap<Reverse<usize>> = BinaryHeap::from(input_lines.into_iter().map(|value| Reverse(value)).collect::<Vec<Reverse<usize>>>());

    for _ in 0..count_of_people {
        let mut update_line = operating_lines.pop().unwrap();
        writeln!(writer, "{}", update_line.0).unwrap();
        update_line.0 += 1;
        operating_lines.push(update_line);
    }
}