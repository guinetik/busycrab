pub trait Motion: Send {
    fn update(&mut self);
}

pub mod clock;
pub mod crab;
pub mod mandelbrot;
pub mod matrix;
