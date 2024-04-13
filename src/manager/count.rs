/// Counts certain actions.
/// Not stored persistently.
/// Resets after each service restart.
pub struct CountManager {
    counter: u32,
}

impl CountManager {
    /// Creates a new `StatManager`-struct
    pub fn new() -> Self {
        CountManager { counter: 0 }
    }

    /// Increments the counter by one
    pub fn increment(&mut self) {
        self.counter += 1;
    }

    /// Increments the counter by the given value
    /// - `amount` The number by which to increment the counter
    pub fn increment_by(&mut self, amount: u32) {
        self.counter += amount;
    }

    /// Returns the current counter value of this manager
    /// - `returns` The current counter value of this manager
    pub fn value(&self) -> u32 {
        self.counter
    }
}
