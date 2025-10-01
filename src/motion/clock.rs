use super::Motion;
use std::io::{self, Write};
use chrono::{Local, Timelike};

pub struct ClockMotion {
    terminal_width: usize,
    terminal_height: usize,
    first_update: bool,
    frame_count: u32,
    
    // Clock parameters
    last_second: u64,
    cycling_chars: Vec<Vec<char>>,
}

impl ClockMotion {
    pub fn new() -> Self {
        let (width, height) = match term_size::dimensions() {
            Some((w, h)) => (w.saturating_sub(2), h),
            None => (78, 20),
        };

        // Define cycling characters for each digit (0-9)
        let cycling_chars = vec![
            // 0
            vec!['0', 'O', 'o', '°', '○', '◯', '◉', '●', '◆', '◇', '◈', '◊'],
            // 1
            vec!['1', 'I', 'l', '|', '│', '┃', '║', '╏', '╎', '┊', '┋', '┆'],
            // 2
            vec!['2', 'Z', 'z', 'Ƶ', 'ƶ', 'Ɀ', 'Ȿ', 'ɀ', 'Ɂ', 'ɂ', 'Ƀ', 'Ʉ'],
            // 3
            vec!['3', 'E', 'e', 'Ɛ', 'ε', 'ɛ', '∃', '∄', '∈', '∉', '∊', '∋'],
            // 4
            vec!['4', 'A', 'a', '∀', '∁', '∂', '∃', '∄', '∅', '∆', '∇', '∈'],
            // 5
            vec!['5', 'S', 's', '∫', '∬', '∭', '∮', '∯', '∰', '∱', '∲', '∳'],
            // 6
            vec!['6', 'G', 'g', 'Γ', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ'],
            // 7
            vec!['7', 'T', 't', 'τ', 'υ', 'φ', 'χ', 'ψ', 'ω', 'ϖ', 'ϗ', 'Ϙ'],
            // 8
            vec!['8', 'B', 'b', '∞', '∝', '∟', '∠', '∡', '∢', '∣', '∤', '∥'],
            // 9
            vec!['9', 'P', 'p', 'π', 'ϖ', 'ϗ', 'Ϙ', 'ϙ', 'Ϛ', 'ϛ', 'Ϝ', 'ϝ'],
        ];

        Self {
            terminal_width: width,
            terminal_height: height,
            first_update: true,
            frame_count: 0,
            last_second: 0,
            cycling_chars,
        }
    }

    fn get_current_time() -> (u8, u8, u8) {
        let now = Local::now();
        let hours = now.hour() as u8;
        let minutes = now.minute() as u8;
        let seconds = now.second() as u8;
        
        (hours, minutes, seconds)
    }

    fn get_cycling_char(&self, digit: u8, frame: u32) -> char {
        let chars = &self.cycling_chars[digit as usize];
        let index = (frame / 3) as usize % chars.len(); // Change every 3 frames
        chars[index]
    }

    fn draw_digit(&self, digit: u8, frame: u32) -> Vec<String> {
        let char_to_use = self.get_cycling_char(digit, frame);
        
        match digit {
            0 => vec![
                format!(" _____ "),
                format!("|     |"),
                format!("|  {}  |", char_to_use),
                format!("|     |"),
                format!("|_____|"),
            ],
            1 => vec![
                format!("   |   "),
                format!("   |   "),
                format!("   {}   ", char_to_use),
                format!("   |   "),
                format!("   |   "),
            ],
            2 => vec![
                format!(" _____ "),
                format!("      |"),
                format!(" _____|"),
                format!("|      "),
                format!("|_____ "),
            ],
            3 => vec![
                format!(" _____ "),
                format!("      |"),
                format!(" _____|"),
                format!("      |"),
                format!(" _____|"),
            ],
            4 => vec![
                format!("|     |"),
                format!("|     |"),
                format!("|_____|"),
                format!("      |"),
                format!("      |"),
            ],
            5 => vec![
                format!(" _____ "),
                format!("|      "),
                format!("|_____ "),
                format!("      |"),
                format!(" _____|"),
            ],
            6 => vec![
                format!(" _____ "),
                format!("|      "),
                format!("|_____ "),
                format!("|     |"),
                format!("|_____|"),
            ],
            7 => vec![
                format!(" _____ "),
                format!("      |"),
                format!("      |"),
                format!("      |"),
                format!("      |"),
            ],
            8 => vec![
                format!(" _____ "),
                format!("|     |"),
                format!("|_____|"),
                format!("|     |"),
                format!("|_____|"),
            ],
            9 => vec![
                format!(" _____ "),
                format!("|     |"),
                format!("|_____|"),
                format!("      |"),
                format!(" _____|"),
            ],
            _ => vec![
                format!("       "),
                format!("       "),
                format!("       "),
                format!("       "),
                format!("       "),
            ],
        }
    }

    fn draw_colon() -> Vec<String> {
        vec![
            format!("       "),
            format!("   |   "),
            format!("       "),
            format!("   |   "),
            format!("       "),
        ]
    }

    fn center_clock(&self, clock_lines: &[String]) -> Vec<String> {
        let max_width = clock_lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let start_col = (self.terminal_width.saturating_sub(max_width)) / 2;
        let start_row = (self.terminal_height.saturating_sub(clock_lines.len())) / 2;
        
        let mut centered_lines = Vec::new();
        
        // Add empty lines at the top
        for _ in 0..start_row {
            centered_lines.push(" ".repeat(self.terminal_width));
        }
        
        // Add clock lines with proper centering
        for line in clock_lines {
            let padding = " ".repeat(start_col);
            let full_line = format!("{}{}", padding, line);
            centered_lines.push(full_line);
        }
        
        // Fill remaining lines
        while centered_lines.len() < self.terminal_height {
            centered_lines.push(" ".repeat(self.terminal_width));
        }
        
        centered_lines
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
        
        // Only update if second changed (to avoid flickering)
        let current_second = seconds as u64;
        if current_second != self.last_second {
            self.last_second = current_second;
        }

        // Clear screen and move cursor to top
        print!("\x1B[2J\x1B[H");

        // Draw the clock
        let hour_tens = hours / 10;
        let hour_ones = hours % 10;
        let minute_tens = minutes / 10;
        let minute_ones = minutes % 10;
        let second_tens = seconds / 10;
        let second_ones = seconds % 10;

        let mut clock_lines = Vec::new();
        
        // Draw each digit and combine them
        let hour_tens_lines = self.draw_digit(hour_tens, self.frame_count);
        let hour_ones_lines = self.draw_digit(hour_ones, self.frame_count);
        let colon1_lines = Self::draw_colon();
        let minute_tens_lines = self.draw_digit(minute_tens, self.frame_count);
        let minute_ones_lines = self.draw_digit(minute_ones, self.frame_count);
        let colon2_lines = Self::draw_colon();
        let second_tens_lines = self.draw_digit(second_tens, self.frame_count);
        let second_ones_lines = self.draw_digit(second_ones, self.frame_count);

        // Combine all lines
        for i in 0..5 {
            let mut combined_line = String::new();
            combined_line.push_str(&hour_tens_lines[i]);
            combined_line.push_str(&hour_ones_lines[i]);
            combined_line.push_str(&colon1_lines[i]);
            combined_line.push_str(&minute_tens_lines[i]);
            combined_line.push_str(&minute_ones_lines[i]);
            combined_line.push_str(&colon2_lines[i]);
            combined_line.push_str(&second_tens_lines[i]);
            combined_line.push_str(&second_ones_lines[i]);
            clock_lines.push(combined_line);
        }

        // Center the clock on screen
        let centered_lines = self.center_clock(&clock_lines);

        // Print the clock in white
        for line in centered_lines.iter() {
            println!("\x1B[38;5;255m{}\x1B[0m", line);
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
