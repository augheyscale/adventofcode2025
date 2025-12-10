use anyhow::Result;
use common::grid::{Grid, XY};
use day9::Tile;
use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::{HashSet, VecDeque},
    sync::atomic::{AtomicUsize, Ordering},
};

fn main() -> Result<()> {
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    let data = common::read_file(&arg1)?;
    let data = day9::parse_data(&data)?;
    println!("Part 1: {}", part1(&data)?);
    println!("Part 2: {}", part2(&data)?);
    Ok(())
}

fn part1(data: &[XY]) -> Result<usize> {
    let xy_pairs = data.iter().tuple_combinations().map(sort_pair);

    let sizes = xy_pairs.map(|pair| rectangle_area(&pair));

    sizes.max().ok_or_else(|| anyhow::anyhow!("No sizes found"))
}

fn rectangle_area(pair: &(XY, XY)) -> usize {
    let (xy1, xy2) = pair;
    let dx = xy1.x.abs_diff(xy2.x) + 1;
    let dy = xy1.y.abs_diff(xy2.y) + 1;
    dx * dy
}

fn part2(data: &[XY]) -> Result<usize> {
    // Create a grid of the data
    let max_x = data
        .iter()
        .map(|xy| xy.x)
        .max()
        .ok_or_else(|| anyhow::anyhow!("No max x found"))?;
    let max_y = data
        .iter()
        .map(|xy| xy.y)
        .max()
        .ok_or_else(|| anyhow::anyhow!("No max y found"))?;
    let min_x = data
        .iter()
        .map(|xy| xy.x)
        .min()
        .ok_or_else(|| anyhow::anyhow!("No min x found"))?;
    let min_y = data
        .iter()
        .map(|xy| xy.y)
        .min()
        .ok_or_else(|| anyhow::anyhow!("No min y found"))?;

    println!("min_x: {}, min_y: {}", min_x, min_y);
    println!("max_x: {}, max_y: {}", max_x, max_y);

    let mut grid = Grid::<Tile>::new_sized(max_x + 2, max_y + 2, Tile::Empty);

    // Populate the grid with the data
    let coordinates = data.iter().cycle().take(data.len() + 1);
    let coordinate_pairs = coordinates.tuple_windows::<(&XY, &XY)>();

    for (xy1, xy2) in coordinate_pairs {
        drawline(&mut grid, xy1, xy2)?;
    }

    println!("Classifying tiles");

    classify_tiles(&mut grid)?;
    //classify_tiles_ray_casting(&mut grid)?;

    // Now go through the pairs as in part 1
    let xy_pairs = data
        .iter()
        .tuple_combinations::<(&XY, &XY)>()
        .map(sort_pair)
        .collect::<Vec<_>>();

    println!("Analyzing {} pairs", xy_pairs.len());

    let len = xy_pairs.len();
    let max_size = std::sync::RwLock::new(0);
    let completed = AtomicUsize::new(0);
    let bad_rectangles = std::sync::RwLock::new(HashSet::<(XY, XY)>::new());
    xy_pairs.par_iter().enumerate().for_each(|(index, pair)| {
        println!(
            "Index: {} of {}, completed: {}, remaining: {}",
            index,
            len,
            completed.load(Ordering::Relaxed),
            len - completed.load(Ordering::Relaxed)
        );
        let size = rectangle_area(pair);
        completed.fetch_add(1, Ordering::Relaxed);
        let smaller_than_max = {
            let max_size = max_size.read().unwrap();
            size <= *max_size
        };
        if smaller_than_max {
            return;
        }
        {
            let bad_rectangles = bad_rectangles.read().unwrap();
            if bad_rectangles
                .iter()
                .any(|badpair| is_inside(pair, badpair))
            {
                return;
            }
        }
        if rectangle_area_inside(pair, &grid).is_some() {
            println!(
                "Pair: {:?}, index: {} of {}, size: {}",
                pair, index, len, size
            );
            let mut max_size = max_size.write().unwrap();
            if size > *max_size {
                *max_size = size;
            }
        } else {
            let mut bad_rectangles = bad_rectangles.write().unwrap();
            // Only add if this pair is not already part of a larger rectangle.
            if bad_rectangles
                .iter()
                .any(|badpair| is_inside(badpair, pair))
            {
                return;
            }
            bad_rectangles.insert(pair.clone());
        }
    });
    Ok(*max_size.read().unwrap())
}

