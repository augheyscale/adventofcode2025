use std::ops::RangeInclusive;

use anyhow::Result;

type RangeType = u64;

fn main() -> Result<()> {
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    let data = common::read_file(&arg1)?;
    let (ranges, ing) = data.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            let start = start.parse::<RangeType>().unwrap();
            let end = end.parse::<RangeType>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();
    let ing = ing
        .lines()
        .map(|line| line.parse::<RangeType>().unwrap())
        .collect::<Vec<_>>();

    let ing_within_ranges = ing.iter().filter(|&i| ranges.iter().any(|r| r.contains(i)));
    let ing_not_in_ranges_count = ing_within_ranges.collect::<Vec<_>>();
    println!("Part 1: {}", ing_not_in_ranges_count.len());

    // Deconflict the ranges.  For all the ranges, find the overlapping ones
    // split and combine.

    // Sort ranges by start
    let mut ranges = ranges;
    ranges.sort_by_key(|r| *r.start());

    loop {
        if !do_one_merge(&mut ranges) {
            break;
        }
    }

    // Sum all ranges
    let sum: RangeType = ranges.iter().map(|r| r.end() + 1 - r.start()).sum();

    println!("Part2: {sum:?}");

    Ok(())
}

// Performs one merge operation with the provided ranges.
fn do_one_merge(ranges: &mut Vec<RangeInclusive<RangeType>>) -> bool {
    // Using indexed looping instead of iterators because we need to control mutability to modify the ranges
    for r1_pos in 0..ranges.len() {
        for r2_pos in r1_pos + 1..ranges.len() {
            let r1 = &ranges[r1_pos];
            let r2 = &ranges[r2_pos];
            if let Some(action) = what_action(r1, r2) {
                match action {
                    Action::RemoveR1 => {
                        //println!("Given: {r2:?} Removing: {r1:?}");
                        ranges.remove(r1_pos);
                    }
                    Action::RemoveR2 => {
                        //println!("Given: {r1:?} Removing: {r2:?}");
                        ranges.remove(r2_pos);
                    }
                    Action::Merge(new_r1) => {
                        //println!("Given: {r1:?} Replacing: {new_r1:?} and removing {r2:?}");
                        ranges[r1_pos] = new_r1;
                        ranges.remove(r2_pos);
                    }
                }
                return true;
            }
        }
    }
    return false;
}

enum Action {
    Merge(RangeInclusive<RangeType>),
    RemoveR2,
    RemoveR1,
}

fn what_action(r1: &RangeInclusive<RangeType>, r2: &RangeInclusive<RangeType>) -> Option<Action> {
    // r2 is entirely within r1
    if entirely_within(r1, r2) {
        return Some(Action::RemoveR2);
    }
    if entirely_within(r2, r1) {
        return Some(Action::RemoveR1);
    }

    // r2 start is within r1
    if second_range_within_first(r1, r2) {
        return Some(Action::Merge(*r1.start()..=*r2.end()));
    }

    // r1 start is within r2
    if second_range_within_first(r2, r1) {
        return Some(Action::Merge(*r2.start()..=*r1.end()));
    }

    None
}

// True if r2 is entirely within r1
fn entirely_within<T: Ord>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool {
    r2.start() >= r1.start() && r2.end() <= r1.end()
}

// Does r2 start within r1
fn second_range_within_first<T: Ord>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool {
    r2.start() >= r1.start() && r2.start() <= r1.end()
}
