use std::{io::BufRead, vec};

type OilAmount = u32;

struct MapConstraints {
    height: usize,
    width: usize,
    block: usize,
}

impl MapConstraints {
    fn block_offset(&self) -> usize {
        self.block - 1
    }
}

enum Grid {
    HeatCoordinate(Vec<Vec<OilAmount>>),
    PrefixSum(Vec<Vec<OilAmount>>),
    BlockValue(Vec<Vec<OilAmount>>),
}

impl Grid {
    fn to_prefix_sums(&mut self, constraints: &MapConstraints) -> Result<(), &str> {
        if let Grid::HeatCoordinate(map) = self {
            let mut prefix_sums = vec![vec![0; constraints.width + 1]; constraints.height + 1];

            for row_index in 0..constraints.height {
                for column_index in 0..constraints.width {
                    let top_left_value = prefix_sums[row_index][column_index];
                    let top_value = prefix_sums[row_index][column_index + 1];
                    let left_value = prefix_sums[row_index + 1][column_index];
                    let column_value = map[row_index][column_index];

                    prefix_sums[row_index + 1][column_index + 1] =
                        (top_value - top_left_value) + (left_value + column_value);
                }
            }

            *self = Grid::PrefixSum(prefix_sums);

            Ok(())
        } else {
            Err("Tried calling to_prefix_sums on a unsupported variant")
        }
    }

    fn to_block_values(&mut self, constraints: &MapConstraints) -> Result<(), &str> {
        if let Grid::PrefixSum(map) = self {
            let block_values_width = constraints.width - constraints.block_offset();
            let block_values_height = constraints.height - constraints.block_offset();

            let mut block_values = vec![vec![0; block_values_width]; block_values_height];

            for row_index in 0..block_values_height {
                for column_index in 0..block_values_width {
                    let offset_row_index = row_index + constraints.block;
                    let offset_column_index = column_index + constraints.block;

                    let cell_value = map[row_index][column_index];
                    let top_value = map[row_index][offset_column_index];
                    let left_value = map[offset_row_index][column_index];
                    let bottom_right_value = map[offset_row_index][offset_column_index];

                    block_values[row_index][column_index] =
                        (cell_value + bottom_right_value) - (top_value + left_value);
                }
            }
            *self = Grid::BlockValue(block_values);

            Ok(())
        } else {
            Err("Tried calling to_block_values on a unsupported variant")
        }
    }
}

struct Map {
    grid: Grid,
    constraints: MapConstraints,
}

fn main() {
    // Collect constraints input
    let mut input = String::new();
    std::io::stdin().lock().read_line(&mut input).unwrap();
    let mut str_values = input.split_whitespace();

    // Build constraints
    let map_constraints = MapConstraints {
        height: str_values.next().unwrap().parse::<usize>().unwrap(),
        width: str_values.next().unwrap().parse::<usize>().unwrap(),
        block: str_values.next().unwrap().parse::<usize>().unwrap(),
    };

    // Collect grid values and build grid

    // Build map object

    // Divide grid
}
