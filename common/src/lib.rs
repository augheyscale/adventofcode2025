pub mod grid;

/// Reads the contents of a file.
pub fn read_file(path: &str) -> std::io::Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

pub fn sum_results<T: CheckedAdd<T> + Default, E>(
    i: impl Iterator<Item = anyhow::Result<T>>,
) -> anyhow::Result<T> {
    let mut sum = T::default();
    for v in i {
        sum = sum
            .checked_add(v?)
            .ok_or_else(|| anyhow::anyhow!("add overflowed"))?;
    }
    Ok(sum)
}

// checked_add functions on u16, i16, u32, i32, etc are not defined as a trait.
// This is our own definition of checked_add that is implemented for a few types used
// in the solutions.  Other types can be added as needed.
pub trait CheckedAdd<T> {
    fn checked_add(self, other: T) -> Option<T>;
}
impl CheckedAdd<u32> for u32 {
    fn checked_add(self, rhs: u32) -> Option<u32> {
        self.checked_add(rhs)
    }
}
impl CheckedAdd<u64> for u64 {
    fn checked_add(self, rhs: u64) -> Option<u64> {
        self.checked_add(rhs)
    }
}
impl CheckedAdd<usize> for usize {
    fn checked_add(self, rhs: usize) -> Option<usize> {
        self.checked_add(rhs)
    }
}

pub trait CountResults<T, E> {
    fn count_results(self) -> Result<usize, E>;
}
impl<T, E, I> CountResults<T, E> for I
where
    I: Iterator<Item = Result<T, E>>,
{
    fn count_results(self) -> Result<usize, E> {
        let mut count = 0;
        for v in self {
            _ = v?;
            count += 1;
        }
        Ok(count)
    }
}

/// Similar to the sum() function on iterators, but will check for overflow of
/// the sum itself.
pub trait CheckedSum<T> {
    /// Sums the values in an iterator and checks for overflow of the sum itself.
    /// Returns None if the sum overflows.
    fn checked_sum(self) -> Option<T>;
}
impl<T, I> CheckedSum<T> for I
where
    I: Iterator<Item = T>,
    T: std::ops::Add<Output = T> + Default + CheckedAdd<T>,
{
    fn checked_sum(self) -> Option<T> {
        let mut sum = T::default();
        for v in self {
            sum = sum.checked_add(v)?;
        }
        Some(sum)
    }
}
