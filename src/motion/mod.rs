pub trait Motion: Send {
    fn update(&mut self);
}

pub mod crab;
