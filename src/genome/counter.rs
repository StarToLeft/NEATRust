pub struct Counter {
    current_innovation: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            current_innovation: 0,
        }
    }

    pub fn get_innovation(&mut self) -> i32 {
        self.current_innovation += 1;

        self.current_innovation
    }
}
