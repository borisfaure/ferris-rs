use crate::io_expander::IoExpander;

/// Right side of the keyboard
pub struct Right {
    /// The IO Expander used to control the right side
    io_expander: IoExpander,
}
impl Right {
    /// Create a new structure representing the right side of the keyboard
    pub fn new(io_expander: IoExpander) -> Self {
        Self { io_expander }
    }

    /// Scan the right side to know whick keys are pressed
    pub fn scan(&mut self) -> [[bool; 5]; 4] {
        let mut keys = [[false; 5]; 4];
        for row in 0_u8..=3_u8 {
            keys[row as usize] = self.io_expander.get_row(row);
        }
        keys
    }
}
