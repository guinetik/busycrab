# BusyCrab Usage Examples

BusyCrab is a utility that prevents sleep and fakes activity to keep your status green. It does this by:
1. Preventing system sleep using platform-specific APIs
2. Moving your mouse cursor slightly at regular intervals
3. Displaying delightful motion animations in your console

## Basic Usage

Run with default settings (60-second interval, 3-pixel mouse movement, with crab animation):
```
busycrab
```

## Exiting the Program

You can gracefully exit BusyCrab at any time by pressing Ctrl+C. The program will clean up resources and exit properly.

## Customizing Behavior

### Change the interval between mouse movements:
```
busycrab --interval 120    # Move mouse every 2 minutes
busycrab -i 30             # Move mouse every 30 seconds
```

### Change the distance the mouse moves:
```
busycrab --wiggle 5        # Move mouse 5 pixels
busycrab -w 10             # Move mouse 10 pixels
```

### Select a motion animation type:
```
busycrab --motion crab     # Show crab animation (default)
busycrab --motion matrix   # Show Matrix-style falling characters
busycrab -m none           # Disable animations
```

### Display verbose logging:
```
busycrab --verbose         # Show detailed activity logs
busycrab -v                # Shorter form for verbose flag
```

### Combine multiple options:
```
busycrab -i 45 -w 2 -v     # 45-second interval, 2-pixel wiggle, verbose
busycrab -i 10 -w 5 -m none -v   # 10-second interval, 5-pixel wiggle, no animation, verbose
```

## Help and Version Information

```
busycrab --help            # Display help with all available options
busycrab --version         # Display version information
```

## Running in the Background

To run BusyCrab in the background on Windows, you can use:
```
start /min busycrab
```

To run in the background and close the console window:
```
start /b busycrab
```

## Building from Source

```
cargo build --release
```

This will create the executable at `target/release/busycrab.exe`
