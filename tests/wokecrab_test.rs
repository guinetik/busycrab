mod common;

use wokecrab::WokeCrab;
use std::time::Duration;
use std::cell::RefCell;
use wokecrab::{MouseController, PlatformTrait};

/// A mock mouse controller for testing.
pub struct MockMouseController {
    /// Tracks the calls to mouse_move_relative
    pub move_calls: RefCell<Vec<(i32, i32)>>,
}

impl MockMouseController {
    /// Creates a new MockMouseController.
    pub fn new() -> Self {
        Self {
            move_calls: RefCell::new(Vec::new()),
        }
    }

    /// Gets the number of times mouse_move_relative was called.
    pub fn call_count(&self) -> usize {
        self.move_calls.borrow().len()
    }

    /// Gets the parameters of a specific call to mouse_move_relative.
    pub fn call_args(&self, index: usize) -> Option<(i32, i32)> {
        self.move_calls.borrow().get(index).cloned()
    }
}

impl MouseController for MockMouseController {
    fn mouse_move_relative(&mut self, x: i32, y: i32) {
        self.move_calls.borrow_mut().push((x, y));
    }
}

/// A mock platform implementation for testing.
pub struct MockPlatform {
    /// Whether prevent_sleep was called
    pub prevent_sleep_called: RefCell<bool>,
    /// The result to return from prevent_sleep
    pub prevent_sleep_result: RefCell<Result<(), &'static str>>,
}

impl MockPlatform {
    /// Creates a new MockPlatform.
    pub fn new() -> Self {
        Self {
            prevent_sleep_called: RefCell::new(false),
            prevent_sleep_result: RefCell::new(Ok(())),
        }
    }

    /// Creates a MockPlatform that returns an error.
    pub fn with_error(error: &'static str) -> Self {
        Self {
            prevent_sleep_called: RefCell::new(false),
            prevent_sleep_result: RefCell::new(Err(error)),
        }
    }
}

impl PlatformTrait for MockPlatform {
    fn prevent_sleep(&self) -> Result<(), &'static str> {
        *self.prevent_sleep_called.borrow_mut() = true;
        *self.prevent_sleep_result.borrow()
    }
}

#[test]
fn test_wokecrab_initialization() {
    // Test that WokeCrab can be created with valid parameters
    let wokecrab = WokeCrab::new(60, 3);
    assert_eq!(wokecrab.get_interval(), Duration::from_secs(60));
    assert_eq!(wokecrab.get_wiggle_distance(), 3);
    assert_eq!(wokecrab.is_verbose(), false);
    assert_eq!(wokecrab.has_motion(), false);
}

#[test]
fn test_wokecrab_builder_pattern() {
    // Test the builder pattern methods
    let wokecrab = WokeCrab::new(30, 5)
        .with_verbose(true)
        .with_motion("crab");
    
    assert_eq!(wokecrab.get_interval(), Duration::from_secs(30));
    assert_eq!(wokecrab.get_wiggle_distance(), 5);
    assert_eq!(wokecrab.is_verbose(), true);
    assert_eq!(wokecrab.has_motion(), true);
}

#[test]
fn test_motion_selection() {
    // Test that motion selection works correctly
    let crab_motion = WokeCrab::new(60, 3).with_motion("crab");
    assert_eq!(crab_motion.has_motion(), true);
    
    let no_motion = WokeCrab::new(60, 3).with_motion("none");
    assert_eq!(no_motion.has_motion(), false);
    
    // Invalid motion type should default to none
    let invalid_motion = WokeCrab::new(60, 3).with_motion("invalid");
    assert_eq!(invalid_motion.has_motion(), false);
}

// Basic test to demonstrate mocking
#[test]
fn test_simulate_activity() {
    let mock_mouse = Box::new(MockMouseController::new());
    let mock_ptr = &*mock_mouse as *const MockMouseController;
    
    let mut wokecrab = WokeCrab::with_mouse_controller(60, 5, mock_mouse);
    wokecrab.simulate_activity();
    
    // Safe because we know the mock is still alive inside wokecrab
    let mock = unsafe { &*mock_ptr };
    
    assert_eq!(mock.call_count(), 2);
    assert_eq!(mock.call_args(0), Some((5, 0)));
    assert_eq!(mock.call_args(1), Some((-5, 0)));
}

// Test the activity cycle with mocked components
#[test]
fn test_activity_cycle() {
    let mock_mouse = Box::new(MockMouseController::new());
    let mock_platform = Box::new(MockPlatform::new());
    
    // Store raw pointers to access the mocks later
    let mouse_ptr = &*mock_mouse as *const MockMouseController;
    let platform_ptr = &*mock_platform as *const MockPlatform;
    
    let mut wokecrab = WokeCrab::for_testing(60, 5, mock_mouse, mock_platform);
    let mut count = 0;
    let result = wokecrab.execute_activity_cycle(&mut count);
    
    // Safe because we know the mocks are still alive inside wokecrab
    let mouse = unsafe { &*mouse_ptr };
    let platform = unsafe { &*platform_ptr };
    
    assert!(result.is_ok());
    assert_eq!(count, 1);
    assert_eq!(mouse.call_count(), 2);
    assert!(*platform.prevent_sleep_called.borrow());
}

// Test handling of platform errors
#[test]
fn test_platform_error() {
    let mock_mouse = Box::new(MockMouseController::new());
    let mock_platform = Box::new(MockPlatform::with_error("Test error"));
    
    let mut wokecrab = WokeCrab::for_testing(60, 5, mock_mouse, mock_platform);
    let mut count = 0;
    let result = wokecrab.execute_activity_cycle(&mut count);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Test error");
    assert_eq!(count, 0); // Count should not be incremented on error
}
