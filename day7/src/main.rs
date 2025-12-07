use std::collections::HashSet;

use anyhow::Result;
use common::grid::{CellInGrid, Grid};
use day7::Cell;

fn main() -> Result<()> {
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    let data = common::read_file(&arg1)?;
    let mut grid = common::grid::parse_data_into_grid::<Cell>(&data)?;

    println!("Part 1: {}", part1(&mut grid)?);

    Ok(())
}

fn part1(grid: &mut Grid<Cell>) -> Result<impl std::fmt::Display> {
    let start_pos = grid
        .cells()
        .find(|c| c.value() == &Cell::Start)
        .ok_or_else(|| anyhow::anyhow!("No start position found"))?
        .xy();

    // Keep a list of beams
    let mut beams = vec![start_pos];
    let mut split_count = 0;
    loop {
        let down_beams = beams
            .iter()
            .filter_map(|xy| grid.get(xy.down()?))
            .collect::<HashSet<_>>();
        let down_beams = down_beams.iter();
        // Count the number of splitters
        split_count += down_beams
            .clone()
            .filter(|b| b.value() == &Cell::Splitter)
            .count();
        let down_with_split = down_beams
            .flat_map(|beam| split_beam(beam))
            .filter_map(|b| Some(b?.xy()));
        // Update our grid
        beams = down_with_split.collect::<Vec<_>>();
        for xy in beams.iter() {
            let cell = grid
                .get_mut(xy)
                .ok_or_else(|| anyhow::anyhow!("Beam not found"))?;
            *cell = Cell::Beam;
        }
        if beams.is_empty() {
            break;
        }
    }

    Ok(split_count)
}

fn split_beam<'a>(beam: &CellInGrid<'a, Cell>) -> [Option<CellInGrid<'a, Cell>>; 2] {
    match beam.value() {
        Cell::Splitter => beam.left_right(),
        _ => [Some(beam.clone()), None],
    }
}
