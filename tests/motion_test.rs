mod common;

use busycrab::motion::crab::CrabMotion;
use busycrab::motion::Motion;

// Create a simple mock motion implementation for testing
struct MockMotion {
    update_count: std::cell::Cell<usize>,
}

impl MockMotion {
    fn new() -> Self {
        MockMotion {
            update_count: std::cell::Cell::new(0),
        }
    }
    
    fn update_count(&self) -> usize {
        self.update_count.get()
    }
}

impl Motion for MockMotion {
    fn update(&mut self) {
        let count = self.update_count.get();
        self.update_count.set(count + 1);
    }
}

#[test]
fn test_crab_motion_creation() {
    // Test that CrabMotion can be created
    let _motion = CrabMotion::new();
    // Just verify it can be created without error
    assert!(true);
}

#[test]
fn test_motion_trait_impl() {
    // Test that a Motion trait object can be created and used
    let mut mock = MockMotion::new();
    
    // Call update a few times
    mock.update();
    mock.update();
    mock.update();
    
    // Verify that the updates were counted
    assert_eq!(mock.update_count(), 3);
    
    // Test that we can use it through a trait object
    let mut motion_box: Box<dyn Motion> = Box::new(MockMotion::new());
    
    // This verifies we can call methods through the trait object
    motion_box.update();
}
