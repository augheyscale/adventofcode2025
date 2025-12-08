use anyhow::Result;
use day8::{part1, part2};

fn main() -> Result<()> {
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    let data = common::read_file(&arg1)?;
    let xyzs = day8::parse_data(&data)?;
    println!("Part 1: {}", part1(&xyzs)?);
    println!("Part 2: {}", part2(&xyzs)?);
    Ok(())
}
