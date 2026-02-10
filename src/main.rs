mod structs;
mod terminal;

use structs::Editor;
use terminal::Terminal;
use crossterm::event::{Event, KeyEvent, read};
use std::io;
use std::env;


use crate::structs::{Document, Position};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terminal = Terminal::default()?;

    let args: Vec<String> = env::args().collect();

    let document = if args.len() > 1 {
        Document::open(&args[1])?
    } else {
        Document { rows: Vec::new(), filename: None }
    };

    let mut editor = Editor {
        should_quit: false,
        terminal: terminal,
        cursor_position: Position::default(),
        offset: Position::default(),
        document: document,
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

