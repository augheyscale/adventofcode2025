use std::{
    collections::HashSet,
    sync::atomic::{AtomicUsize, Ordering},
};

use anyhow::Result;
use common::grid::{Grid, XY};
use day12::{Cell, Present, Problem, parse::parse_problem};
use rayon::prelude::*;

fn main() -> Result<()> {
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    let data = common::read_file(&arg1)?;
    let problem = parse_problem(&data)?;
    println!("Part 1: {}", part1(&problem)?);
    println!("Part 2: {}", part2(&problem)?);
    Ok(())
}

fn part1(problem: &Problem) -> Result<usize> {
    let grids = problem.regions.par_iter().map(|region| {
        let presents_in_region = region.presents(&problem.presents);
        let empty_grid = Grid::new_sized(region.xsize, region.ysize, Cell::Empty);
        (empty_grid, presents_in_region)
    });

    let total_count = problem.regions.len();
    let count = AtomicUsize::new(0);

    let solved_grids = grids
        .map(|(grid, presents_in_region)| {
            println!("Solving grid: {} x {}", grid.width(), grid.height());
            let res = solve_grid(&grid, presents_in_region);
            let cur_count = count.fetch_add(1, Ordering::Relaxed);
            println!(
                "Solved grid {} x {}.  {}/{}",
                grid.width(),
                grid.height(),
                cur_count,
                total_count
            );

            res
        })
        .collect::<Result<Vec<_>>>()?;
    let solved_grid_count = solved_grids.iter().filter(|grid| grid.is_some()).count();
    Ok(solved_grid_count)
}

fn part2(_problem: &Problem) -> Result<usize> {
    Ok(0)
}

fn solve_grid<'a>(
    grid: &Grid<Cell>,
    mut presents: impl Iterator<Item = &'a Present> + Clone,
) -> Result<Option<()>> {
    let Some(this_present) = presents.next() else {
        println!("Solved!");
        println!("Grid:\n{:?}", grid);
        return Ok(Some(()));
    };

    let all_orientations = all_orientations(this_present).collect::<HashSet<_>>();

    for orientation in all_orientations {
        // println!("Trying to place:\n{:?}", orientation.grid);
        // println!("Inside grid:\n{:?}", grid);
        // println!("--------------------------------");
        let result = place_present_and_solve(grid, &orientation, presents.clone())?;
        if result.is_some() {
            return Ok(result);
        }
    }
    Ok(None)
}

fn all_orientations(present: &Present) -> impl Iterator<Item = Present> + Clone + use<> {
    [
        present.clone(),
        present.rotate_90(),
        present.rotate_180(),
        present.rotate_270(),
        present.flip_horizontal(),
        present.flip_vertical(),
    ]
    .into_iter()
}

fn place_present_and_solve<'a>(
    grid: &Grid<Cell>,
    present: &Present,
    presents: impl Iterator<Item = &'a Present> + Clone,
) -> Result<Option<()>> {
    let possible_placements = xy_possibilities(
        grid.width(),
        grid.height(),
        present.grid.width(),
        present.grid.height(),
    )
    .filter(|xy| can_place_present(grid, present, xy));
    for placement in possible_placements {
        let mut grid = grid.clone();
        place_present(&mut grid, present, &placement)?;
        let result = solve_grid(&grid, presents.clone())?;
        if result.is_some() {
            return Ok(result);
        }
    }
    Ok(None)
}

fn place_present(grid: &mut Grid<Cell>, present: &Present, offset: &XY) -> Result<()> {
    for xy in present.occupied_cells().map(|xy| xy.add(offset)) {
        let cell = grid
            .get_mut(&xy)
            .ok_or_else(|| anyhow::anyhow!("xy should be in grid"))?;
        *cell = Cell::Filled;
    }
    Ok(())
}

fn xy_possibilities(
    grid_xsize: usize,
    grid_ysize: usize,
    present_xsize: usize,
    present_ysize: usize,
) -> impl Iterator<Item = XY> {
    (0..grid_xsize - present_xsize + 1)
        .flat_map(move |x| (0..grid_ysize - present_ysize + 1).map(move |y| XY::new(x, y)))
}

fn can_place_present(grid: &Grid<Cell>, present: &Present, offset: &XY) -> bool {
    present
        .occupied_cells()
        .map(|xy| xy.add(offset))
        .map(|xy| grid.get(xy).expect("xy should be in grid"))
        .all(|cell| cell.value() == &Cell::Empty)
}
