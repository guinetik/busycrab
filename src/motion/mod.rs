pub trait Motion: Send {
    fn update(&mut self);
}

pub mod crab;
pub mod matrix;
pub mod mandelbrot;
pub mod clock;
