use anyhow::Result;
use common::grid::Grid;
use day4::Cell;

fn main() -> Result<()> {
    // Read data
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    let data = common::read_file(&arg1)?;
    let mut cells = common::grid::parse_data_into_grid::<Cell>(&data)?;

    // Run both parts 1 and 2
    part1(&cells)?;
    part2(&mut cells)?;
    isabel(&arg1)?;

    Ok(())
}

/// Part 1: Count the number of accessible paper cells.
fn part1(grid: &Grid<Cell>) -> Result<()> {
    let all_cells = grid.cells();
    let cells_with_paper = all_cells.filter(day4::is_paper);
    let accessible_paper_cells = cells_with_paper.filter(day4::is_accessible);
    let number_of_accessible_paper_cells = accessible_paper_cells.count();
    println!(
        "Part 1: Accessible paper cells: {}",
        number_of_accessible_paper_cells
    );
    Ok(())
}

fn part2(grid: &mut Grid<Cell>) -> Result<()> {
    let mut removed_count = 0;
    loop {
        let all_cells = grid.cells();
        let cells_with_paper = all_cells.filter(day4::is_paper);
        let accessible_paper_cell = cells_with_paper.filter(day4::is_accessible);
        // We have to collect the XYs into a vector because the grid needs to be mutable.
        let xy_to_remove = accessible_paper_cell.map(|c| c.xy()).collect::<Vec<_>>();

        // Remove each of these cells from the grid
        let cleared_count = day4::remove_cells(grid, xy_to_remove)?;
        if cleared_count == 0 {
            break;
        }
        removed_count += cleared_count;
    }
    println!("Part 2: Removed count: {}", removed_count);
    Ok(())
}

use std::fs::File;
use std::io::{self, BufRead};

fn isabel(file: &str) -> io::Result<()> {
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);

    let mut input_matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        input_matrix.push(line.chars().collect());
    }

    let mut count = 0;
    let mut removed_none = false;

    while !removed_none {
        let mut removed_some = false;
        for i in 0..input_matrix.len() {
            for j in 0..input_matrix[i].len() {
                if input_matrix[i][j] == '@' {
                    // check 8 directions
                    let directions = [
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, -1),
                        (0, 1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                    ];
                    let mut num_neighbors = 0;
                    for (di, dj) in directions.iter() {
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0
                            && ni < input_matrix.len() as isize
                            && nj >= 0
                            && nj < input_matrix[i].len() as isize
                        {
                            if input_matrix[ni as usize][nj as usize] == '@' {
                                num_neighbors += 1;
                            }
                        }
                    }
                    if num_neighbors < 4 {
                        count += 1;
                        input_matrix[i][j] = '.';
                        removed_some = true;
                    }
                }
            }
        }
        if !removed_some {
            removed_none = true;
        }
    }
    println!("Count of '@' with fewer than 4 '@' neighbors: {}", count);
    // for row in copy_matrix {
    //     let line: String = row.iter().collect();
    //     println!("{}", line);
    // }
    Ok(())
}
