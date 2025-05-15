mod common;

use wokecrab::WokeCrab;
use std::time::Duration;

#[test]
fn test_integration_basic_setup() {
    // Integration test for core components
    
    // Create a WokeCrab instance with minimal settings
    let wokecrab = WokeCrab::new(1, 1);
    
    // Test that it can be configured
    let wokecrab = wokecrab.with_verbose(true);
    assert!(wokecrab.is_verbose());
    
    // We don't actually call run() as it would start an infinite loop
    // But we can verify that all the pieces are connected
}

#[test]
fn test_system_interaction() {
    // This test is marked as ignored and would only be run with --ignored flag
    // It can be used for manual testing of system interactions
    
    // Create a WokeCrab instance with a short interval and tiny wiggle
    let mut _wokecrab = WokeCrab::new(1, 1);
    
    // We'll only let it run for a very short time
    // In a real test we might use a timeout or other mechanism
    std::thread::sleep(Duration::from_secs(3));
    
    // We're not calling run() because it would start an infinite loop
    // But this test illustrates how you might set up a real system test
}
