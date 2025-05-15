//! # Platform Module
//! 
//! Platform-specific implementations for preventing system sleep.
//! 
//! This module uses conditional compilation to select the appropriate
//! implementation based on the target operating system:
//! 
//! - **Windows**: Uses `SetThreadExecutionState` from the Windows API
//! - **macOS**: Uses `IOPMAssertionCreateWithName` from the IOKit framework
//! - **Linux**: Currently not implemented (returns an error)
//! 
//! The module exposes a consistent `Platform` type with the same interface
//! regardless of the underlying platform, making the rest of the application
//! platform-agnostic.

/// Trait defining platform-specific operations.
/// 
/// This trait abstracts platform-specific functionality, allowing for
/// different implementations including real platform APIs and test mocks.
pub trait PlatformTrait {
    /// Prevents the system from sleeping or turning off the display.
    /// 
    /// ### Returns
    /// 
    /// - `Ok(())` if the operation was successful.
    /// - `Err` with an error message if the operation failed.
    fn prevent_sleep(&self) -> Result<(), &'static str>;
}

// Use cfg_if to select the appropriate implementation
cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
        pub use windows::Platform;
    } else if #[cfg(target_os = "macos")] {
        mod macos;
        pub use macos::Platform;
    } else {
        mod linux;
        pub use linux::Platform;
    }
} 