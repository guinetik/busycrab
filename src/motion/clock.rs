use super::Motion;
use std::io::{self, Write};
use chrono::{Local, Timelike};

pub struct ClockMotion {
    terminal_width: usize,
    terminal_height: usize,
    first_update: bool,
    frame_count: u32,
    skew_offset: f64, // Horizontal offset per vertical line for 3D effect
}

impl ClockMotion {
    pub fn new() -> Self {
        let (width, height) = match term_size::dimensions() {
            Some((w, h)) => (w.saturating_sub(2), h),
            None => (78, 20),
        };

        Self {
            terminal_width: width,
            terminal_height: height,
            first_update: true,
            frame_count: 0,
            skew_offset: 0.4, // How much to skew each line
        }
    }

    fn get_current_time() -> (u8, u8, u8) {
        let now = Local::now();
        let hours = now.hour() as u8;
        let minutes = now.minute() as u8;
        let seconds = now.second() as u8;

        (hours, minutes, seconds)
    }

    // Get 7-segment pattern for a digit (5 rows x 3 columns of segments)
    // Returns a 2D grid where true = filled
    fn get_digit_pattern(digit: u8) -> Vec<Vec<bool>> {
        let patterns = match digit {
            0 => vec![
                vec![true,  true,  true ],  // top
                vec![true,  false, true ],  // upper middle
                vec![true,  false, true ],  // middle
                vec![true,  false, true ],  // lower middle
                vec![true,  true,  true ],  // bottom
            ],
            1 => vec![
                vec![false, false, true ],
                vec![false, false, true ],
                vec![false, false, true ],
                vec![false, false, true ],
                vec![false, false, true ],
            ],
            2 => vec![
                vec![true,  true,  true ],
                vec![false, false, true ],
                vec![true,  true,  true ],
                vec![true,  false, false],
                vec![true,  true,  true ],
            ],
            3 => vec![
                vec![true,  true,  true ],
                vec![false, false, true ],
                vec![true,  true,  true ],
                vec![false, false, true ],
                vec![true,  true,  true ],
            ],
            4 => vec![
                vec![true,  false, true ],
                vec![true,  false, true ],
                vec![true,  true,  true ],
                vec![false, false, true ],
                vec![false, false, true ],
            ],
            5 => vec![
                vec![true,  true,  true ],
                vec![true,  false, false],
                vec![true,  true,  true ],
                vec![false, false, true ],
                vec![true,  true,  true ],
            ],
            6 => vec![
                vec![true,  true,  true ],
                vec![true,  false, false],
                vec![true,  true,  true ],
                vec![true,  false, true ],
                vec![true,  true,  true ],
            ],
            7 => vec![
                vec![true,  true,  true ],
                vec![false, false, true ],
                vec![false, false, true ],
                vec![false, false, true ],
                vec![false, false, true ],
            ],
            8 => vec![
                vec![true,  true,  true ],
                vec![true,  false, true ],
                vec![true,  true,  true ],
                vec![true,  false, true ],
                vec![true,  true,  true ],
            ],
            9 => vec![
                vec![true,  true,  true ],
                vec![true,  false, true ],
                vec![true,  true,  true ],
                vec![false, false, true ],
                vec![true,  true,  true ],
            ],
            _ => vec![
                vec![false, false, false],
                vec![false, false, false],
                vec![false, false, false],
                vec![false, false, false],
                vec![false, false, false],
            ],
        };
        patterns
    }

    // Draw a digit with horizontal dot lines and skew
    fn render_digit_with_skew(&self, digit: u8, start_x: usize, start_y: usize, buffer: &mut Vec<Vec<char>>) {
        let pattern = Self::get_digit_pattern(digit);
        let digit_height = 15; // Height in terminal rows
        let digit_width = 12;  // Width in characters
        let segment_height = 3; // Each pattern row takes 3 terminal rows

        for (pattern_row, rows) in pattern.iter().enumerate() {
            for (pattern_col, &filled) in rows.iter().enumerate() {
                if !filled {
                    continue;
                }

                // Calculate segment position
                let seg_x = pattern_col * 4;
                let seg_y = pattern_row * segment_height;

                // Draw horizontal scanlines for this segment
                for line in 0..segment_height {
                    let y = start_y + seg_y + line;
                    if y >= buffer.len() {
                        continue;
                    }

                    // Apply skew: higher lines are more offset to the right
                    let skew = ((digit_height - (seg_y + line)) as f64 * self.skew_offset) as usize;

                    // Draw dots across the segment width
                    for dx in 0..4 {
                        let x = start_x + seg_x + dx + skew;
                        if x < buffer[y].len() && x < start_x + digit_width + digit_height {
                            buffer[y][x] = if dx % 2 == 0 { '●' } else { '•' };
                        }
                    }
                }
            }
        }
    }

