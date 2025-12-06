use anyhow::Result;
use std::str::FromStr;

pub type RangeType = u64;

#[derive(Debug, Clone)]
pub struct Range {
    start: RangeType,
    end: RangeType,
}
impl Range {
    pub fn try_new(start: RangeType, end: RangeType) -> Result<Self> {
        if start > end {
            anyhow::bail!("Start can not be greaTER than end")
        } else {
            Ok(Self { start, end })
        }
    }
    pub fn ids(&self) -> impl Iterator<Item = RangeType> + use<> + Clone {
        self.start..=self.end
    }
    pub fn invalid_ids(&self) -> impl Iterator<Item = RangeType> + use<> {
        self.ids().filter(|id| is_invalid_id(*id))
    }
    pub fn invalid_ids_part2(&self) -> impl Iterator<Item = RangeType> + use<> + Clone {
        self.ids().filter(|id| is_invalid_id_part2(*id))
    }
}

// In the case of an empty iterator, return false.
fn all_values_equal(iter: impl IntoIterator<Item = impl PartialEq>) -> bool {
    let mut iter = iter.into_iter();
    let first = iter.next();
    if first.is_none() {
        // Indicates the iterator is empty.
        return false;
    }
    iter.all(|x| &x == first.as_ref().unwrap())
}

fn combinations_of_str(s: &str, count: usize) -> impl Iterator<Item = &str> {
    let ranges = (0..)
        .map(move |offset| offset * count)
        .map(move |offset| offset..offset + count);
    ranges
        .map(|range| s.get(range))
        .take_while(|s| s.is_some())
        .map(|s| s.unwrap())
}

fn has_repeating_values(s: &str, count: usize) -> bool {
    // It can't generate equal substrings if the length is not divisible by the count.
    if !s.len().is_multiple_of(count) {
        return false;
    }

    all_values_equal(combinations_of_str(s, count))
}

/// An invalid id is one where, if you split the string version of the
/// number into two halves, the first half is the same as the second half.
fn is_invalid_id(id: RangeType) -> bool {
    // Get a string version of this number.
    let id = id.to_string();
    // Split it into the first and second half (don't worry if it's an odd split)
    let (first, last) = id.split_at(id.len() / 2);
    // Is the first half the same as the second half?
    first == last
}

fn is_invalid_id_part2(id: RangeType) -> bool {
    let id = id.to_string();
    let mut substr_lengths = 1..=id.len() / 2;
    substr_lengths.any(|length| has_repeating_values(&id, length))
}

// Implement string parsing for range.  A range is ####-#####
// and is a valid range (start < end)
impl FromStr for Range {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once('-')
            .ok_or_else(|| anyhow::anyhow!("Invalid range: {}", s))?;
        // no zero leading digits
        if start.chars().nth(0) == Some('0') || end.chars().nth(0) == Some('0') {
            anyhow::bail!("Cannot have leading zeros");
        }
        Range::try_new(start.parse()?, end.parse()?)
    }
}

pub fn read_data(path: impl AsRef<str>) -> Result<String> {
    Ok(std::fs::read_to_string(path.as_ref())?)
}

/// Safe version of parsing that incrementally parses and provides a Result Range
/// in case the Range::parse fails.
pub fn parse_data_result(data: &str) -> impl Iterator<Item = Result<Range>> + Clone {
    data.split(',').map(|pair| pair.trim().parse::<Range>())
}

/// Unsafe version that will panic on an invalid range.
pub fn parse_data(data: &str) -> impl Iterator<Item = Range> + Clone {
    parse_data_result(data).map(|r| r.expect("Valid range"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data() {
        let range = Range::from_str("1-10").unwrap();
        let ids = range.ids().collect::<Vec<_>>();
        assert_eq!(ids, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_combinations() {
        assert_eq!(
            combinations_of_str("1234567", 3).collect::<Vec<_>>(),
            vec!["123", "456"]
        );
        assert_eq!(
            combinations_of_str("123456789", 3).collect::<Vec<_>>(),
            vec!["123", "456", "789"]
        );
    }

    #[test]
    fn test_invalid_id() {
        assert!(is_invalid_id(1) == false);
        assert!(is_invalid_id(10) == false);
        assert!(is_invalid_id(11) == true);
        assert!(is_invalid_id(1010) == true);
        assert!(is_invalid_id(123) == false);
        assert!(is_invalid_id(12312) == false);
        assert!(is_invalid_id(123123) == true);
    }

    #[test]
    fn test_sample() {
        assert_eq!(
            Range::try_new(11, 22)
                .unwrap()
                .invalid_ids()
                .collect::<Vec<_>>(),
            vec![11, 22]
        );
        assert_eq!(
            Range::try_new(1188511880, 1188511890)
                .unwrap()
                .invalid_ids()
                .collect::<Vec<_>>(),
            vec![1188511885]
        );
    }

    #[test]
    fn test_all_values_equal() {
        assert!(all_values_equal(vec![1, 1, 1]));
        assert!(all_values_equal(vec![1, 1, 1, 1]));
        assert!(!all_values_equal(vec![1, 1, 2]));
        assert!(!all_values_equal(vec![1, 2, 1]));
        assert!(!all_values_equal(vec![2, 1, 1]));
        assert!(all_values_equal(vec![1]));
        assert!(!all_values_equal(Vec::<i32>::new()));
    }

    #[test]
    fn test_invalid_id_part2() {
        let invalid_ids = [11, 1010, 123123, 123123123];
        for id in invalid_ids {
            assert!(is_invalid_id_part2(id), "{} is invalid", id);
        }

        let valid_ids = [123, 12312, 12312312, 2121212119];
        for id in valid_ids {
            assert!(!is_invalid_id_part2(id), "{} is valid", id);
        }
    }
}
