
use std::path::PathBuf;
use crate::terminal::Terminal;
use std::io::{self, stdout, Write};
use crossterm::{
    cursor,
    style::{self, Print},
    terminal::{self, Clear, ClearType},
    QueueableCommand, 
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};


// Cursor location on screen 0 indexed
#[derive(Default, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

// Single line of text
pub struct Row {
    pub string_content: String,
    pub render_content: String,
}

// The file that is being edited
pub struct Document {
    pub rows: Vec<Row>,
    pub filename: Option<PathBuf>,
}

pub struct Editor {
    pub should_quit: bool,
    pub terminal: Terminal, // Abstraction for stdout/stdin
    pub cursor_position: Position,
    pub offset: Position, // For scrolling aka which row if is at the top of the screen
    pub document: Document,
}

impl Editor {
    // Rendering function, clears screen, draws content, resets cursor
    pub fn refresh_screen(&self) -> Result<(), io::Error> {
        let mut stdout = stdout();

        // Hide cursor
        stdout.queue(cursor::Hide)?;
        // Queue command to move cursor to 0,0 aka top-left
        stdout.queue(cursor::MoveTo(0, 0))?;
        // Draw the rows
        self.draw_rows(&mut stdout)?;
        // Move cursor to the stored position in struct
        stdout.queue(cursor::MoveTo(
            self.cursor_position.x as u16,
            self.cursor_position.y as u16,
        ))?;

        // Show cursor again
        stdout.queue(cursor::Show)?;

        // Flush all queued commands to the OS
        stdout.flush()?;

        Ok(())
    }

    fn move_cursor(&mut self, key: KeyCode) {
        let Position { mut x, mut y } = self.cursor_position;

        // Retrieve terminal size to prevent going off screen
        let width = self.terminal.size.width as usize;
        let height = self.terminal.size.height as usize;

        // Map keys to coordinate changes
        match key {
            KeyCode::Up | KeyCode::Char('w') => {
                // saturating_sub to prevent underflow
                y = y.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('s') => {
                if y < height - 1 {
                    y += 1;
                }
            }
            KeyCode::Left | KeyCode::Char('a') => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right | KeyCode::Char('d') => {
                if x < width - 1 {
                    x += 1;
                }
            }
            _ => {}
        }

        self.cursor_position = Position { x, y };
    }

    pub fn process_keypress(&mut self, key: KeyEvent) -> Result<(), io::Error> {
        match key.code {
            // Quit CTRL + Q
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Up | KeyCode::Down | KeyCode:: Left | KeyCode::Right | KeyCode::Char('w') | KeyCode::Char('a') | KeyCode::Char('s') | KeyCode::Char('d') => {
                self.move_cursor(key.code);
            }
            _ => {
                // TODO: Character logic
            }
        }
        Ok(())
    }



    // Helper for drawing the rows
    fn draw_rows(&self, stdout: &mut io::Stdout) -> Result<(), io::Error> {
        // Iterate through the rows of the terminal height
        for i in 0..self.terminal.size.height {

            // Current line clear before drawing
            stdout.queue(terminal::Clear(ClearType::CurrentLine))?;

            // If we are on a line that doesnt have any text buffer then we draw a tilde
            if i >= self.document.rows.len() as u16 {
                if self.document.rows.is_empty() && i == self.terminal.size.height / 3 {
                    self.draw_welcome_message(stdout)?;
                } else {
                    stdout.queue(Print("~"))?;
                }
            } else {
                let row = &self.document.rows[i as usize];
                stdout.queue(Print(&row.render_content))?;       
            }

            // Move to the next line except if its the last one
            if i < self.terminal.size.height - 1 {
                stdout.queue(Print("\r\n"))?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message(&self, stdout: &mut io::Stdout) -> Result<(), io::Error> {
        let mut welcome = format!("Rust Editor -- version 0.1");
        let width = self.terminal.size.width as usize;
        let len = welcome.len();
        
        // Center the message
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding);
        welcome = format!("~{}{}", spaces, welcome);
        
        // Truncate if screen is too narrow
        welcome.truncate(width);
        
        stdout.queue(Print(welcome))?;
        Ok(())
    }
}