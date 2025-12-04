use anyhow::Result;
use day4::Grid;

fn main() -> Result<()> {
    // Read data
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    let data = day4::read_file(&arg1)?;
    let mut cells = day4::parse_data_into_grid::<Cell>(&data)?;

    // Run both parts 1 and 2
    part1(&cells)?;
    part2(&mut cells)?;

    Ok(())
}

/// Part 1: Count the number of accessible paper cells.
fn part1(grid: &Grid<Cell>) -> Result<()> {
    let all_cells = grid.cells();
    let cells_with_paper = all_cells.filter(is_paper);
    let accessible_paper_cells = cells_with_paper.filter(is_accessible);
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
        let cells_with_paper = all_cells.filter(is_paper);
        let accessible_paper_cell = cells_with_paper.filter(is_accessible);
        // We have to collect the XYs into a vector because the grid needs to be mutable.
        let xy_to_remove = accessible_paper_cell.map(|c| c.xy()).collect::<Vec<_>>();

        // Remove each of these cells from the grid
        let cleared_count = remove_cells(grid, xy_to_remove)?;
        if cleared_count == 0 {
            break;
        }
        removed_count += cleared_count;
    }
    println!("Part 2: Removed count: {}", removed_count);
    Ok(())
}
/// Represents a cell in the grid, either empty or containing paper.
#[derive(Debug, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Paper,
}
impl Cell {
    /// Checks if the cell is empty.
    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }
}
impl std::str::FromStr for Cell {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "." => Ok(Cell::Empty),
            "@" => Ok(Cell::Paper),
            _ => Err(anyhow::anyhow!("Invalid cell: {}", s)),
        }
    }
}
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Paper => write!(f, "@"),
        }
    }
}

/// Removes paper cells from the grid at the specified positions.
pub fn remove_cells(
    cells: &mut Grid<Cell>,
    xys: impl IntoIterator<Item = day4::XY>,
) -> Result<usize> {
    let mut cleared_count = 0;
    for xy in xys {
        let cell = cells
            .get_mut(xy)
            .ok_or_else(|| anyhow::anyhow!("Cell not found"))?;

        // Cannot remove an empty cell.
        if cell.is_empty() {
            anyhow::bail!("Tried to remove an empty cell");
        }

        // Remove the cell.
        *cell = Cell::Empty;
        cleared_count += 1;
    }
    Ok(cleared_count)
}

/// Checks if a cell contains paper.
pub fn is_paper(cell: &day4::CellInGrid<Cell>) -> bool {
    matches!(cell.value(), Cell::Paper)
}

/// Checks if a cell is accessible based on the number of adjacent paper cells.
/// A cell is accessible if it has less than 4 adjacent paper cells.
pub fn is_accessible(cell: &day4::CellInGrid<Cell>) -> bool {
    let adjacent_cells = cell.adjacent_cells();
    let adjacent_cells_with_paper = adjacent_cells.filter(is_paper);
    adjacent_cells_with_paper.count() < 4
}
