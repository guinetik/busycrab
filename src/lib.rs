//! # BusyCrab
//! A utility that prevents sleep and simulates mouse activity to keep your system active.
//! ## Features
//! * Prevents system sleep
//! * Simulates mouse movement
//! * Shows terminal animations
//! * Handles Ctrl+C for clean shutdown
//! ## Core components
//! * `BusyCrab`: Main application struct
//! * `platform`: Platform-specific functionality
//! * `motion`: Terminal animations

use enigo::{Enigo, MouseControllable};
use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

pub mod cli;
pub mod motion;
pub mod platform;

use motion::crab::CrabMotion;
use motion::matrix::MatrixMotion;
use motion::Motion;
pub use platform::Platform;
pub use platform::PlatformTrait;

/// Mouse control operations trait.
/// Abstracts mouse movement for real control and testing.
pub trait MouseController {
    /// Moves the mouse cursor relative to its current position.
    /// 
    /// * `x` - Horizontal movement in pixels
    /// * `y` - Vertical movement in pixels
    fn mouse_move_relative(&mut self, x: i32, y: i32);
}

/// Default mouse controller using Enigo.
pub struct DefaultMouseController {
    enigo: Enigo,
}

/// Default mouse controller implementation.
impl DefaultMouseController {
    /// Creates a new controller.
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(),
        }
    }
}

/// MouseController implementation.
impl MouseController for DefaultMouseController {
    fn mouse_move_relative(&mut self, x: i32, y: i32) {
        self.enigo.mouse_move_relative(x, y);
    }
}

/// Type alias for managing the animation thread and its control flag.
/// Contains:
/// - A `JoinHandle` for the thread running the animation.
/// - A shared boolean flag wrapped in `Arc<Mutex<bool>>` to signal thread termination.
/// ### What is `Arc`?
/// `Arc` stands for "Atomic Reference Counted". It is a thread-safe way to share ownership of a value across multiple threads.
/// When you clone an `Arc`, it increases the reference count, and the value is only dropped when all references are gone.
/// ### What is `Mutex`?
/// `Mutex` stands for "mutual exclusion". It provides safe, synchronized access to data from multiple threads.
/// Only one thread can lock and access the data inside the `Mutex` at a time, preventing data races.
/// ### Why is Rust so confusing?????
/// In this type alias, `Arc<Mutex<bool>>` is used so that both the main thread and the animation thread can safely share and update
/// a boolean flag (for example, to signal the animation thread to stop).
type AnimationThread = Option<(thread::JoinHandle<()>, Arc<Mutex<bool>>)>;

/// Main application struct.
pub struct BusyCrab {
    /// Mouse controller
    mouse: Box<dyn MouseController>,
    /// Platform implementation
    platform: Box<dyn PlatformTrait>,
    /// Time between activities
    interval: Duration,
    /// Mouse movement distance
    wiggle_distance: i32,
    /// Verbose mode flag
    verbose: bool,
    /// Optional animation
    motion: Option<Box<dyn Motion + Send>>,
}

/// BusyCrab implementation.
impl BusyCrab {
    /// Creates a new instance.
    ///
    /// * `interval_secs` - Seconds between mouse movements
    /// * `wiggle_distance` - Pixels to move the mouse
    pub fn new(interval_secs: u64, wiggle_distance: i32) -> Self {
        Self {
            mouse: Box::new(DefaultMouseController::new()),
            platform: Box::new(Platform::new()),
            interval: Duration::from_secs(interval_secs),
            wiggle_distance,
            verbose: false,
            motion: None,
        }
    }

    /// Creates an instance with custom mouse controller.
    /// Used for testing.
    pub fn with_mouse_controller(
        interval_secs: u64,
        wiggle_distance: i32,
        mouse_controller: Box<dyn MouseController>,
    ) -> Self {
        Self {
            mouse: mouse_controller,
            platform: Box::new(Platform::new()),
            interval: Duration::from_secs(interval_secs),
            wiggle_distance,
            verbose: false,
            motion: None,
        }
    }

    /// Creates a fully customizable instance for testing.
    pub fn for_testing(
        interval_secs: u64,
        wiggle_distance: i32,
        mouse_controller: Box<dyn MouseController>,
        platform: Box<dyn PlatformTrait>,
    ) -> Self {
        Self {
            mouse: mouse_controller,
            platform,
            interval: Duration::from_secs(interval_secs),
            wiggle_distance,
            verbose: false,
            motion: None,
        }
    }

