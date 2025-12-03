use anyhow::Result;
use common::CheckedSum;

fn main() -> Result<()> {
    let file = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    part1_resulted(&day2::read_data(&file)?)?;
    part2(&day2::read_data(&file)?)?;
    Ok(())
}

#[allow(unused)]
fn part2(data: &str) -> Result<()> {
    let ranges = day2::parse_data(data);

    // Get all the invalid ids for all the ranges
    let invalid_ids = ranges.clone().flat_map(|r| r.invalid_ids_part2());

    println!("Part 2: {}", invalid_ids.sum::<day2::RangeType>());

    Ok(())
}

#[allow(unused)]
fn part1(data: &str) -> Result<()> {
    let ranges = day2::parse_data(data);

    // Get all the invalid ids for all the ranges
    let invalid_ids = ranges.flat_map(|r| r.invalid_ids());

    println!("Part 1: {}", invalid_ids.sum::<day2::RangeType>());
    Ok(())
}

#[allow(unused)]
fn part1_resulted(data: &str) -> Result<()> {
    let result_ranges = day2::parse_data_result(data);

    let mut total_sum = day2::RangeType::default();

    for r in result_ranges {
        // Safely unwrap r, get all the invalid ids for this range, and sum it safely.
        // Prop any overflow error.
        let this_sum = r?
            .invalid_ids()
            .checked_sum()
            .ok_or_else(|| anyhow::anyhow!("Sum overflow for invalid ids"))?;

        // Add this sum to our accumulated sum
        total_sum = total_sum
            .checked_add(this_sum)
            .ok_or_else(|| anyhow::anyhow!("Sum overflow for sum"))?;
    }

    println!("Part 1 Resulted: {}", total_sum);
    Ok(())
}
