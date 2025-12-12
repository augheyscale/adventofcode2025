use anyhow::Result;
use common::grid::{Grid, XY};
pub mod parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Empty,
    Filled,
}
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Filled => write!(f, "#"),
        }
    }
}
impl std::str::FromStr for Cell {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Cell::Empty),
            "#" => Ok(Cell::Filled),
            _ => Err(anyhow::anyhow!("Invalid cell: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Present {
    pub grid: Grid<Cell>,
    pub occupied_cells: Vec<XY>,
}

impl std::hash::Hash for Present {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.occupied_cells.hash(state);
    }
}

impl Present {
    pub fn new(grid: Grid<Cell>) -> Self {
        let occupied_cells = grid
            .cells()
            .filter(|cell| cell.value() == &Cell::Filled)
            .map(|cell| cell.xy())
            .collect();
        Self {
            grid,
            occupied_cells,
        }
    }
    pub fn occupied_cells(&self) -> impl Iterator<Item = &XY> + Clone {
        self.occupied_cells.iter()
    }
    pub fn rotate_90(&self) -> Self {
        Self::new(self.grid.rotate_90())
    }
    pub fn rotate_180(&self) -> Self {
        Self::new(self.grid.rotate_180())
    }
    pub fn rotate_270(&self) -> Self {
        Self::new(self.grid.rotate_270())
    }
    pub fn flip_horizontal(&self) -> Self {
        Self::new(self.grid.flip_horizontal())
    }
    pub fn flip_vertical(&self) -> Self {
        Self::new(self.grid.flip_vertical())
    }
}

#[derive(Debug, Clone)]
pub struct Region {
    pub xsize: usize,
    pub ysize: usize,
    pub present_count: Vec<usize>,
}
impl Region {
    pub fn presents<'a>(
        &'a self,
        presents: &'a [Present],
    ) -> impl Iterator<Item = &'a Present> + Clone {
        self.present_count
            .iter()
            .enumerate()
            .filter(|(_, count)| **count > 0)
            .flat_map(move |(i, count)| {
                let present = presents.get(i).expect("present index out of bounds");
                (0..*count).map(move |_| present)
            })
    }
}

#[derive(Debug)]
pub struct Problem {
    pub presents: Vec<Present>,
    pub regions: Vec<Region>,
}
impl Problem {
    pub fn try_new(presents: Vec<Present>, regions: Vec<Region>) -> Result<Self> {
        // length of each region's present count must be the same as the length of presents
        if regions
            .iter()
            .any(|region| region.present_count.len() != presents.len())
        {
            anyhow::bail!("Length of present count must be the same as the length of presents");
        }

        Ok(Problem { presents, regions })
    }
}
