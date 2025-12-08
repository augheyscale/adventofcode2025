use std::str::FromStr;

use anyhow::Result;

/// An x,y position in a two-dimensional grid.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct XY {
    x: usize,
    y: usize,
}

impl std::hash::Hash for XY {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl XY {
    /// Creates a new position with the given coordinates.
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    /// Returns an iterator of the adjacent cardinal positions.
    pub fn adjacent_cardinal_positions(&self) -> impl Iterator<Item = XY> {
        const DIRECTIONS: &[(isize, isize)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];
        DIRECTIONS.iter().filter_map(|(dx, dy)| {
            Some(XY {
                x: self.x.checked_add_signed(*dx)?,
                y: self.y.checked_add_signed(*dy)?,
            })
        })
    }

    /// Returns an iterator of all adjacent positions, including diagonals.
    pub fn adjacent_positions(&self) -> impl Iterator<Item = XY> + use<> {
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
        let (x, y) = (self.x, self.y);
        DIRECTIONS.iter().filter_map(move |(dx, dy)| {
            Some(XY {
                x: x.checked_add_signed(*dx)?,
                y: y.checked_add_signed(*dy)?,
            })
        })
    }

    pub fn down(&self) -> Option<XY> {
        self.y.checked_add(1).map(|y| XY::new(self.x, y))
    }
    pub fn left(&self) -> Option<XY> {
        self.x.checked_sub(1).map(|x| XY::new(x, self.y))
    }
    pub fn right(&self) -> Option<XY> {
        self.x.checked_add(1).map(|x| XY::new(x, self.y))
    }
}

/// A two-dimensional grid of cells.
#[derive(Eq, PartialEq)]
pub struct Grid<Inner> {
    cells: Vec<Vec<Inner>>,
}

impl<Inner: Clone> Clone for Grid<Inner> {
    fn clone(&self) -> Self {
        Grid {
            cells: self.cells.clone(),
        }
    }
}

impl<Inner: FromStr> FromStr for Grid<Inner> {
    type Err = <Inner as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_lines(s.lines())
    }
}
impl<Inner> Grid<Inner>
where
    Inner: FromStr,
{
    pub fn from_lines(
        lines: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Self, <Inner as FromStr>::Err> {
        let cells = lines
            .into_iter()
            .map(|line| {
                line.as_ref()
                    .chars()
                    .map(|c| Inner::from_str(&c.to_string()))
                    .collect::<Result<Vec<_>, <Inner as FromStr>::Err>>()
            })
            .collect::<Result<Vec<Vec<_>>, <Inner as FromStr>::Err>>()?;
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
    /// Returns an iterator over all cells in the grid.
    pub fn cells<'a>(&'a self) -> impl Iterator<Item = CellInGrid<'a, Inner>> {
        self.cells.iter().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().map(move |(x, cell)| CellInGrid {
                cell,
                xy: XY::new(x, y),
                grid: self,
            })
        })
    }
    /// Gets a cell at the specified position.
    pub fn get<'a>(&'a self, xy: XY) -> Option<CellInGrid<'a, Inner>> {
        Some(CellInGrid {
            cell: self.cells.get(xy.y)?.get(xy.x)?,
            xy,
            grid: self,
        })
    }
    /// Gets a mutable reference to a cell at the specified position.
    pub fn get_mut(&mut self, xy: &XY) -> Option<&mut Inner> {
        self.cells.get_mut(xy.y)?.get_mut(xy.x)
    }
}

/// A cell within a grid, providing access to the cell value and its position.
#[derive(Clone, Eq)]
pub struct CellInGrid<'a, Inner> {
    cell: &'a Inner,
    xy: XY,
    grid: &'a Grid<Inner>,
}

impl<Inner> PartialEq for CellInGrid<'_, Inner>
where
    Inner: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.xy == other.xy
    }
}

impl<Inner> std::hash::Hash for CellInGrid<'_, Inner> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.xy.hash(state);
    }
}

impl<Inner> std::fmt::Debug for CellInGrid<'_, Inner>
where
    Inner: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.xy)
    }
}
impl<'a, Inner> CellInGrid<'a, Inner> {
    /// Gets the position of this cell.
    pub fn xy(&self) -> XY {
        self.xy.clone()
    }

    pub fn left(&self) -> Option<CellInGrid<'a, Inner>> {
        self.xy.left().and_then(|xy| self.grid.get(xy))
    }
    pub fn right(&self) -> Option<CellInGrid<'a, Inner>> {
        self.xy.right().and_then(|xy| self.grid.get(xy))
    }
    pub fn down(&self) -> Option<CellInGrid<'a, Inner>> {
        self.xy.down().and_then(|xy| self.grid.get(xy))
    }
    pub fn left_right(&self) -> [Option<CellInGrid<'a, Inner>>; 2] {
        // Same trick: destructure self
        let CellInGrid { cell: _, xy, grid } = self;

        let left = xy.left().and_then(|xy| grid.get(xy));
        let right = xy.right().and_then(|xy| grid.get(xy));

        [left, right]
    }

    pub fn cardinal_direction_adjacent_cells(&self) -> impl Iterator<Item = CellInGrid<'_, Inner>> {
        self.xy
            .adjacent_cardinal_positions()
            .filter_map(|xy| self.grid.get(xy))
    }

    pub fn adjacent_cells_ref(&self) -> impl Iterator<Item = CellInGrid<'_, Inner>> {
        self.xy
            .adjacent_positions()
            .filter_map(move |xy| self.grid.get(xy))
    }

    /// Returns an iterator over all adjacent cells in the grid.
    pub fn adjacent_cells(self) -> impl Iterator<Item = CellInGrid<'a, Inner>> {
        // Same trick: destructure self
        let CellInGrid { cell: _, xy, grid } = self;
        xy.adjacent_positions().filter_map(move |xy| grid.get(xy))
    }
    /// Gets the value stored in this cell.
    pub fn value(&self) -> &Inner {
        self.cell
    }
}
impl<Inner> AsRef<Inner> for CellInGrid<'_, Inner> {
    fn as_ref(&self) -> &Inner {
        self.cell
    }
}

/// Parses a string into a grid of cells.
pub fn parse_data_into_grid<Inner>(data: &str) -> Result<Grid<Inner>, <Inner as FromStr>::Err>
where
    Inner: FromStr,
{
    data.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data() {
        let data = "..@..\n@.@.@\n..@..";
        let cells = parse_data_into_grid::<char>(data).unwrap();
        assert_eq!(
            cells,
            Grid {
                cells: vec![
                    vec!['.', '.', '@', '.', '.'],
                    vec!['@', '.', '@', '.', '@'],
                    vec!['.', '.', '@', '.', '.']
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
