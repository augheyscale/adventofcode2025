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
impl Cell {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Cell::Empty),
            '@' => Ok(Cell::Paper),
            _ => Err(anyhow::anyhow!("Invalid cell: {}", c)),
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

pub struct CellInGrid<'a> {
    cell: &'a Cell,
    xy: XY,
    grid: &'a Grid,
}
impl std::fmt::Debug for CellInGrid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.xy)
    }
}
impl CellInGrid<'_> {
    pub fn xy(&self) -> XY {
        self.xy.clone()
    }
    pub fn adjacent_cells<'a>(&'a self) -> impl Iterator<Item = CellInGrid<'a>> {
        self.xy
            .adjacent_positions()
            .filter_map(|xy| self.grid.get(xy))
    }
    pub fn is_empty(&self) -> bool {
        self.cell.is_empty()
    }
    pub fn has_paper(&self) -> bool {
        !self.is_empty()
    }
    pub fn is_accessible(&self) -> bool {
        // A cell is accessible if there are fewer than 4 rolls of paper adjacent to it.
        let empty_adjacent_cells = self.adjacent_cells().filter(|cell| cell.has_paper());
        empty_adjacent_cells.count() < 4
    }
}

#[derive(Eq, PartialEq)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
}
impl FromStr for Grid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Cell::from_char(c))
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<Vec<_>>>>()?;
        Ok(Grid { cells })
    }
}
impl std::fmt::Debug for Grid {
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
impl Grid {
    pub fn cells<'a>(&'a self) -> impl Iterator<Item = CellInGrid<'a>> {
        self.cells.iter().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().map(move |(x, cell)| CellInGrid {
                cell,
                xy: XY::new(x, y),
                grid: self,
            })
        })
    }
    pub fn get<'a>(&'a self, xy: XY) -> Option<CellInGrid<'a>> {
        Some(CellInGrid {
            cell: self.cells.get(xy.y)?.get(xy.x)?,
            xy,
            grid: self,
        })
    }
    pub fn get_mut<'a>(&'a mut self, xy: XY) -> Option<&'a mut Cell> {
        self.cells.get_mut(xy.y)?.get_mut(xy.x)
    }
    /// Clears the cells at the given XYs and returns the number of cells cleared.
    pub fn clear_cells(&mut self, xys: impl IntoIterator<Item = XY>) -> Result<usize> {
        let mut cleared_count = 0;
        for xy in xys {
            *self
                .get_mut(xy)
                .ok_or_else(|| anyhow::anyhow!("Cell not found"))? = Cell::Empty;
            cleared_count += 1;
        }
        Ok(cleared_count)
    }
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

pub fn parse_data(data: &str) -> Result<Grid> {
    data.parse::<Grid>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_cell() {
        let cell = Cell::from_char('X');
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
