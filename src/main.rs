use std::process;
use busycrab::cli::Args;
use busycrab::BusyCrab;
use clap::Parser;

pub fn main() {
    let args = Args::parse();
    
    if args.verbose {
        println!("Configuration:");
        println!("  Interval: {} seconds", args.interval);
        println!("  Wiggle distance: {} pixels", args.wiggle);
        println!("  Motion type: {}", args.motion);
    }
    
    let mut crab = BusyCrab::new(args.interval, args.wiggle)
        .with_verbose(args.verbose)
        .with_motion(&args.motion);
    
    if let Err(err) = crab.run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
