use anyhow::Result;
use std::str::FromStr;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Battery {
    joltage: u64,
}
impl Battery {
    fn from_char(c: char) -> Result<Self> {
        Ok(Battery {
            joltage: c
                .to_digit(10)
                .ok_or_else(|| anyhow::anyhow!("Invalid joltage: {}", c))?
                as u64,
        })
    }
}

pub struct BatteryBank {
    batteries: Vec<Battery>,
}
impl FromStr for BatteryBank {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s
            .chars()
            .map(|c| Battery::from_char(c))
            .collect::<Result<Vec<_>>>()?;
        // must have at least 2 batteries
        if batteries.len() < 2 {
            anyhow::bail!("Must have at least 2 batteries");
        }
        Ok(BatteryBank { batteries })
    }
}
impl BatteryBank {
    pub fn recursive_max_joltage(&self, num_batteries: u32) -> u64 {
        recursive_max_joltage(&self.batteries, num_batteries)
            .expect("Must have at least 2 batteries")
    }
    pub fn max_pairs(&self) -> u64 {
        let batteries = self.batteries.as_slice();

        let (pos, max) = first_max(batteries).expect("There are always at least 2 batteries");
        let next_max = batteries
            .get(pos + 1..)
            .and_then(|batteries| batteries.iter().max());
        // if the max is not at the end, we find the next highest in the list
        let (first, second) = if let Some(next_max) = next_max {
            (max, next_max)
        } else {
            // Max was at the end, so we need to find the next highest in the list
            let batteries_without_the_end = batteries
                .get(0..batteries.len() - 1)
                .expect("There are always at least 2 batteries");
            let (pos, max) =
                first_max(batteries_without_the_end).expect("there is always at least one battery");
            let batteries_starting_at_max = batteries
                .get(pos + 1..)
                .expect("There are always at least 2 batteries");
            let next_max = batteries_starting_at_max
                .iter()
                .max()
                .expect("Must have at least one");
            (max, next_max)
        };

        first.joltage * 10 + second.joltage
    }
}

fn recursive_max_joltage(batteries: &[Battery], num_batteries: u32) -> Option<u64> {
    if num_batteries == 0 {
        return Some(0);
    }
    if batteries.is_empty() {
        return None;
    }

    let mut less_than = 10;

    while less_than > 0 {
        let without = batteries
            .iter()
            .filter(|battery| battery.joltage < less_than);
        let (pos, max) = first_max(without)?;
        let batteries_after = batteries.get(pos + 1..)?;
        if let Some(child_max) = recursive_max_joltage(batteries_after, num_batteries - 1) {
            // Multiplier is actually a 10's based shift.  So 1 is 1, 2 is 10, 3 is 100, etc.
            let multiplier = 10_u64.pow(num_batteries - 1);
            return Some(max.joltage * multiplier + child_max);
        } else {
            less_than = max.joltage;
        }
    }
    None
}

pub fn first_max<V>(iter: impl IntoIterator<Item = V>) -> Option<(usize, V)>
where
    V: Ord,
{
    let mut iter = iter.into_iter().enumerate();
    let mut max = iter.next();
    for (pos, v) in iter {
        if v > max.as_ref().unwrap().1 {
            max.replace((pos, v));
        }
    }
    max
}

pub fn read_input(path: impl AsRef<str>) -> Result<String> {
    Ok(std::fs::read_to_string(path.as_ref())?)
}

pub fn parse_input(input: &str) -> Result<Vec<BatteryBank>> {
    input.lines().map(BatteryBank::from_str).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1234567890\n123";

        let banks = parse_input(input).expect("Failed to parse input");
        assert_eq!(banks.len(), 2);
        let first_bank = banks.first().expect("No bank");
        assert_eq!(first_bank.batteries.len(), 10);
    }

    static TEST_DATA: &[(&str, u64)] = &[
        ("1234567890", 90),
        ("811111111111119", 89),
        ("234234234234278", 78),
        ("818181911112111", 92),
    ];

    #[test]
    fn test_max_pairs() {
        for (input, expected) in TEST_DATA.iter() {
            let bank = BatteryBank::from_str(input).expect("Failed to parse input");
            assert_eq!(bank.max_pairs(), *expected);
        }
    }

    #[test]
    fn test_recursive_max_joltage() {
        for (input, expected) in TEST_DATA.iter() {
            let bank = BatteryBank::from_str(input).expect("Failed to parse input");
            assert_eq!(recursive_max_joltage(&bank.batteries, 2), Some(*expected));
        }
    }
}
