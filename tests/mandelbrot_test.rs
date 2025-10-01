mod common;

use busycrab::motion::mandelbrot::MandelbrotMotion;
use busycrab::motion::Motion;

#[test]
fn test_mandelbrot_motion_creation() {
    // Test that MandelbrotMotion can be created
    let _motion = MandelbrotMotion::new();
    // Just verify it can be created without error
    assert!(true);
}

#[test]
fn test_mandelbrot_motion_update() {
    // Test that MandelbrotMotion can be updated without crashing
    let mut motion = MandelbrotMotion::new();
    
    // Run a few updates to make sure it doesn't crash
    for _ in 0..3 {
        motion.update();
    }
    
    // If we get here without panicking, the test passes
    assert!(true);
}
