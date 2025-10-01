use super::Motion;
use std::io::Write;
use rand::Rng;

pub struct MatrixMotion {
    columns: usize,
    rows: usize,
    drops: Vec<f32>, // Position of each drop (can be negative for off-screen)
    chars: Vec<char>, // Grid of characters
    terminal_width: usize,
    terminal_height: usize,
    first_update: bool,
    frame_count: u32,
    symbols: Vec<char>,
}

impl MatrixMotion {
    pub fn new() -> Self {
        let (width, height) = match term_size::dimensions() {
            Some((w, h)) => (w.saturating_sub(2), h), // Use full height
            None => (78, 20),
        };

        // Custom symbols including GUINETIK as requested
        let symbols = vec![
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
            'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
            'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            'G', 'U', 'I', 'N', 'E', 'T', 'I', 'K', // GUINETIK
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=',
            '+', '[', ']', '{', '}', '|', '\\', ':', ';', '"', '\'', '<', '>',
            ',', '.', '?', '/', '~', '`'
        ];

        let columns = width;
        let rows = height;
        let grid_size = columns * rows;

        // Initialize drops to be off-screen (negative positions)
        let mut drops = Vec::new();
        let mut rng = rand::rng();
        for _ in 0..columns {
            drops.push(-(rng.random_range(0..rows) as f32));
        }

        // Initialize character grid
        let mut chars = Vec::new();
        for _ in 0..grid_size {
            chars.push(' ');
        }

        Self {
            columns,
            rows,
            drops,
            chars,
            terminal_width: width,
            terminal_height: height,
            first_update: true,
            frame_count: 0,
            symbols,
        }
    }

    fn get_random_symbol(&self) -> char {
        let mut rng = rand::rng();
        self.symbols[rng.random_range(0..self.symbols.len())]
    }

    fn update_drops(&mut self) {
        let mut rng = rand::rng();
        
        // Move drops down and reset when they go off screen
        for i in 0..self.drops.len() {
            if self.drops[i] > self.rows as f32 + 10.0 || rng.random_bool(0.025) {
                // Reset drop to off-screen position
                self.drops[i] = -(rng.random_range(0..self.rows) as f32);
            } else {
                // Move drop down
                self.drops[i] += 1.0;
            }
        }
    }

    fn update_grid(&mut self) {
        let mut rng = rand::rng();
        
        // Update each character in the grid
        for row in 0..self.rows {
            for col in 0..self.columns {
                let index = row * self.columns + col;
                let drop_pos = self.drops[col];
                
                // Head of drop - bright white
                if row as f32 == drop_pos.floor() {
                    self.chars[index] = self.get_random_symbol();
                }
                // Tail of drop - fading green
                else if (row as f32) < drop_pos && (row as f32) > drop_pos - 8.0 {
                    // Keep the character but it will be rendered with fading green
                    if rng.random_bool(0.1) {
                        self.chars[index] = self.get_random_symbol();
                    }
                }
                // Empty space - occasionally show random characters
                else {
                    if rng.random_bool(0.05) {
                        self.chars[index] = self.get_random_symbol();
                    } else {
                        self.chars[index] = ' ';
                    }
                }
            }
        }
    }
}

impl Motion for MatrixMotion {
    fn update(&mut self) {
        // Get the latest terminal dimensions in case they changed
        if let Some((w, h)) = term_size::dimensions() {
            self.terminal_width = w.saturating_sub(2);
            self.terminal_height = h; // Use full height
            self.columns = self.terminal_width;
            self.rows = self.terminal_height;
            
            // Resize drops array if needed
            if self.drops.len() != self.columns {
                self.drops.resize(self.columns, -1.0);
            }
            
            // Resize chars array if needed
            let new_size = self.columns * self.rows;
            if self.chars.len() != new_size {
                self.chars.resize(new_size, ' ');
            }
        }

        // Handle the first update specially
        if self.first_update {
            self.first_update = false;
            // Hide cursor and clear screen
            print!("\x1B[?25l\x1B[2J\x1B[H");
        }

        // Update drops and grid
        self.update_drops();
        self.update_grid();

        // Clear screen and move cursor to top
        print!("\x1B[2J\x1B[H");

        // Render the matrix
        for row in 0..self.rows {
            for col in 0..self.columns {
                let index = row * self.columns + col;
                let char_to_display = self.chars[index];
                let drop_pos = self.drops[col];
                
                // Determine color and brightness based on position relative to drop
                if row as f32 == drop_pos.floor() {
                    // Head of drop - bright white
                    print!("\x1B[38;5;15m{}", char_to_display);
                } else if (row as f32) < drop_pos && (row as f32) > drop_pos - 8.0 {
                    // Tail of drop - fading green
                    let distance_from_head = drop_pos - row as f32;
                    let opacity = (0.9 - distance_from_head * 0.1).max(0.1);
                    let green_intensity = (opacity * 7.0) as u8 + 22; // Scale to green range 22-29
                    print!("\x1B[38;5;{}m{}", green_intensity, char_to_display);
                } else {
                    // Empty space - very dim green
                    print!("\x1B[38;5;22m{}", char_to_display);
                }
            }
            println!();
        }

        // Reset color and flush
        print!("\x1B[0m");
        let _ = std::io::stdout().flush();

        self.frame_count += 1;
    }
}

impl Drop for MatrixMotion {
    fn drop(&mut self) {
        // Show cursor and reset color when the program exits
        print!("\x1B[?25h\x1B[0m");
        let _ = std::io::stdout().flush();
    }
}
