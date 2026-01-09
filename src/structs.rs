
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
use std::fs;


// Cursor location on screen 0 indexed
#[derive(Default, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string_content: String::from(slice),
            render_content: String::from(slice),
        }
    }
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

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;

        let rows = contents
            .lines() // Iterator over lines that handles \n and \r\n
            .map(|line| Row::from(line))
            .collect();

        Ok(Self {
            rows,
            filename: Some(PathBuf::from(filename)),
        })
    }
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



    // Drawing the rows
    fn draw_rows(&self, stdout: &mut io::Stdout) -> Result<(), io::Error> {
        let height = self.terminal.size.height;
        
        for terminal_row in 0..height {
            // Clear current line
            stdout.queue(terminal::Clear(ClearType::CurrentLine))?;
            
            // We need to know which row of the file maps to this row of the screen.
            let file_row = terminal_row as usize;

            if file_row < self.document.rows.len() {
                let row = &self.document.rows[file_row];
                
                // If line is wider than screen, chop it off visually
                let len = std::cmp::min(row.render_content.len(), self.terminal.size.width as usize);
                
                // A slice to print only what fits
                stdout.queue(Print(&row.render_content[..len]))?;
            } else if self.document.rows.is_empty() && terminal_row == height / 3 {
                 self.draw_welcome_message(stdout)?;
            } else {
                 stdout.queue(Print("~"))?;
            }
            
            // New line handling
            if terminal_row < height - 1 {
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

