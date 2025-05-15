# WokeCrab Usage Examples

WokeCrab is a utility that prevents sleep and fakes activity to keep your status green. It does this by:
1. Preventing system sleep using platform-specific APIs
2. Moving your mouse cursor slightly at regular intervals
3. Displaying delightful motion animations in your console

## Basic Usage

Run with default settings (60-second interval, 3-pixel mouse movement, with crab animation):
```
wokecrab
```

## Exiting the Program

You can gracefully exit WokeCrab at any time by pressing Ctrl+C. The program will clean up resources and exit properly.

## Customizing Behavior

### Change the interval between mouse movements:
```
wokecrab --interval 120    # Move mouse every 2 minutes
wokecrab -i 30             # Move mouse every 30 seconds
```

### Change the distance the mouse moves:
```
wokecrab --wiggle 5        # Move mouse 5 pixels
wokecrab -w 10             # Move mouse 10 pixels
```

### Select a motion animation type:
```
wokecrab --motion crab     # Show crab animation (default)
wokecrab -m none           # Disable animations
```

### Display verbose logging:
```
wokecrab --verbose         # Show detailed activity logs
wokecrab -v                # Shorter form for verbose flag
```

### Combine multiple options:
```
wokecrab -i 45 -w 2 -v     # 45-second interval, 2-pixel wiggle, verbose
wokecrab -i 10 -w 5 -m none -v   # 10-second interval, 5-pixel wiggle, no animation, verbose
```

## Help and Version Information

```
wokecrab --help            # Display help with all available options
wokecrab --version         # Display version information
```

## Running in the Background

To run WokeCrab in the background on Windows, you can use:
```
start /min wokecrab
```

To run in the background and close the console window:
```
start /b wokecrab
```

## Building from Source

```
cargo build --release
```

This will create the executable at `target/release/wokecrab.exe`
