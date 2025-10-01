# 🦀 BusyCrab

**BusyCrab** is a lightweight Rust utility that prevents your computer from sleeping and simulates subtle mouse activity to keep your status green on apps like Microsoft Teams or Slack.

## About This Project

This is my learning project for exploring Rust's features and ecosystem. I was looking for a use-case for a systems language and I created BusyCrab to experiment with several modern Rust concepts:

- **Multi-threading and concurrency** - Managing separate threads with safe communication
- **Cross-platform development** - Supporting Windows and macOS with conditional compilation
- **Trait-based polymorphism** - Using traits to create extensible systems
- **Builder pattern** - Creating a fluent API with method chaining
- **Error handling** - Using Result types and proper error propagation
- **Dynamic dispatch** - Working with trait objects (`Box<dyn Trait>`)

## Key Features

- Keeps your machine awake with platform-specific APIs  
- Jiggles your mouse cursor by a tiny amount every interval  
- Helps you avoid being marked "idle" or "away" without annoying interruptions  
- Fully customizable with command-line options
- Features fun terminal animations with an extensible motion system
- Gracefully shuts down with Ctrl+C

## Technical Implementation

### Crates Used

- **enigo** - Cross-platform library for simulating mouse and keyboard input
- **clap** - Command-line argument parser with derive macros
- **ctrlc** - Library for handling Ctrl+C and other termination signals
- **term_size** - Small utility for getting terminal dimensions
- **cfg-if** - Helper macro for conditional compilation based on target platform
- **winapi** - Windows API bindings (used for Windows-specific functionality)

### Concurrent Animation System

One of the main learning goals was to understand Rust's threading model. The application runs two separate threads:

1. **Main thread** - Handles mouse movement, system sleep prevention, and program flow
2. **Animation thread** - Updates terminal animations independently at a higher framerate

The animation thread serves as both a visual indicator that the program is running (like a screensaver) and a practical example of thread synchronization in Rust. It demonstrates:

- Using `Arc<Mutex<bool>>` for thread communication
- Safe thread termination with join handles
- Taking ownership across thread boundaries
- Proper cleanup during program shutdown

This design allows the animation to run smoothly without being tied to the mouse movement interval, while also providing a real-world use case for concurrency.

## Installation

```bash
cargo install busycrab
```

## Basic Usage

Run with default settings (moves mouse by 3 pixels every 60 seconds):

```bash
busycrab
```

## Customization

Adjust the interval and movement distance:

```bash
busycrab --interval 120 --wiggle 5  # 2-minute interval, 5-pixel movement
busycrab -i 30 -w 2                 # 30-second interval, 2-pixel movement
```

Select different motion animations:

```bash
busycrab --motion crab              # Default crab animation
busycrab --motion matrix            # Matrix-style falling characters
busycrab -m none                    # No animation
```

See [USAGE.md](USAGE.md) for more examples and options.

## Testing

The project includes a test suite in the `tests/` directory. To run the tests:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run ignored tests (those that interact with the system)
cargo test -- --ignored

# Run a specific test
cargo test test_busycrab_initialization

# Coverage Support
cargo llvm-cov --html
```

## Release Process

BusyCrab uses GitHub Actions for automated releases. The workflow is as follows:

1. Update the version number in `Cargo.toml` when you're ready to release
2. Push your changes to the `master` branch
3. The CI/CD pipeline will automatically:
   - Build the project for Windows, macOS, and Linux
   - Create a new tag based on the version in `Cargo.toml`
   - Generate a GitHub release with the compiled binaries

You can find all releases in the [GitHub Releases](https://github.com/guinetik/busycrab/releases) section.

## Contributing

Pull requests are welcome! This is a learning project, so I'm open to suggestions for:

- New motion animations
- Additional platform support
- Code improvements and Rust best practices
- Performance optimizations
