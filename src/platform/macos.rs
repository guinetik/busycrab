#![cfg(target_os = "macos")]

//! # macOS Platform Implementation (Stub)
//! 
//! This module provides a stub implementation for macOS.
//! The actual implementation would use IOKit framework's power management functions.

use super::PlatformTrait;

/// macOS-specific platform stub implementation.
pub struct Platform {}

impl Platform {
    /// Creates a new Platform instance.
    pub fn new() -> Self {
        Platform {}
    }
}

impl PlatformTrait for Platform {
    /// Stub implementation that just logs the call.
    /// 
    /// ### Returns
    /// 
    /// - Always returns `Ok(())`.
    fn prevent_sleep(&self) -> Result<(), &'static str> {
        println!("[STUB] macOS prevent_sleep called - not implemented");
        Ok(())
    }
}

impl Drop for Platform {
    fn drop(&mut self) {
        println!("[STUB] macOS platform dropped");
    }
}