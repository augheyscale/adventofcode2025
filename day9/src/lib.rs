use anyhow::Result;
use common::grid::XY;

pub fn parse_data(data: &str) -> Result<Vec<XY>> {
    data.lines().map(|line| line.parse::<XY>()).collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    Red,
    Green,
    Empty,
    Inside,
    Outside,
}
impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Red => write!(f, "#"),
            Tile::Green => write!(f, "X"),
            Tile::Empty => write!(f, "."),
            Tile::Inside => write!(f, "I"),
            Tile::Outside => write!(f, "O"),
        }
    }
}