fn sort_pair(pair: (&XY, &XY)) -> (XY, XY) {
    // Sorts in both x and y
    let min_x = pair.0.x.min(pair.1.x);
    let max_x = pair.0.x.max(pair.1.x);
    let min_y = pair.0.y.min(pair.1.y);
    let max_y = pair.0.y.max(pair.1.y);
    (XY::new(min_x, min_y), XY::new(max_x, max_y))
}

// Is the rectangle defined by pair2 completely inside the rectangle defined by pair1?
fn is_inside(pair1: &(XY, XY), pair2: &(XY, XY)) -> bool {
    // pairs are already sorted by x and y
    let (min_x1, max_x1, min_y1, max_y1) = (pair1.0.x, pair1.1.x, pair1.0.y, pair1.1.y);
    let (min_x2, max_x2, min_y2, max_y2) = (pair2.0.x, pair2.1.x, pair2.0.y, pair2.1.y);

    min_x2 >= min_x1 && max_x2 <= max_x1 && min_y2 >= min_y1 && max_y2 <= max_y1
}

fn rectangle_area_inside(pair: &(XY, XY), grid: &Grid<Tile>) -> Option<usize> {
    // Pair is already sorted by x and y
    let (min_x, max_x, min_y, max_y) = (pair.0.x, pair.1.x, pair.0.y, pair.1.y);

    let mut xys_to_check =
        (min_x..=max_x).flat_map(|x| (min_y..=max_y).map(move |y| XY::new(x, y)));
    let any_outside = xys_to_check.any(|xy| {
        grid.get(xy)
            .map(|t| t.value() == &Tile::Outside)
            .unwrap_or(true)
    });
    if any_outside {
        return None;
    }
    Some(rectangle_area(pair))
}

fn drawline(grid: &mut Grid<Tile>, xy1: &XY, xy2: &XY) -> Result<()> {
    let line = line_between(xy1, xy2);
    let mut first = None;
    let mut last = None;
    for xy in line {
        if first.is_none() {
            first = Some(xy.clone());
        }
        last = Some(xy.clone());
        *grid
            .get_mut(&xy)
            .ok_or_else(|| anyhow::anyhow!("Cell not found"))? = Tile::Green;
    }
    if let (Some(first), Some(last)) = (first, last) {
        *grid
            .get_mut(&first)
            .ok_or_else(|| anyhow::anyhow!("Cell not found"))? = Tile::Red;
        *grid
            .get_mut(&last)
            .ok_or_else(|| anyhow::anyhow!("Cell not found"))? = Tile::Red;
    }
    Ok(())
}

fn line_between(xy1: &XY, xy2: &XY) -> impl Iterator<Item = XY> {
    let min_x = xy1.x.min(xy2.x);
    let max_x = xy1.x.max(xy2.x);
    let min_y = xy1.y.min(xy2.y);
    let max_y = xy1.y.max(xy2.y);

    (min_x..=max_x).flat_map(move |x| (min_y..=max_y).map(move |y| XY::new(x, y)))
}

