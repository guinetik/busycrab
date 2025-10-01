use super::Motion;
use std::io::Write;

pub struct MandelbrotMotion {
    terminal_width: usize,
    terminal_height: usize,
    first_update: bool,
    frame_count: u32,
    
    // Mandelbrot parameters
    center_x: f64,
    center_y: f64,
    zoom: f64,
    max_iterations: u32,
    
    // Animation parameters
    rotation_angle: f64,
    color_shift: f32,
    
    // Breathing effect parameters
    breathing_cycle: f64, // Track breathing cycle
    base_zoom: f64, // Base zoom level for breathing
    
    // Fractal waypoints for interesting zoom targets
    waypoint_index: usize,
    waypoints: Vec<(f64, f64, f64)>, // (x, y, zoom_level)
}

impl MandelbrotMotion {
    pub fn new() -> Self {
        let (width, height) = match term_size::dimensions() {
            Some((w, h)) => (w.saturating_sub(2), h), // Use full height
            None => (78, 20),
        };

        // Predefined interesting fractal coordinates to zoom into
        let waypoints = vec![
            // Classic Mandelbrot set
            (-0.5, 0.0, 1.0),
            // Seahorse valley - beautiful spiral structures
            (-0.75, 0.1, 10.0),
            // Elephant valley - intricate details
            (0.3, 0.0, 50.0),
            // Mini-Mandelbrot - self-similar structures
            (-0.1592, 1.0317, 100.0),
            // Spiral structures
            (-0.8, 0.156, 200.0),
            // More intricate details
            (0.0, 0.8, 500.0),
            // Deep zoom into interesting area
            (-0.235125, 0.827215, 1000.0),
            // Another beautiful spiral
            (-0.7269, 0.1889, 2000.0),
        ];

        Self {
            terminal_width: width,
            terminal_height: height,
            first_update: true,
            frame_count: 0,
            
            // Start with first waypoint
            center_x: waypoints[0].0,
            center_y: waypoints[0].1,
            zoom: waypoints[0].2,
            max_iterations: 100, // Increased for better detail
            
            // Animation parameters
            rotation_angle: 0.0,
            color_shift: 0.0,
            
            // Breathing effect parameters
            breathing_cycle: 0.0,
            base_zoom: waypoints[0].2,
            
            // Waypoint system
            waypoint_index: 0,
            waypoints,
        }
    }

    fn mandelbrot_iterations(&self, x: f64, y: f64) -> u32 {
        let mut zx = 0.0;
        let mut zy = 0.0;
        let mut iterations = 0;
        
        while iterations < self.max_iterations && (zx * zx + zy * zy) < 4.0 {
            let xtemp = zx * zx - zy * zy + x;
            zy = 2.0 * zx * zy + y;
            zx = xtemp;
            iterations += 1;
        }
        
        iterations
    }

    fn get_color(&self, iterations: u32, _row: usize, _col: usize) -> u8 {
        if iterations == self.max_iterations {
            // Inside the set - pure black
            return 0;
        }
        
        // Create beautiful color mapping
        let normalized = (iterations as f32 + self.color_shift) / self.max_iterations as f32;
        let color_index = (normalized * 255.0) as u8;
        
        // Map to terminal color range (16-231 for 256-color palette)
        // Use a cycling pattern for smooth color transitions
        let cycle = (color_index as f32 * 0.1 + self.color_shift) % 1.0;
        let color_palette = [
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
            48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
            64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
            80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
            96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
            112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
            128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
            144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
            160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175,
            176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191,
            192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207,
            208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223,
            224, 225, 226, 227, 228, 229, 230, 231
        ];
        
        let index = (cycle * (color_palette.len() - 1) as f32) as usize;
        color_palette[index.min(color_palette.len() - 1)]
    }

    fn get_character(&self, iterations: u32, _row: usize, _col: usize) -> char {
        if iterations == self.max_iterations {
            // Inside the set - use solid character
            return '█';
        }
        
        // Use more detailed character mapping for better fractal visualization
        let chars = [' ', '.', ':', ';', '+', '*', 'x', 'X', '#', '█'];
        
        // Map iterations to character index with better distribution
        let normalized = iterations as f32 / self.max_iterations as f32;
        let index = (normalized * (chars.len() - 1) as f32) as usize;
        chars[index.min(chars.len() - 1)]
    }


    fn update_animation(&mut self) {
        // Breathing cycle - zoom in and out
        self.breathing_cycle += 0.01;
        
        // Create breathing effect using sine wave
        let breathing_factor = (self.breathing_cycle).sin();
        
        // Map sine wave (-1 to 1) to zoom range (0.5 to 2.0)
        // This ensures we always zoom in and out properly
        let zoom_multiplier = 0.5 + (breathing_factor + 1.0) * 0.75; // Maps to 0.5 to 2.0
        self.zoom = self.base_zoom * zoom_multiplier;
        
        // Add slight rotation for dynamic effect
        self.rotation_angle += 0.005;
        
        // Shift colors for rainbow effect
        self.color_shift += 0.2;
        
        // Cycle through waypoints every few breathing cycles
        let should_cycle = self.breathing_cycle > std::f64::consts::PI * 6.0; // After 3 full breathing cycles
        
        if should_cycle {
            self.breathing_cycle = 0.0;
            self.waypoint_index = (self.waypoint_index + 1) % self.waypoints.len();
            let waypoint = self.waypoints[self.waypoint_index];
            
            // Smooth transition to new waypoint
            self.center_x = waypoint.0;
            self.center_y = waypoint.1;
            self.base_zoom = waypoint.2;
            
            // Add some randomness to make it more interesting
            self.center_x += (self.frame_count as f64 * 0.001).sin() * 0.005;
            self.center_y += (self.frame_count as f64 * 0.001).cos() * 0.005;
        }
    }
}

impl Motion for MandelbrotMotion {
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

        // Update animation parameters
        self.update_animation();

        // Clear screen and move cursor to top
        print!("\x1B[2J\x1B[H");

        // Calculate the scale based on zoom and terminal size
        let scale = 4.0 / (self.zoom * self.terminal_width as f64);
        let aspect_ratio = self.terminal_width as f64 / self.terminal_height as f64;

        // Render the Mandelbrot set
        for row in 0..self.terminal_height {
            for col in 0..self.terminal_width {
                // Convert screen coordinates to complex plane coordinates
                let x = self.center_x + (col as f64 - self.terminal_width as f64 / 2.0) * scale;
                let y = self.center_y + (row as f64 - self.terminal_height as f64 / 2.0) * scale * aspect_ratio;
                
                // Apply rotation for dynamic effect
                let cos_angle = self.rotation_angle.cos();
                let sin_angle = self.rotation_angle.sin();
                let rotated_x = x * cos_angle - y * sin_angle;
                let rotated_y = x * sin_angle + y * cos_angle;
                
                // Calculate Mandelbrot iterations
                let iterations = self.mandelbrot_iterations(rotated_x, rotated_y);
                
                // Get color and character
                let color = self.get_color(iterations, row, col);
                let character = self.get_character(iterations, row, col);
                
                // Print with color
                if color == 0 {
                    // Black for inside the set
                    print!("\x1B[30m{}", character);
                } else {
                    // Colorful for outside the set
                    print!("\x1B[38;5;{}m{}", color, character);
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

impl Drop for MandelbrotMotion {
    fn drop(&mut self) {
        // Show cursor and reset color when the program exits
        print!("\x1B[?25h\x1B[0m");
        let _ = std::io::stdout().flush();
    }
}
