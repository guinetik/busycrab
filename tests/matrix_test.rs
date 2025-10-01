mod common;

use busycrab::motion::matrix::MatrixMotion;
use busycrab::motion::Motion;

#[test]
fn test_matrix_motion_creation() {
    // Test that MatrixMotion can be created
    let _motion = MatrixMotion::new();
    // Just verify it can be created without error
    assert!(true);
}

#[test]
fn test_matrix_motion_update() {
    // Test that MatrixMotion can be updated without crashing
    let mut motion = MatrixMotion::new();
    
    // Run a few updates to make sure it doesn't crash
    for _ in 0..5 {
        motion.update();
    }
    
    // If we get here without panicking, the test passes
    assert!(true);
}
