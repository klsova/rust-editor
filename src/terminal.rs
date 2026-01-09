use crossterm::terminal::{enable_raw_mode, disable_raw_mode, size};
use std::io;

// Helper struct for terminal dimensions u16 is the standard
#[derive(Default, Clone, Copy)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    pub size: Size,
}

impl Terminal {
    // Constructor that enables the raw mode and gets the window size
    pub fn default()  -> Result<Self, io::Error> {
        // Raw mode
        enable_raw_mode()?;

        // Current terminal size        
        let (cols, rows) = size()?;

        Ok(Self {
            size: Size {
                width: cols,
                height: rows,
            },
        })
    }
}

// Incase program panics terminal goes back to a usable state
impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}