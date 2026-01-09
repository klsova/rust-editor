mod structs;
mod terminal;

use structs::Editor;
use terminal::Terminal;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use std::io;


use crate::structs::{Document, Position};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terminal = Terminal::default()?;

    let mut editor = Editor {
        should_quit: false,
        terminal: terminal,
        cursor_position: Position::default(),
        offset: Position::default(),
        document: Document {
            rows: Vec::new(),
            filename: None
        },
    };

    loop {
        editor.refresh_screen()?;

        if editor.should_quit {
            break;
        }

        let key = read_key()?;

        editor.process_keypress(key)?;
    }

    Ok(())
}

fn read_key() -> Result<KeyEvent, io::Error> {
    loop {
        if let Event::Key(event) = read()? {
            return Ok(event);
        }
    }
}

