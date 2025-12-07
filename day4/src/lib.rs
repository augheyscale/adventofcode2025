use anyhow::Result;
use common::grid::{CellInGrid, Grid, XY};

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
pub fn remove_cells(cells: &mut Grid<Cell>, xys: impl IntoIterator<Item = XY>) -> Result<usize> {
    let mut cleared_count = 0;
    for xy in xys {
        let cell = cells
            .get_mut(&xy)
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
pub fn is_paper(cell: &CellInGrid<Cell>) -> bool {
    matches!(cell.value(), Cell::Paper)
}

/// Checks if a cell is accessible based on the number of adjacent paper cells.
/// A cell is accessible if it has less than 4 adjacent paper cells.
pub fn is_accessible(cell: &CellInGrid<Cell>) -> bool {
    let adjacent_cells = cell.adjacent_cells_ref();
    let adjacent_cells_with_paper = adjacent_cells.filter(is_paper);
    adjacent_cells_with_paper.count() < 4
}
