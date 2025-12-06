use anyhow::Result;
use day6::Operation;
use std::str::FromStr;

fn main() -> Result<()> {
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    let input = common::read_file(&arg1)?;
    let worksheet = day6::Worksheet::from_str(&input)?;

    let results = worksheet
        .columns()
        .zip(worksheet.operations())
        .map(|(column, operation)| apply_operation(column, operation));

    println!("Part 1: {}", results.sum::<u64>());

    let (column_numbers, operations) = day6::Worksheet::parse_part2(&input)?;
    let results = column_numbers
        .into_iter()
        .zip(operations.into_iter())
        .map(|(column, operation)| apply_operation(column, operation));
    println!("Part 2: {:?}", results.sum::<u64>());

    Ok(())
}

fn apply_operation(row: impl IntoIterator<Item = u64>, operation: Operation) -> u64 {
    row.into_iter()
        .reduce(|acc, value| match operation {
            Operation::Add => acc + value,
            Operation::Multiply => acc * value,
        })
        .unwrap_or(0)
}