    /// Starts the main application loop.
    pub fn run(&mut self) -> Result<(), &'static str> {
        self.display_startup_info();
        let animation_thread = self.start_animation_thread();
        let running = self.setup_shutdown_signal();
        self.run_activity_loop(running.clone())?;
        self.cleanup_resources(animation_thread);
        self.display_shutdown_message();
        Ok(())
    }

    /// Shows startup info.
    fn display_startup_info(&self) {
        println!("🦀 BusyCrab started. Press Ctrl+C to exit.");
        println!(
            "Running with interval: {} seconds, wiggle: {} pixels",
            self.interval.as_secs(),
            self.wiggle_distance
        );
    }

    /// Sets up Ctrl+C handler.
    fn setup_shutdown_signal(&self) -> Arc<AtomicBool> {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            println!("\n🦀 Caught Ctrl+C, shutting down gracefully...");
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl+C handler");

        running
    }

    /// Runs the main loop.
    fn run_activity_loop(&mut self, running: Arc<AtomicBool>) -> Result<(), &'static str> {
        let mut activity_count = 0;
        while running.load(Ordering::SeqCst) {
            self.execute_activity_cycle(&mut activity_count)?;
            if !self.wait_for_next_cycle(&running) {
                break;
            }
        }
        Ok(())
    }

    /// Executes one activity cycle.
    pub fn execute_activity_cycle(&mut self, activity_count: &mut u64) -> Result<(), &'static str> {
        self.platform.prevent_sleep()?;
        self.simulate_activity();
        
        *activity_count += 1;
        self.log_activity_status(*activity_count);
        
        Ok(())
    }

    /// Logs activity if verbose mode is on.
    fn log_activity_status(&self, activity_count: u64) {
        if self.verbose {
            print!("\r");
            io::stdout().flush().unwrap();
            println!(
                "Activity cycle #{} completed. Next update in {} seconds.",
                activity_count,
                self.interval.as_secs()
            );
        }
    }

    /// Waits until next cycle.
    fn wait_for_next_cycle(&self, running: &Arc<AtomicBool>) -> bool {
        let step_sleep = Duration::from_millis(200);
        let mut remaining = self.interval;
        while remaining > Duration::from_millis(0) && running.load(Ordering::SeqCst) {
            let sleep_time = if remaining < step_sleep {
                remaining
            } else {
                step_sleep
            };
            thread::sleep(sleep_time);
            remaining = remaining.saturating_sub(sleep_time);
        }
        running.load(Ordering::SeqCst)
    }

    /// Cleans up resources.
    fn cleanup_resources(&self, animation_thread: AnimationThread) {
        if let Some((handle, running_flag)) = animation_thread {
            if let Ok(mut flag) = running_flag.lock() {
                *flag = false;
            }

            if let Err(_) = handle.join() {
                if self.verbose {
                    println!("");
                    println!("Animation thread did not exit cleanly");
                }
            }
        }
    }

    /// Shows shutdown message.
    fn display_shutdown_message(&self) {
        println!("");
        println!("🦀 BusyCrab shut down successfully.");
    }

    /// Moves mouse slightly to simulate activity.
    pub fn simulate_activity(&mut self) {
        if self.verbose {
            print!("\r");
            io::stdout().flush().unwrap();
            println!("Moving mouse by {} pixels", self.wiggle_distance);
        }
        
        self.mouse.mouse_move_relative(self.wiggle_distance, 0);
        thread::sleep(Duration::from_millis(100));
        self.mouse.mouse_move_relative(-self.wiggle_distance, 0);
    }

    /// Starts animation thread if configured.
    fn start_animation_thread(&mut self) -> AnimationThread {
        if let Some(motion) = self.motion.take() {
            let running = Arc::new(Mutex::new(true));
            let running_clone = running.clone();

            let animation_thread = thread::spawn(move || {
                let animation_interval = Duration::from_millis(50);
                let mut motion = motion;
                while *running_clone.lock().unwrap() {
                    motion.update();
                    thread::sleep(animation_interval);
                }
            });

            Some((animation_thread, running))
        } else {
            None
        }
    }

    /// Sets verbose mode.
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Sets animation type.
    pub fn with_motion(mut self, motion_type: &str) -> Self {
        self.motion = match motion_type.to_lowercase().as_str() {
            "crab" => Some(Box::new(CrabMotion::new())),
            "matrix" => Some(Box::new(MatrixMotion::new())),
            "none" | _ => None,
        };
        self
    }

    /// Gets interval.
    pub fn get_interval(&self) -> Duration {
        self.interval
    }

    /// Gets wiggle distance.
    pub fn get_wiggle_distance(&self) -> i32 {
        self.wiggle_distance
    }

    /// Checks if verbose mode is on.
    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    /// Checks if motion is enabled.
    pub fn has_motion(&self) -> bool {
        self.motion.is_some()
    }
}


