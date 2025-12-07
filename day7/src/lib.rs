use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Start,
    Splitter,
    Beam,
}
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Start => write!(f, "S"),
            Cell::Splitter => write!(f, "^"),
            Cell::Beam => write!(f, "|"),
        }
    }
}

impl FromStr for Cell {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Cell::Empty),
            "S" => Ok(Cell::Start),
            "^" => Ok(Cell::Splitter),
            "|" => Ok(Cell::Beam),
            _ => Err(anyhow::anyhow!("Invalid cell: {}", s)),
        }
    }
}
