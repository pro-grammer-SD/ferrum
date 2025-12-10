# Ferrum

A small yet super-fast Ferrum prototype interpreter written in Rust. This is an experimental implementation of the Ferrum language featuring a runtime, REPL, and support for building scripts to bytecode.

## Features

- **Interpreted Execution**: Run `.fm` scripts directly
- **Interactive REPL**: Explore Ferrum code interactively with direct expression evaluation (>>>)
- **Bytecode Compilation**: Build scripts to serialized bytecode (prototype)
- **Object-Oriented Programming**: Class definitions and inheritance support
- **Built-in Functions**: String, integer, float, and boolean conversion utilities
- **Arithmetic Operations**: Full expression evaluation with +, -, *, / operators
- **GUI Development**: Iced GUI framework integration with:
  - Window, Button, Slider, RadioButton, Column, Row containers
  - Icon support for windows
  - Dynamic property updates (positions, sizes, values)
  - Interactive widgets with event callbacks
- **Computer Vision**: OpenCV integration for image processing:
  - Image loading, display, and saving
  - Gaussian blur and filters
  - Shape annotations (rectangles, circles, lines, text)
  - Face detection and body/hand landmark detection
  - Live camera capture and video processing
- **Code Analysis**: Static analysis and checking for scripts:
  - Syntax validation
  - Type checking (basic)
  - Unused variable detection
  - Deprecated function warnings
- **Help System**: Dynamic help for functions and modules
- **Standard Library Modules**:
  - **math**: Mathematical functions (sin, cos, sqrt, pow, exp, ln, log, etc.)
  - **json**: JSON serialization and deserialization
  - **os**: Operating system interactions (getcwd, listdir, platform)
  - **io**: Input/output operations (read_file, write_file)
  - **time**: Time and date utilities (time, sleep)
  - **random**: Random number generation (randint)
  - **sys**: System information
  - **iced**: GUI framework integration (with optional real Iced support)
  - **check**: Code analysis and validation
  - **opencv**: Computer vision (image processing, face detection, etc.)

## Installation

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/)) and ensure `cargo` is on PATH
- LLVM pre-built binaries installed and added to PATH (so `llvm-config` and `libclang.dll` can be found)
- Visual Studio with the **Desktop development with C++** workload installed (required for building some Rust crates on Windows)
- CMake installed and added to PATH (required for the Rust opencv crate to locate OpenCV)
- OpenCV for Windows installed and the `bin` folder added to PATH (so required DLLs like `opencv_world*.dll` can be found at runtime)
- Set the OpenCV_DIR environment variable pointing to the OpenCV CMake folder (e.g., C:\path\to\opencv\build\x64\vc16\lib) so the Rust opencv crate can locate the library during build

### Build

```bash
cargo build --release

The compiled binary will be in `target/release/ferrum`.

### Build with OpenCV Support as well as Iced Support

```bash
cargo build --release --features real-iced, opencv-support
```

## Usage

### Run a Script

```bash
ferrum run example.fm
```

### Start the REPL

```bash
ferrum repl
```

**REPL Features:**

- Direct expression evaluation: `>>> 1 + 2` returns `3`
- Statement execution: `>>> x = 5` creates variable
- Chained operations: `>>> x * 2 + 3` returns computed value
- Function calls: `>>> print("Hello")` executes statement

### Build to Bytecode

```bash
ferrum build example.fm
```

## Example Scripts

### Basic Arithmetic

```ferrum
x = 5
y = 10
result = x + y
print(result)  # Output: 15
```

### GUI Application

```ferrum
# Create window
w = Window()
w.set_title("My App")
w.set_size(600, 400)

# Add interactive controls
button = Button("Click Me!")
slider = Slider(0, 100)
radio = RadioButton("Option A")

w.add(button)
w.add(slider)
w.add(radio)

# Display the window
w.run()
```

### Image Processing

```ferrum
# Load and process image
img = cv_load_image("photo.jpg")

# Apply effects
blurred = cv_gaussian_blur(img, 5)

# Add annotations
cv_draw_rect(img, 100, 100, 200, 200, 255, 0, 0, 2)
cv_draw_circle(img, 150, 150, 50, 0, 255, 0, 2)
cv_draw_text(img, "Labeled Image", 10, 20, 255, 255, 255, 1.0)

# Detect features
faces = cv_detect_faces(img)
landmarks = cv_detect_body_landmarks(img)

# Save result
cv_save_image(img, "output.jpg")
```

### Code Analysis

```ferrum
code = "x = 5\ny = 10\nprint(y)"
result = check(code)
print(result)  # Shows analysis results
```

### Help System

```ferrum
# Get help on a function
help("print")
help("Slider")
help("cv_draw_rect")

