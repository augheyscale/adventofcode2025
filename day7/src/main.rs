use std::collections::{HashMap, HashSet};

use anyhow::Result;
use common::grid::{CellInGrid, Grid, XY};
use day7::Cell;

fn main() -> Result<()> {
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    let data = common::read_file(&arg1)?;
    let grid = common::grid::parse_data_into_grid::<Cell>(&data)?;

    println!("Part 1: {}", part1(&mut grid.clone())?);
    println!("Part 1 again: {}", part1_again(&grid)?);
    println!("Part 2: {}", part2(&grid)?);
    Ok(())
}

fn part1_again(grid: &Grid<Cell>) -> Result<u64> {
    // Find the starting position.
    let start_pos = grid
        .cells()
        .find(|c| c.value() == &Cell::Start)
        .ok_or_else(|| anyhow::anyhow!("No start position found"))?;

    // Store the active beams in a set.
    let mut active_beams = HashSet::from([start_pos]);
    // Keep track of how many times we split.
    let mut split_count = 0;
    while !active_beams.is_empty() {
        // Evaluate the beams
        active_beams = active_beams
            .iter()
            // This map returns an array of two options.  Either
            // a left and right beam that then goes down, or a
            // beam that just goes down (and a second None value).
            .flat_map(|beam| match beam.value() {
                Cell::Splitter => {
                    split_count = split_count + 1;
                    [
                        beam.left().and_then(|b| b.down()),
                        beam.right().and_then(|b| b.down()),
                    ]
                }
                // If the beam is not a splitter, it just goes down.
                _ => [beam.down(), None],
            })
            // Filter out any None values (off the edge of the grid).
            .filter_map(|b| b)
            .collect::<HashSet<_>>();
    }
    Ok(split_count)
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

fn part2(grid: &Grid<Cell>) -> Result<impl std::fmt::Display> {
    let start_pos = grid
        .cells()
        .find(|c| c.value() == &Cell::Start)
        .ok_or_else(|| anyhow::anyhow!("No start position found"))?;

    // Add 1 to include our own timeline.
    Ok(part2_recursive(Some(start_pos), &mut HashMap::new()) + 1)
}

// The saving grace here is to keep a cache of what the count is from a given position.
// The recursive call will hit many positions many times, so this avoids recalculating the same
// value many times.
fn part2_recursive(in_pos: Option<CellInGrid<Cell>>, cache: &mut HashMap<XY, u64>) -> u64 {
    // We're off the grid, so no recursive calls.
    let Some(pos) = in_pos else {
        return 0;
    };

    // We've been here before, so return the cached value.
    if let Some(value) = cache.get(&pos.xy()) {
        return *value;
    }

    let count = match pos.value() {
        Cell::Splitter => {
            let left_count = part2_recursive(pos.left().and_then(|p| p.down()), cache);
            let right_count = part2_recursive(pos.right().and_then(|p| p.down()), cache);
            left_count + right_count + 1
        }
        _ => part2_recursive(pos.down(), cache),
    };

    cache.insert(pos.xy(), count);
    count
}
