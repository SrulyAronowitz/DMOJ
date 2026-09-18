use std::io::{self, Write, BufWriter};

fn main() {
    let mut writer = BufWriter::new(io::stdout().lock());

    let mut first_input = String::new();
    std::io::stdin().read_line(&mut first_input).unwrap();
    let number_of_input_lines = first_input.trim().parse::<usize>().unwrap();

    let mut problem_input = String::new();
    for _ in 0..number_of_input_lines {
        problem_input.clear();
        std::io::stdin().read_line(&mut problem_input).unwrap();

        let mut str_values = problem_input.split_whitespace().into_iter();

        let expression_values = (
            str_values.next().unwrap().parse::<i32>().unwrap(),
            str_values.next().unwrap().parse::<i32>().unwrap(), 
            str_values.next().unwrap().parse::<i64>().unwrap()
        );

        if ((expression_values.0 as i64) * (expression_values.1 as i64)) == expression_values.2 {
            writeln!(writer, "POSSIBLE DOUBLE SIGMA").unwrap();
        } else {
            writeln!(writer, "16 BIT S/W ONLY").unwrap();
        }
    }
}