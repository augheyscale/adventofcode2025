use anyhow::Context;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Multiply,
}
impl FromStr for Operation {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err(anyhow::anyhow!("Invalid operation: {}", s)),
        }
    }
}

pub struct Worksheet {
    grid: Vec<Vec<u64>>,
    operations: Vec<Operation>,
}
impl FromStr for Worksheet {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        // Split lines into the grid and operations.  Operations is the last line.
        let (operations, grid_lines) = lines
            .split_last()
            .ok_or_else(|| anyhow::anyhow!("No operations line found"))?;

        let grid = grid_lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<u64>().context("Failed to parse grid cell"))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<Vec<u64>>, _>>()
            .context("Failed to parse grid")?;
        let operations = operations
            .split_whitespace()
            .map(Operation::from_str)
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to parse operations")?;

        // Make sure the column count is the same for all rows and operations
        {
            let operation_count = operations.len();
            if grid.iter().any(|row| row.len() != operation_count) {
                return Err(anyhow::anyhow!(
                    "Grid row count does not match operation count"
                ));
            }
        }

        Ok(Worksheet { grid, operations })
    }
}

impl Worksheet {
    pub fn parse_part2(input: &str) -> anyhow::Result<(Vec<Vec<u64>>, Vec<Operation>)> {
        let lines = input.lines().collect::<Vec<_>>();
        // Split lines into the grid and operations.  Operations is the last line.
        let (operations, grid_lines) = lines
            .split_last()
            .ok_or_else(|| anyhow::anyhow!("No operations line found"))?;

        // Split out the operations first, this will help tell how wide the columns are.
        // Operations look like "*    *  +   *  "
        // where the number of spaces between the operations is the width of the columns.
        let operations = split_operations_part2(operations).collect::<Vec<_>>();
        // Create a nxm grid of chars.
        let grid = grid_lines
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // Assert all rows are the same length.
        {
            let row_length = grid
                .get(0)
                .ok_or_else(|| anyhow::anyhow!("No grid rows found"))?
                .len();
            if grid.iter().any(|row| row.len() != row_length) {
                return Err(anyhow::anyhow!("Grid row lengths do not match"));
            }
        }

        let column_indices = column_indices(operations.iter().map(|operation| operation.len()));

        let column_numbers = column_indices
            .map(|(start_index, end_index)| {
                column_numbers(&grid, start_index, end_index).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let operations = operations
            .iter()
            .map(|op| Operation::from_str(op.trim()).context("Failed to parse operation"))
            .collect::<Result<Vec<_>, _>>()?;

        Ok((column_numbers, operations))
    }
}

fn column_numbers(
    grid: &[impl AsRef<[char]>],
    start_index: usize,
    end_index: usize,
) -> impl Iterator<Item = u64> {
    (start_index..end_index).map(move |column_index| accumlate_column(grid, column_index))
}

fn accumlate_column(grid: &[impl AsRef<[char]>], column_index: usize) -> u64 {
    let mut place_value = 1;
    walk_up_column(grid, column_index)
        .filter_map(|c| c.to_digit(10))
        .fold(0, |acc, digit| {
            let value = digit * place_value;
            place_value *= 10;

            acc + value as u64
        })
}

fn walk_up_column(grid: &[impl AsRef<[char]>], column_index: usize) -> impl Iterator<Item = char> {
    grid.iter().rev().map(move |row| row.as_ref()[column_index])
}

// Accumulates the column widths and returns the start and end indices of each column.
fn column_indices(
    column_widths: impl Iterator<Item = usize>,
) -> impl Iterator<Item = (usize, usize)> {
    column_widths.scan(0, |acc, width| {
        let start = *acc;
        *acc += width + 1;
        Some((start, *acc - 1))
    })
}

pub fn split_operations_part2(operations: &str) -> impl Iterator<Item = &str> {
    let mut start_index = 0;
    let mut chars = operations.chars().peekable();
    std::iter::from_fn(move || {
        let _operation = chars.next()?;
        let mut space_count = 0;
        while let Some(c) = chars.peek() {
            if c.is_whitespace() {
                space_count += 1;
                chars.next();
            } else {
                space_count -= 1;
                break;
            }
        }
        let end_index = start_index + space_count + 1;

        let ret = operations.get(start_index..end_index)?;
        start_index = end_index + 1;
        Some(ret)
    })
}

impl Worksheet {
    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = u64>> {
        self.grid.iter().map(|row| row.iter().copied())
    }
    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = u64>> {
        (0..self.grid[0].len()).map(|col| self.grid.iter().map(move |row| row[col]))
    }
    pub fn operations(&self) -> impl Iterator<Item = Operation> + Clone {
        self.operations.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_operations_part2() {
        let input = "*   +   *  + ";
        let operations = split_operations_part2(input);
        assert_eq!(
            operations.collect::<Vec<_>>(),
            vec!["*  ", "+  ", "* ", "+ "]
        );
    }

    #[test]
    fn test_column_indices() {
        let column_widths = vec![3, 3, 3, 3];
        let indices = column_indices(column_widths.iter().copied());
        assert_eq!(
            indices.collect::<Vec<_>>(),
            vec![(0, 3), (4, 7), (8, 11), (12, 15)]
        );
    }
}
