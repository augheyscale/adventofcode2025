use std::collections::HashMap;

use anyhow::Result;
use day11::Graph;

fn main() -> Result<()> {
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    let data = common::read_file(&arg1)?;
    let data = day11::parse_data(&data)?;
    let data = {
        let mut data = data;
        data.insert("out", vec![]);
        data
    };

    println!(
        "Part 1 again: {}",
        recurse_traverse_part2(&data, "you", "out", &mut HashMap::new())?
    );
    let svr_to_fft = recurse_traverse_part2(&data, "svr", "fft", &mut HashMap::new())?;
    let fft_to_dac = recurse_traverse_part2(&data, "fft", "dac", &mut HashMap::new())?;
    let dac_to_out = recurse_traverse_part2(&data, "dac", "out", &mut HashMap::new())?;
    println!("Part 2: {}", dac_to_out * fft_to_dac * svr_to_fft);
    Ok(())
}

pub fn recurse_traverse_part2<'a>(
    graph: &'a Graph<'_>,
    start: &'a str,
    end: &'a str,
    count_cache: &mut HashMap<&'a str, usize>,
) -> Result<usize> {
    let ret = if start == end {
        1
    } else if let Some(count) = count_cache.get(start) {
        *count
    } else {
        let children_counts = graph
            .get(start)
            .ok_or_else(|| anyhow::anyhow!("Node {:?} not found", start))?
            .iter()
            .map(|child| {
                recurse_traverse_part2(graph, child, end, count_cache).expect("Invalid child")
            });

        let children_count = children_counts.sum();

        count_cache.insert(start, children_count);
        children_count
    };
    Ok(ret)
}
