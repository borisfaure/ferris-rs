use crate::IoExpander;

pub struct Right {
    io_expander: IoExpander,
    keys: [[bool; 5]; 4],
    pub is_ok: bool,
}

impl Right {
    pub fn new(io_expander: IoExpander) -> Self {
        let is_ok = io_expander.is_ok;
        let keys = [[false; 5]; 4];
        Self {
            io_expander,
            keys,
            is_ok,
        }
    }
}
