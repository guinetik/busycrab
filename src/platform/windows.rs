//! # Windows Platform Implementation
//! 
//! This module provides a Windows-specific implementation for preventing system sleep
//! using the Windows API `SetThreadExecutionState` function.

use winapi::um::winbase::SetThreadExecutionState;
use winapi::um::winnt::{ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED};

use super::PlatformTrait;

/// Windows-specific platform implementation.
/// 
/// Uses the `SetThreadExecutionState` Windows API function to prevent the system
/// from entering sleep mode or turning off the display.
pub struct Platform;

impl Platform {
    /// Creates a new Platform instance.
    /// 
    /// This is a zero-sized struct for the Windows platform as no state needs to be maintained.
    pub fn new() -> Self {
        Platform
    }
}

impl PlatformTrait for Platform {
    /// Prevents the system from sleeping or turning off the display.
    /// 
    /// This function calls `SetThreadExecutionState` with the following flags:
    /// - `ES_CONTINUOUS`: The state should remain in effect until the next call.
    /// - `ES_SYSTEM_REQUIRED`: The system should be prevented from sleeping.
    /// - `ES_DISPLAY_REQUIRED`: The display should be prevented from turning off.
    /// 
    /// ### Returns
    /// 
    /// - `Ok(())` if the call was successful.
    /// - `Err` with an error message if the call failed.
    /// 
    /// ### Safety
    /// 
    /// This function makes an unsafe call to the Windows API. The `unsafe` block is contained
    /// within this function and doesn't leak to the rest of the application.
    fn prevent_sleep(&self) -> Result<(), &'static str> {
        let result = unsafe {
            SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED)
        };
        if result == 0 {
            Err("Failed to set execution state")
        } else {
            Ok(())
        }
    }
} 