    // Draw a colon separator
    fn render_colon_with_skew(&self, start_x: usize, start_y: usize, buffer: &mut Vec<Vec<char>>) {
        let digit_height = 15;
        let positions = [5, 10]; // Y positions for the two dots

        for &y_offset in &positions {
            let y = start_y + y_offset;
            if y >= buffer.len() {
                continue;
            }

            let skew = ((digit_height - y_offset) as f64 * self.skew_offset) as usize;

            for dx in 0..3 {
                let x = start_x + dx + skew;
                if x < buffer[y].len() {
                    buffer[y][x] = '●';
                }
            }
        }
    }
}

impl Motion for ClockMotion {
    fn update(&mut self) {
        // Get the latest terminal dimensions in case they changed
        if let Some((w, h)) = term_size::dimensions() {
            self.terminal_width = w.saturating_sub(2);
            self.terminal_height = h;
        }

        // Handle the first update specially
        if self.first_update {
            self.first_update = false;
            // Hide cursor and clear screen
            print!("\x1B[?25l\x1B[2J\x1B[H");
        }

        // Get current time
        let (hours, minutes, seconds) = Self::get_current_time();

        // Extract digits
        let h1 = hours / 10;
        let h2 = hours % 10;
        let m1 = minutes / 10;
        let m2 = minutes % 10;
        let s1 = seconds / 10;
        let s2 = seconds % 10;

        // Create buffer
        let mut buffer = vec![vec![' '; self.terminal_width]; self.terminal_height];

        // Calculate positions for digits
        let digit_width = 20;  // Includes skew space
        let colon_width = 8;
        let total_width = digit_width * 6 + colon_width * 2;
        let start_x = if self.terminal_width > total_width {
            (self.terminal_width - total_width) / 2
        } else {
            5
        };
        let start_y = if self.terminal_height > 20 {
            (self.terminal_height - 20) / 2
        } else {
            2
        };

        let mut x = start_x;

        // Render HH:MM:SS
        self.render_digit_with_skew(h1, x, start_y, &mut buffer);
        x += digit_width;

        self.render_digit_with_skew(h2, x, start_y, &mut buffer);
        x += digit_width;

        self.render_colon_with_skew(x, start_y, &mut buffer);
        x += colon_width;

        self.render_digit_with_skew(m1, x, start_y, &mut buffer);
        x += digit_width;

        self.render_digit_with_skew(m2, x, start_y, &mut buffer);
        x += digit_width;

        self.render_colon_with_skew(x, start_y, &mut buffer);
        x += colon_width;

        self.render_digit_with_skew(s1, x, start_y, &mut buffer);
        x += digit_width;

        self.render_digit_with_skew(s2, x, start_y, &mut buffer);

        // Clear screen and render
        print!("\x1B[2J\x1B[H");

        for row in &buffer {
            for &ch in row {
                match ch {
                    '●' => print!("\x1B[38;5;51m{}\x1B[0m", ch),  // Bright cyan
                    '•' => print!("\x1B[38;5;45m{}\x1B[0m", ch),  // Medium cyan
                    _ => print!("{}", ch),
                }
            }
            println!();
        }

        // Reset color and flush
        print!("\x1B[0m");
        let _ = io::stdout().flush();

        self.frame_count += 1;
    }
}

impl Drop for ClockMotion {
    fn drop(&mut self) {
        // Show cursor and reset color when the program exits
        print!("\x1B[?25h\x1B[0m");
        let _ = io::stdout().flush();
    }
}
