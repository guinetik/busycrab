use super::Motion;
use std::io::Write;
use rand::Rng;

pub struct MatrixMotion {
    columns: usize,
    rows: usize,
    drops: Vec<f32>, // Position of each drop (can be negative for off-screen)
    drop_speeds: Vec<f32>, // Speed of each drop
    trail_lengths: Vec<usize>, // Length of each trail
    chars: Vec<Vec<char>>, // Grid of characters (column-major for trails)
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

        // Custom symbols - more Katakana-like for authentic Matrix feel
        let symbols = vec![
            // Half-width Katakana and Latin characters
            'ｦ', 'ｧ', 'ｨ', 'ｩ', 'ｪ', 'ｫ', 'ｬ', 'ｭ', 'ｮ', 'ｯ',
            'ｰ', 'ｱ', 'ｲ', 'ｳ', 'ｴ', 'ｵ', 'ｶ', 'ｷ', 'ｸ', 'ｹ',
            'ｺ', 'ｻ', 'ｼ', 'ｽ', 'ｾ', 'ｿ', 'ﾀ', 'ﾁ', 'ﾂ', 'ﾃ',
            'ﾄ', 'ﾅ', 'ﾆ', 'ﾇ', 'ﾈ', 'ﾉ', 'ﾊ', 'ﾋ', 'ﾌ', 'ﾍ',
            'ﾎ', 'ﾏ', 'ﾐ', 'ﾑ', 'ﾒ', 'ﾓ', 'ﾔ', 'ﾕ', 'ﾖ', 'ﾗ',
            'ﾘ', 'ﾙ', 'ﾚ', 'ﾛ', 'ﾜ', 'ﾝ',
            // Numbers and Latin letters
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
            'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            // GUINETIK signature
            'G', 'U', 'I', 'N', 'E', 'T', 'I', 'K',
            // Symbols
            ':', '.', '=', '*', '+', '-', '<', '>', '¦', '|', '/',
        ];

        let columns = width;
        let rows = height;

        let mut rng = rand::rng();

        // Initialize drops to be staggered
        let mut drops = Vec::new();
        let mut drop_speeds = Vec::new();
        let mut trail_lengths = Vec::new();

        for _ in 0..columns {
            drops.push(-(rng.random_range(0..rows) as f32));
            drop_speeds.push(rng.random_range(0.5..1.5));
            trail_lengths.push(rng.random_range(8..20));
        }

        // Initialize character grid (column-major storage)
        let mut chars = Vec::new();
        for _ in 0..columns {
            let mut column = Vec::new();
            for _ in 0..rows {
                column.push(' ');
            }
            chars.push(column);
        }

        Self {
            columns,
            rows,
            drops,
            drop_speeds,
            trail_lengths,
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
            let trail_end = self.drops[i] - self.trail_lengths[i] as f32;

            if trail_end > self.rows as f32 + 5.0 {
                // Reset drop to off-screen position with new properties
                self.drops[i] = -(rng.random_range(0..self.rows) as f32);
                self.drop_speeds[i] = rng.random_range(0.5..1.5);
                self.trail_lengths[i] = rng.random_range(8..20);
            } else {
                // Move drop down at its speed
                self.drops[i] += self.drop_speeds[i];
            }
        }
    }

    fn update_grid(&mut self) {
        let mut rng = rand::rng();

        // Clear and update each column
        for col in 0..self.columns {
            let drop_pos = self.drops[col];
            let trail_length = self.trail_lengths[col];

            for row in 0..self.rows {
                let row_f = row as f32;

                // Head of drop - place new character
                if row_f >= drop_pos.floor() && row_f < drop_pos.floor() + 1.0 && drop_pos >= 0.0 {
                    self.chars[col][row] = self.get_random_symbol();
                }
                // Trail area - occasionally update characters
                else if row_f < drop_pos && row_f > drop_pos - trail_length as f32 {
                    // Occasionally change characters in the trail for that "glitchy" effect
                    if rng.random_bool(0.05) {
                        self.chars[col][row] = self.get_random_symbol();
                    }
                }
                // Outside trail - clear or occasionally show random faint character
                else if row_f > drop_pos || row_f < drop_pos - trail_length as f32 {
                    if rng.random_bool(0.01) {
                        self.chars[col][row] = self.get_random_symbol();
                    } else {
                        self.chars[col][row] = ' ';
                    }
                }
            }
        }
    }

    fn get_green_color(&self, position_in_trail: f32, trail_length: usize) -> u8 {
        // Use proper green colors from the 256-color palette
        // Colors 40-51 are various shades of green in the RGB cube
        let normalized_pos = (position_in_trail / trail_length as f32).clamp(0.0, 1.0);

        // Map to green colors: 46 (bright green) -> 40 (dark green) -> 22 (very dark)
        if normalized_pos < 0.3 {
            // Bright part of trail
            46 // Bright green
        } else if normalized_pos < 0.5 {
            40 // Medium-bright green
        } else if normalized_pos < 0.7 {
            34 // Medium green
        } else if normalized_pos < 0.85 {
            28 // Darker green
        } else {
            22 // Very dark green
        }
    }
}

impl Motion for MatrixMotion {
    fn update(&mut self) {
        // Get the latest terminal dimensions in case they changed
        if let Some((w, h)) = term_size::dimensions() {
            let new_width = w.saturating_sub(2);
            let new_height = h;

            // Only resize if dimensions changed
            if new_width != self.columns || new_height != self.rows {
                self.terminal_width = new_width;
                self.terminal_height = new_height;

                let old_columns = self.columns;
                self.columns = new_width;
                self.rows = new_height;

                // Resize/reinitialize arrays
                if self.drops.len() != self.columns {
                    self.drops.resize(self.columns, -1.0);
                    self.drop_speeds.resize(self.columns, 1.0);
                    self.trail_lengths.resize(self.columns, 10);
                }

                // Resize chars grid (column-major)
                if old_columns != self.columns {
                    let mut new_chars = Vec::new();
                    for _ in 0..self.columns {
                        let mut column = Vec::new();
                        for _ in 0..self.rows {
                            column.push(' ');
                        }
                        new_chars.push(column);
                    }
                    self.chars = new_chars;
                } else {
                    // Just resize each column
                    for col in 0..self.columns {
                        self.chars[col].resize(self.rows, ' ');
                    }
                }
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
                let char_to_display = self.chars[col][row];
                let drop_pos = self.drops[col];
                let trail_length = self.trail_lengths[col];
                let row_f = row as f32;

                // Determine color based on position relative to drop
                if row_f >= drop_pos.floor() && row_f < drop_pos.floor() + 1.0 && drop_pos >= 0.0 {
                    // Head of drop - bright white with slight green tint
                    print!("\x1B[38;5;231m\x1B[1m{}\x1B[0m", char_to_display);
                } else if row_f < drop_pos && row_f > drop_pos - trail_length as f32 {
                    // Trail - fading green
                    let distance_from_head = drop_pos - row_f;
                    let color = self.get_green_color(distance_from_head, trail_length);

                    // Add bold for brighter characters near the head
                    if distance_from_head < 3.0 {
                        print!("\x1B[38;5;{};1m{}\x1B[0m", color, char_to_display);
                    } else {
                        print!("\x1B[38;5;{}m{}", color, char_to_display);
                    }
                } else if char_to_display != ' ' {
                    // Random faint characters - very dark green
                    print!("\x1B[38;5;22m{}", char_to_display);
                } else {
                    // Empty space
                    print!(" ");
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
