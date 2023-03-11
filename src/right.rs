use crate::IoExpander;

pub struct Right {
    io_expander: IoExpander,
    pub is_ok: bool,
}

impl Right {
    /// Create a new structure representing the right side of the keyboard
    pub fn new(io_expander: IoExpander) -> Self {
        let is_ok = io_expander.is_ok;
        Self { io_expander, is_ok }
    }

    /// Scan the right side to know whick keys are pressed
    pub fn get<E>(&mut self) -> Result<[[bool; 5]; 4], E> {
        let mut keys = [[false; 5]; 4];
        for row in 0_u8..=3_u8 {
            keys[row as usize] = self.io_expander.get_row(row);
        }
        Ok(keys)
    }
}
