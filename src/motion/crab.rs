use super::Motion;
use std::io::Write;

pub struct CrabMotion {
    position: usize,
    direction: i32,
    terminal_width: usize,
    first_update: bool,
}

impl CrabMotion {
    pub fn new() -> Self {
        // Default terminal width, will be updated on first update
        let width = match term_size::dimensions() {
            Some((w, _)) => w.saturating_sub(5),
            None => 75,
        };
        
        Self {
            position: 0,
            direction: 1,
            terminal_width: width,
            first_update: true,
        }
    }
}

impl Motion for CrabMotion {
    fn update(&mut self) {
        // Get the latest terminal width in case it changed
        if let Some((w, _)) = term_size::dimensions() {
            self.terminal_width = w.saturating_sub(5);
        }
        
        // Update position
        let new_position = self.position as i32 + self.direction;
        
        // Check boundaries and reverse direction if needed
        if new_position <= 0 {
            self.position = 0;
            self.direction = 1;
        } else if new_position >= self.terminal_width as i32 {
            self.position = self.terminal_width;
            self.direction = -1;
        } else {
            self.position = new_position as usize;
        }
        
        // Handle the first update specially - print a newline to ensure we have room
        if self.first_update {
            self.first_update = false;
            // Hide cursor
            print!("\x1B[?25l");
        }
        
        // Clear the current line and move cursor to beginning
        print!("\r\x1B[K");
        
        // Print the crab at its position without a newline
        let spaces = " ".repeat(self.position);
        print!("{}🦀", spaces);
        
        // Flush stdout to ensure the crab is displayed immediately
        let _ = std::io::stdout().flush();
    }
}

impl Drop for CrabMotion {
    fn drop(&mut self) {
        // Show the cursor again when the program exits
        print!("\x1B[?25h");
        let _ = std::io::stdout().flush();
    }
}
