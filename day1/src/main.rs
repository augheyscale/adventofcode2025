use anyhow::Result;
use day1::{Rotations, lock::Lock, parse_data, read_data_lines};

// Note: this will panic on bad input
fn data(input_file: &str) -> Result<impl Iterator<Item = Rotations>> {
    let lines = read_data_lines(input_file)?;
    Ok(parse_data(lines))
}

fn main() -> Result<()> {
    // read data from arg 1
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;

    part1(data(&arg1)?)?;
    part2(data(&arg1)?)?;

    Ok(())
}

fn rotations_to_directions(data: impl Iterator<Item = Rotations>) -> impl Iterator<Item = i32> {
    data.map(|rotation| rotation.signed_direction())
}

fn part2(data: impl Iterator<Item = Rotations>) -> Result<()> {
    // given the data, get the rotation directions
    let rotation_directions = rotations_to_directions(data);

    // impl Iterator<Item = i32>
    let one_click_per = rotation_directions.flat_map(create_single_clicks);

    let count = count_zero_positions_of_lock(Lock::new(50, 100), one_click_per);

    println!("Part 2: Count: {}", count);

    Ok(())
}

/// Count the number of zero positions that a lock rests at each time
/// the lock is rotated by the directions.
fn count_zero_positions_of_lock(mut lock: Lock, directions: impl Iterator<Item = i32>) -> usize {
    // Map the directions to the lock positions
    let lock_positions = directions.map(|direction| lock.rotate(direction));
    // Filter the lock positions to only include the positions that are at zero
    let zero_positions = lock_positions.filter(|position| position == &0);
    // Count the number of zero positions
    zero_positions.count()
}

/// Create an iterator of single clicks for a given count
/// Example: create_single_clicks(10) -> [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
/// Example: create_single_clicks(-5) -> [-1, -1, -1, -1]
fn create_single_clicks(count: i32) -> impl Iterator<Item = i32> {
    let (count, one_click) = if count < 0 { (-count, -1) } else { (count, 1) };

    (0..count).map(move |_| one_click)
}

fn part1(data: impl Iterator<Item = Rotations>) -> Result<()> {
    // given the data, get the rotation directions
    // impl Iterator<Item = i32>
    let rotation_directions = rotations_to_directions(data);

    // given the rotation directions, get the lock positions
    // impl Iterator<Item = u32>
    let count = count_zero_positions_of_lock(Lock::new(50, 100), rotation_directions);

    println!("Part 1: Count: {}", count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clicks() {
        let clicks = create_single_clicks(10);
        assert_eq!(
            clicks.collect::<Vec<i32>>(),
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
        );
        let clicks = create_single_clicks(-10);
        assert_eq!(
            clicks.collect::<Vec<i32>>(),
            vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1]
        );
    }
}
