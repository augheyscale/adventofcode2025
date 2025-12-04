use anyhow::Result;
use day4::Grid;

fn main() -> Result<()> {
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    let data = day4::read_file(&arg1)?;
    let mut cells = day4::parse_data(&data)?;

    part1(&cells)?;
    part2(&mut cells)?;

    Ok(())
}

fn part1(grid: &Grid) -> Result<()> {
    let all_cells = grid.cells();
    let cells_with_paper = all_cells.filter(|c| c.has_paper());
    let accessible_paper_cell = cells_with_paper.filter(|c| c.is_accessible());
    println!(
        "Part 1: Accessible paper cells: {}",
        accessible_paper_cell.count()
    );
    Ok(())
}

fn part2(grid: &mut Grid) -> Result<()> {
    let mut removed_count = 0;
    loop {
        let all_cells = grid.cells();
        let cells_with_paper = all_cells.filter(|c| c.has_paper());
        let accessible_paper_cell = cells_with_paper.filter(|c| c.is_accessible());
        // We have to collect the XYs to remove into a vector because the grid needs to be mutable.
        let xy_to_remove = accessible_paper_cell.map(|c| c.xy()).collect::<Vec<_>>();

        // Remove each of these cells from the grid
        let cleared_count = grid.clear_cells(xy_to_remove)?;
        if cleared_count == 0 {
            break;
        }
        removed_count += cleared_count;
    }
    println!("Part 2: Removed count: {}", removed_count);
    Ok(())
}
