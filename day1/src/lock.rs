#[derive(Debug)]
pub struct Lock {
    position: u32,
    count: u32,
}

impl Lock {
    pub fn new(starting_position: u32, count: u32) -> Self {
        Self {
            position: starting_position,
            count,
        }
    }
    // rotates and returns the new position
    pub fn rotate(&mut self, count: i32) -> u32 {
        // Deal with over and underflow.  Need to do wrapped arithmetic.
        // Convert count to i64 to handle large values safely
        let count_i64 = count as i64;
        let modulus_i64 = self.count as i64;

        // Compute count mod modulus, handling negative values
        // This gives us a value in range [0, modulus)
        let offset_i64 = ((count_i64 % modulus_i64) + modulus_i64) % modulus_i64;
        let offset = offset_i64 as u32;

        // Add offset to position and wrap around
        self.position = (self.position + offset) % self.count;
        self.position
    }
    pub fn at_zero(&self) -> bool {
        self.position == 0
    }
}
