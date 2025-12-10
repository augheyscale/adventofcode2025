use anyhow::Result;
fn main() -> Result<()> {
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    let data = common::read_file(&arg1)?;
    let data = day10::parse_data(&data)?;

    println!("Part 1: {:?}", data);

    Ok(())
}
