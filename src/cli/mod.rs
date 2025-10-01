use clap::Parser;

/// 🦀 BusyCrab - A utility that prevents sleep and fakes activity to keep your status green
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Interval between mouse movements in seconds
    #[arg(short, long, default_value_t = 60)]
    pub interval: u64,

    /// Distance in pixels for mouse movement
    #[arg(short, long, default_value_t = 3)]
    pub wiggle: i32,

    /// Display additional information during operation
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
    
    /// Select motion animation type (crab, matrix, none)
    #[arg(short, long, default_value = "crab")]
    pub motion: String,
}
