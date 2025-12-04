use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Paper,
}
impl Cell {
    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }
}
impl FromStr for Cell {
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

pub struct CellInGrid<'a, Inner> {
    cell: &'a Inner,
    xy: XY,
    grid: &'a Grid<Inner>,
}
impl<Inner> std::fmt::Debug for CellInGrid<'_, Inner>
where
    Inner: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.xy)
    }
}
impl<Inner> CellInGrid<'_, Inner> {
    pub fn xy(&self) -> XY {
        self.xy.clone()
    }
    pub fn adjacent_cells<'a>(&'a self) -> impl Iterator<Item = CellInGrid<'a, Inner>> {
        self.xy
            .adjacent_positions()
            .filter_map(|xy| self.grid.get(xy))
    }
    pub fn value(&self) -> &Inner {
        self.cell
    }
}

#[derive(Eq, PartialEq)]
pub struct Grid<Inner> {
    cells: Vec<Vec<Inner>>,
}
impl<Inner: FromStr> FromStr for Grid<Inner> {
    type Err = <Inner as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Inner::from_str(&c.to_string()))
                    .collect::<Result<Vec<_>, Self::Err>>()
            })
            .collect::<Result<Vec<Vec<_>>, Self::Err>>()?;
        Ok(Grid { cells })
    }
}
impl<Inner> std::fmt::Debug for Grid<Inner>
where
    Inner: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl<Inner> Grid<Inner> {
    pub fn cells<'a>(&'a self) -> impl Iterator<Item = CellInGrid<'a, Inner>> {
        self.cells.iter().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().map(move |(x, cell)| CellInGrid {
                cell,
                xy: XY::new(x, y),
                grid: self,
            })
        })
    }
    pub fn get<'a>(&'a self, xy: XY) -> Option<CellInGrid<'a, Inner>> {
        Some(CellInGrid {
            cell: self.cells.get(xy.y)?.get(xy.x)?,
            xy,
            grid: self,
        })
    }
    pub fn get_mut<'a>(&'a mut self, xy: XY) -> Option<&'a mut Inner> {
        self.cells.get_mut(xy.y)?.get_mut(xy.x)
    }
}

/// Clears the cells at the given XYs and returns the number of cells cleared.
pub fn remove_cells(cells: &mut Grid<Cell>, xys: impl IntoIterator<Item = XY>) -> Result<usize> {
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct XY {
    x: usize,
    y: usize,
}
impl XY {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn adjacent_positions(&self) -> impl Iterator<Item = XY> {
        const DIRECTIONS: &[(isize, isize)] = &[
            // Up
            (-1, -1),
            (0, -1),
            (1, -1),
            // Center
            (-1, 0),
            (1, 0),
            // Down
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        DIRECTIONS.iter().filter_map(|(dx, dy)| {
            Some(XY {
                x: self.x.checked_add_signed(*dx)?,
                y: self.y.checked_add_signed(*dy)?,
            })
        })
    }
}

pub fn read_file(path: &str) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

pub fn parse_data(data: &str) -> Result<Grid<Cell>> {
    data.parse()
}

pub fn is_accessible(cell: &CellInGrid<Cell>) -> bool {
    let adjacent_cells = cell.adjacent_cells();
    let accessible_cells = adjacent_cells.filter(|c| matches!(c.value(), Cell::Paper));
    accessible_cells.count() < 4
}

pub fn is_paper(cell: &CellInGrid<Cell>) -> bool {
    matches!(cell.value(), Cell::Paper)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_cell() {
        let cell = Cell::from_str("X");
        assert!(cell.is_err());
    }

    #[test]
    fn test_parse_data() {
        let data = "..@..\n@.@.@\n..@..";
        let cells = parse_data(data).unwrap();
        assert_eq!(
            cells,
            Grid {
                cells: vec![
                    vec![
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Paper,
                        Cell::Empty,
                        Cell::Empty
                    ],
                    vec![
                        Cell::Paper,
                        Cell::Empty,
                        Cell::Paper,
                        Cell::Empty,
                        Cell::Paper
                    ],
                    vec![
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Paper,
                        Cell::Empty,
                        Cell::Empty
                    ]
                ]
            }
        );
    }

    #[test]
    fn test_adjacent_positions() {
        let xy = XY::new(0, 0);
        let adjacent_positions = xy.adjacent_positions().collect::<Vec<_>>();
        assert_eq!(adjacent_positions.len(), 3);
        assert!(adjacent_positions.contains(&XY::new(0, 1)));
        assert!(adjacent_positions.contains(&XY::new(1, 1)));
        assert!(adjacent_positions.contains(&XY::new(1, 0)));
    }
}
