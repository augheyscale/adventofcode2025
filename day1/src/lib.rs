use anyhow::Result;
pub mod lock;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

impl std::str::FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(anyhow::anyhow!("Invalid direction: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct Rotations {
    _direction: Direction,
    count: i32,
}
impl std::str::FromStr for Rotations {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, count) = s.split_at(1);
        let direction = direction.parse::<Direction>()?;
        let count = count.parse::<i32>()?;
        let count = match direction {
            Direction::Left => -count,
            Direction::Right => count,
        };
        Ok(Rotations {
            _direction: direction,
            count,
        })
    }
}

impl Rotations {
    pub fn signed_direction(&self) -> i32 {
        self.count
    }
}

pub fn parse_data(lines: impl Iterator<Item = String>) -> impl Iterator<Item = Rotations> {
    lines.map(|line| line.parse::<Rotations>().unwrap())
}

pub fn read_data_lines(path: &str) -> Result<impl Iterator<Item = String>> {
    let file = std::fs::File::open(path)?;
    use std::io::BufRead;
    Ok(std::io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Failed to read line")))
}
