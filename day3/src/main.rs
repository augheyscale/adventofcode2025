use anyhow::Result;

fn main() -> Result<()> {
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    let data = day3::read_input(&arg1)?;
    let banks = day3::parse_input(&data)?;

    println!(
        "Part 1 with part 2 logic: {:?}",
        banks
            .iter()
            .map(|bank| bank.recursive_max_joltage(2))
            .sum::<u64>()
    );

    let max_pairs = banks.iter().map(|bank| bank.max_pairs());
    println!("Part 1: {}", max_pairs.sum::<u64>());
    let part2 = banks
        .iter()
        .map(|bank| bank.recursive_max_joltage(12))
        .sum::<u64>();
    println!("Part 2: {}", part2);
    Ok(())
}
