pub struct PrintOptions {
    is_terminal: bool,
}

impl PrintOptions {
    pub fn new(is_terminal: bool) -> Self {
        Self { is_terminal }
    }

    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }
}
