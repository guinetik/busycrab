//! # Linux Platform Implementation
//! 
//! This module provides a placeholder implementation for Linux systems.
//! Currently, sleep prevention is not implemented for Linux.

use super::PlatformTrait;

/// Linux platform implementation (placeholder).
/// 
/// This is a placeholder implementation that does not actually prevent sleep on Linux.
pub struct Platform;

impl Platform {
    /// Creates a new Platform instance.
    /// 
    /// This is a zero-sized struct since no functionality is implemented yet.
    pub fn new() -> Self {
        Platform
    }
}

impl PlatformTrait for Platform {
    /// Placeholder function that returns an error since sleep prevention is not implemented for Linux.
    /// 
    /// ### Returns
    /// 
    /// Always returns `Err` with a message indicating that the functionality is not implemented.
    /// 
    /// ### Future Implementation
    /// 
    /// In the future, this could be implemented using systemd inhibit or other Linux-specific methods.
    fn prevent_sleep(&self) -> Result<(), &'static str> {
        // TODO research systemd inhibit method
        Err("Sleep prevention not implemented for this OS. I will work on this when I have access to a Linux machine.")
    }
} 