# General help
help()
```

## GUI Framework Reference

### Window Class

```ferrum
w = Window()
w.set_title("Window Title")
w.set_position(x, y)
w.set_size(width, height)
w.set_icon("path/to/icon.png")
w.add(element)
w.run()
```

**Properties:**

- `title`: Window title string
- `x, y`: Position coordinates
- `width, height`: Window dimensions
- `icon`: Optional icon path (PNG)

### Button Class

```ferrum
b = Button("Label")
b.set_position(x, y)
b.set_label("New Label")
b.on_click("callback_function")
b.click()  # Simulate click
```

### Slider Class

```ferrum
s = Slider(min_val, max_val)
s.set_position(value)  # Set slider value
s.set_coordinates(x, y)  # Set UI position
s.get_value()  # Get current value
s.on_change("callback")  # Set change handler
```

### RadioButton Class

```ferrum
r = RadioButton("Option A")
r.set_position(x, y)
r.select()  # Select this option
r.deselect()  # Deselect
r.is_selected()  # Check state
r.get_state()  # Get (selected, label) tuple
```

### Column & Row Containers

```ferrum
c = Column()
c.set_position(x, y)
c.set_spacing(pixels)
c.add(child_element)

r = Row()
r.set_position(x, y)
r.set_spacing(pixels)
r.add(child_element)
```

## OpenCV Reference

### Image Operations

```ferrum
# Load image from disk
img = cv_load_image("path/to/image.png")

# Display in window
cv_display("window_name", img)

# Apply Gaussian blur
blurred = cv_gaussian_blur(img, kernel_size)

# Save processed image
cv_save_image(img, "output.png")
```

### Shape Annotations

```ferrum
# Draw rectangle: (image, x1, y1, x2, y2, r, g, b, thickness)
cv_draw_rect(img, 100, 100, 200, 200, 255, 0, 0, 2)

# Draw circle: (image, center_x, center_y, radius, r, g, b, thickness)
cv_draw_circle(img, 150, 150, 50, 0, 255, 0, 2)

# Draw line: (image, x1, y1, x2, y2, r, g, b, thickness)
cv_draw_line(img, 0, 0, 640, 480, 0, 0, 255, 1)

# Draw text: (image, text, x, y, r, g, b, font_scale)
cv_draw_text(img, "Label", 10, 20, 255, 255, 255, 1.0)
```

### Detection & Analysis

```ferrum
# Face detection
faces = cv_detect_faces(img)
# Returns: [(x, y, width, height), ...]

# Body landmark detection
landmarks = cv_detect_body_landmarks(img)
# Returns: [(x, y, "body_part_name"), ...]

# Hand keypoint detection
keypoints = cv_detect_hand_keypoints(img)
# Returns: [(x, y, "keypoint_name"), ...]

# Start webcam
camera = cv_start_camera()
```

## Code Analysis Features

The `check()` function provides static analysis:

```ferrum
result = check(source_code)
```

**Detects:**

- Syntax errors with error codes (E001, etc.)
- Undefined variables (W002)
- Deprecated function usage (W003)
- Unused variables (W001)
- Type mismatches (W004)

## REPL Enhancement

The REPL now supports direct expression evaluation:

```bash
>>> 1 + 2
3
>>> x = 5
>>> x * 2
10
>>> sin(1.57)
1.0000000000000002
>>> print("test")
test
```

Use `>>>` to get expression values directly, or use `print()` for explicit output.

## Architecture

- **main.rs**: CLI interface with subcommands (run, repl, build)
- **lib.rs**: Library interface and public API
- **parser.rs**: Lexing and parsing of Ferrum syntax
- **eval.rs**: Expression evaluation with arithmetic operators
- **runtime.rs**: Runtime environment and execution engine
- **repl.rs**: Interactive REPL with expression evaluation
- **ui.rs**: GUI and UI-related utilities and registry
- **stdlib/**: Standard library modules
  - `math.rs`: Mathematical functions
  - `jsonmod.rs`: JSON support
  - `osmod.rs`: OS interactions
  - `iomod.rs`: I/O operations
  - `timemod.rs`: Time utilities
  - `random.rs`: Random number generation
  - `sys.rs`: System information
  - `iced_stub.rs`: Iced GUI stub and real integration
  - `opencv.rs`: OpenCV computer vision bindings
  - `check.rs`: Code analysis and validation

## Features & Options

### Real Iced Integration

By default, the Iced GUI framework is enabled. To compile without real Iced support:

```bash
cargo build --release --no-default-features
```

### OpenCV Support (Optional)

Enable OpenCV computer vision features:

```bash
cargo build --release --features opencv-support
```

## Project Status

This is a prototype/experimental implementation. The language syntax, runtime behavior, and standard library are subject to change.

## Development

### Running Tests

```bash
cargo test
```

### Building Documentation

```bash
cargo doc --open
```

## Example Script

See `example.fm` for comprehensive examples including:

- Basic arithmetic and expressions
- Math operations
- Random numbers
- String/list operations
- File I/O
- Class definitions
- GUI creation with Iced
- OpenCV image processing
- Code analysis with check()
- Help system usage

## License

This project is provided as-is for educational and experimental purposes.
