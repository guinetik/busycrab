#![cfg(target_os = "macos")]

//! # macOS Platform Implementation
//! 
//! This module provides a macOS-specific implementation for preventing system sleep
//! using the IOKit framework's `IOPMAssertionCreateWithName` function.
//!
//! For debugging, you can run `pmset -g assertions` in Terminal to see active assertions.

use core_foundation::string::CFString;
use core_foundation::base::TCFType;
use core_foundation_sys::base::CFTypeRef;
use std::sync::atomic::{AtomicU32, Ordering};

use super::PlatformTrait;

/// Type alias for the IOPMAssertionID, used to identify power management assertions.
type IOPMAssertionID = u32;

/// Return code for successful IOKit operations.
const K_IO_RETURN_SUCCESS: i32 = 0;

/// Constant for enabling a power management assertion.
const K_IOPMASSERTION_LEVEL_ON: u32 = 255;

/// Assertion type that prevents the display from sleeping due to user inactivity.
#[allow(non_upper_case_globals)]
const K_IOPM_ASSERTION_TYPE_PREVENT_USER_IDLE_DISPLAY_SLEEP: &str = "PreventUserIdleDisplaySleep";

/// Assertion type that prevents the system from sleeping due to user inactivity.
#[allow(non_upper_case_globals)]
const K_IOPM_ASSERTION_TYPE_PREVENT_USER_IDLE_SYSTEM_SLEEP: &str = "PreventUserIdleSystemSleep";

// Foreign function interface to the IOKit framework.
#[link(name = "IOKit", kind = "framework")]
extern "C" {
    /// Creates a power management assertion to prevent sleep or display dimming.
    fn IOPMAssertionCreateWithName(
        assertion_type: CFTypeRef,
        assertion_level: u32,
        assertion_name: CFTypeRef,
        assertion_id: *mut IOPMAssertionID,
    ) -> i32;
    
    /// Releases a previously created power management assertion.
    fn IOPMAssertionRelease(assertion_id: IOPMAssertionID) -> i32;
}

/// macOS-specific platform implementation.
/// 
/// Uses the IOKit framework's power management functions to prevent the system
/// and display from sleeping due to inactivity.
pub struct Platform {
    /// ID of the display sleep assertion (to prevent display from turning off)
    display_assertion_id: AtomicU32,
    
    /// ID of the system sleep assertion (to prevent system from sleeping)
    system_assertion_id: AtomicU32,
}

impl Platform {
    /// Creates a new Platform instance.
    /// 
    /// Initializes a Platform instance with empty assertion IDs.
    /// The actual assertions will be created in the `prevent_sleep` method.
    pub fn new() -> Self {
        Platform {
            display_assertion_id: AtomicU32::new(0),
            system_assertion_id: AtomicU32::new(0),
        }
    }
}

impl PlatformTrait for Platform {
    /// Prevents the system from sleeping or turning off the display.
    /// 
    /// This function creates two assertions using the IOKit framework:
    /// 1. PreventUserIdleDisplaySleep - Prevents the display from turning off
    /// 2. PreventUserIdleSystemSleep - Prevents the system from sleeping
    /// 
    /// ### Returns
    /// 
    /// - `Ok(())` if the assertions were created successfully.
    /// - `Err` with an error message if any assertion creation failed.
    fn prevent_sleep(&self) -> Result<(), &'static str> {
        let app_name = CFString::new("WokeCrab Rust App");
        
        // Create display sleep assertion if not already created
        if self.display_assertion_id.load(Ordering::Relaxed) == 0 {
            let assertion_type = CFString::from_static_string(K_IOPM_ASSERTION_TYPE_PREVENT_USER_IDLE_DISPLAY_SLEEP);
            let mut assertion_id: IOPMAssertionID = 0;
            
            let result = unsafe {
                IOPMAssertionCreateWithName(
                    assertion_type.as_concrete_TypeRef(),
                    K_IOPMASSERTION_LEVEL_ON,
                    app_name.as_concrete_TypeRef(),
                    &mut assertion_id,
                )
            };
            
            if result != K_IO_RETURN_SUCCESS {
                return Err("Failed to create display sleep assertion");
            }
            
            self.display_assertion_id.store(assertion_id, Ordering::Relaxed);
        }
        
        // Create system sleep assertion if not already created
        if self.system_assertion_id.load(Ordering::Relaxed) == 0 {
            let assertion_type = CFString::from_static_string(K_IOPM_ASSERTION_TYPE_PREVENT_USER_IDLE_SYSTEM_SLEEP);
            let mut assertion_id: IOPMAssertionID = 0;
            
            let result = unsafe {
                IOPMAssertionCreateWithName(
                    assertion_type.as_concrete_TypeRef(),
                    K_IOPMASSERTION_LEVEL_ON,
                    app_name.as_concrete_TypeRef(),
                    &mut assertion_id,
                )
            };
            
            if result != K_IO_RETURN_SUCCESS {
                // Release the display assertion if we created it
                let display_id = self.display_assertion_id.swap(0, Ordering::Relaxed);
                if display_id != 0 {
                    unsafe { IOPMAssertionRelease(display_id); }
                }
                
                return Err("Failed to create system sleep assertion");
            }
            
            self.system_assertion_id.store(assertion_id, Ordering::Relaxed);
        }
        
        Ok(())
    }
}

impl Drop for Platform {
    /// Releases any created power management assertions when the Platform instance is dropped.
    /// 
    /// This ensures that we clean up properly and don't leave assertions active after
    /// the program terminates.
    fn drop(&mut self) {
        // Release display sleep assertion
        let display_id = self.display_assertion_id.swap(0, Ordering::Relaxed);
        if display_id != 0 {
            unsafe { IOPMAssertionRelease(display_id); }
        }
        
        // Release system sleep assertion
        let system_id = self.system_assertion_id.swap(0, Ordering::Relaxed);
        if system_id != 0 {
            unsafe { IOPMAssertionRelease(system_id); }
        }
    }
}