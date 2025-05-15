use std::process;
use wokecrab::cli::Args;
use wokecrab::WokeCrab;
use clap::Parser;

pub fn main() {
    let args = Args::parse();
    
    if args.verbose {
        println!("Configuration:");
        println!("  Interval: {} seconds", args.interval);
        println!("  Wiggle distance: {} pixels", args.wiggle);
        println!("  Motion type: {}", args.motion);
    }
    
    let mut crab = WokeCrab::new(args.interval, args.wiggle)
        .with_verbose(args.verbose)
        .with_motion(&args.motion);
    
    if let Err(err) = crab.run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