fn classify_tiles(grid: &mut Grid<Tile>) -> Result<()> {
    // Find grid dimensions by iterating through cells
    let mut max_x = 0;
    let mut max_y = 0;
    for cell in grid.cells() {
        let xy = cell.xy();
        max_x = max_x.max(xy.x);
        max_y = max_y.max(xy.y);
    }
    let width = max_x + 1;
    let height = max_y + 1;

    // Flood fill from all edge tiles
    let mut queue = VecDeque::new();

    // Add all edge tiles to the queue if they're Empty
    // Top and bottom rows
    for x in 0..width {
        if let Some(tile) = grid.get_mut(&XY::new(x, 0)) {
            if matches!(*tile, Tile::Empty) {
                *tile = Tile::Outside;
                queue.push_back(XY::new(x, 0));
            }
        }
        if height > 1 {
            if let Some(tile) = grid.get_mut(&XY::new(x, height - 1)) {
                if matches!(*tile, Tile::Empty) {
                    *tile = Tile::Outside;
                    queue.push_back(XY::new(x, height - 1));
                }
            }
        }
    }

    // Left and right columns
    for y in 0..height {
        if let Some(tile) = grid.get_mut(&XY::new(0, y)) {
            if matches!(*tile, Tile::Empty) {
                *tile = Tile::Outside;
                queue.push_back(XY::new(0, y));
            }
        }
        if width > 1 {
            if let Some(tile) = grid.get_mut(&XY::new(width - 1, y)) {
                if matches!(*tile, Tile::Empty) {
                    *tile = Tile::Outside;
                    queue.push_back(XY::new(width - 1, y));
                }
            }
        }
    }

    // BFS flood fill from edge tiles
    while let Some(xy) = queue.pop_front() {
        for neighbor_xy in xy.adjacent_cardinal_positions() {
            if let Some(tile) = grid.get_mut(&neighbor_xy) {
                if matches!(*tile, Tile::Empty) {
                    *tile = Tile::Outside;
                    queue.push_back(neighbor_xy);
                }
            }
        }
    }

    // Mark all remaining Empty tiles as Inside
    for y in 0..height {
        for x in 0..width {
            let xy = XY::new(x, y);
            if let Some(tile) = grid.get_mut(&xy) {
                if matches!(*tile, Tile::Empty) {
                    *tile = Tile::Inside;
                }
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn classify_tiles_ray_casting(grid: &mut Grid<Tile>) -> Result<()> {
    // Find grid dimensions by iterating through cells
    let mut max_x = 0;
    let mut max_y = 0;
    for cell in grid.cells() {
        let xy = cell.xy();
        max_x = max_x.max(xy.x);
        max_y = max_y.max(xy.y);
    }
    let width = max_x + 1;
    let height = max_y + 1;

    // First, collect all Empty tile positions to avoid borrowing issues
    let mut empty_tiles = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let xy = XY::new(x, y);
            if let Some(cell) = grid.get(xy.clone()) {
                if matches!(cell.value(), Tile::Empty) {
                    empty_tiles.push(xy);
                }
            }
        }
    }

    // For each Empty tile, use ray casting to determine if it's inside or outside
    // Collect classifications first to avoid borrowing conflicts
    let mut classifications = Vec::new();
    for xy in &empty_tiles {
        // Cast a ray horizontally to the right and count boundary intersections
        let intersections = count_boundary_intersections(xy, grid, width);
        // Odd number of intersections = inside, even = outside
        let new_tile = if intersections % 2 == 1 {
            Tile::Inside
        } else {
            Tile::Outside
        };
        classifications.push((xy.clone(), new_tile));
    }

    // Now apply classifications
    for (xy, new_tile) in classifications {
        if let Some(tile) = grid.get_mut(&xy) {
            *tile = new_tile;
        }
    }

    Ok(())
}

fn count_boundary_intersections(start: &XY, grid: &Grid<Tile>, width: usize) -> usize {
    let y = start.y;
    let mut intersections = 0;
    let mut was_on_boundary = false;

    // Cast ray horizontally to the right
    for x in (start.x + 1)..width {
        let xy = XY::new(x, y);
        if let Some(cell) = grid.get(xy) {
            let is_boundary = matches!(cell.value(), Tile::Green | Tile::Red);

            // Count a crossing when we transition from non-boundary to boundary
            // Consecutive boundary tiles count as a single crossing
            if is_boundary {
                if !was_on_boundary {
                    // Entering boundary - count as intersection
                    intersections += 1;
                }
                was_on_boundary = true;
            } else {
                was_on_boundary = false;
            }
        } else {
            // Out of bounds - treat as non-boundary
            was_on_boundary = false;
        }
    }

    intersections
}
