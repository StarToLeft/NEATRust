pub struct Counter {
    pub current_innovation: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            current_innovation: 0,
        }
    }

    /// # get_innovation
    /// Gets the current innovation number and adds a 1 to it
    pub fn get_innovation(&mut self) -> i32 {
        self.current_innovation += 1;

        self.current_innovation
    }

    /// # load_innovation
    /// Returns the innovation number without adding to it
    pub fn load_innovation(&mut self) -> i32 {
        self.current_innovation
    }
}
