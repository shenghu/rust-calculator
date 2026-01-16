# Rust Calculator

A simple calculator application built with Rust, featuring both command-line and GUI interfaces. Supports basic arithmetic operations with proper operator precedence and expression evaluation.

This project was created as a practice exercise in "vibe-coding" - an AI-assisted development approach using Cline (a VS Code extension) with the grok-code-fast-1 model to rapidly prototype and build functional applications through conversational interaction with AI.

## Features

- **Expression Evaluation**: Supports complex mathematical expressions with proper operator precedence
- **Basic Operations**: Addition, subtraction, multiplication, division
- **Decimal Support**: Handle floating-point calculations
- **GUI Interface**: Built with Iced framework for a modern, responsive user interface
- **Library Crate**: Reusable calculator logic that can be integrated into other Rust projects
- **Unit Tests**: Comprehensive test coverage for both library and UI components

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd rust-calculator

# Build the project
cargo build --release

# Run the GUI application
cargo run

# Run tests
cargo test
```

### Using as a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-calculator = "0.1.0"
```

Then use it in your code:

```rust
use rust_calculator::{Calculator, Operation};

let mut calc = Calculator::new();
let result = calc.calculate(Operation::Add, 5.0, 3.0);
```

## Usage

### GUI Mode (Default)

Simply run the application:

```bash
cargo run
```

The GUI provides an intuitive calculator interface with buttons for numbers, operations, and functions.

### Library Usage

```rust
use rust_calculator::Calculator;

let calc = Calculator::new();

// Evaluate expressions
let result = calc.evaluate("7+8*3").unwrap(); // Returns 31.0

// Direct calculations
let sum = calc.calculate(Operation::Add, 10.0, 5.0).unwrap(); // Returns 15.0
```

## Project Structure

```
src/
├── lib.rs           # Library crate definition and public API
├── main.rs          # Binary crate entry point (GUI application)
├── calculator.rs    # Core calculator logic and expression evaluation
├── display.rs       # Display formatting utilities
├── input.rs         # Input handling and validation
└── ui.rs            # GUI state management and message handling

tests/
├── calculator_tests.rs  # Unit tests for calculator logic
├── display_tests.rs     # Display formatting tests
└── input_tests.rs       # Input handling tests
```

## API Documentation

The library provides the following main components:

- `Calculator`: Core calculator struct with expression evaluation
- `Operation`: Enum representing mathematical operations
- `CalculatorUIState`: GUI state management for the iced interface
- `UIMessage`: Message types for GUI interactions

Run `cargo doc --open` to generate and view the full API documentation.

## Testing

Run the test suite:

```bash
cargo test
```

For test coverage (requires tarpaulin):

```bash
cargo tarpaulin --out Html
```

Then open `coverage/tarpaulin-report.html` in your browser.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Setup

```bash
# Install development dependencies
cargo install cargo-tarpaulin

# Run in development mode with hot reloading
cargo watch -x run
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Iced](https://iced.rs/) GUI framework
- Inspired by classic calculator applications
- Thanks to the Rust community for excellent documentation and tools

## Vibe Coding Experience

This project showcases the "vibe coding" approach - rapid AI-assisted development through conversational interaction. Here are key insights from the development process:

1. **Rapid Prototyping**: It was remarkably quick to build a functional calculator with basic math operations. The AI selected Iced 0.12.0 as the GUI framework and efficiently implemented the core calculator logic, expression evaluation, and user interface.

2. **API Migration Challenges**: When upgrading from Iced 0.12.0 to 0.14.0, the AI encountered significant API breaking changes and struggled to adapt the window sizing logic to maintain the compact calculator layout, highlighting the challenges of keeping up with rapidly evolving frameworks.

3. **Human-AI Collaboration**: During the upgrade to Iced 0.13.1, the AI initially failed to implement horizontal scrollbar hiding and auto-scrolling to the rightmost position. This was successfully resolved through specific human-provided instructions, demonstrating the value of targeted guidance in complex UI interactions.

## Roadmap

- [ ] Scientific calculator functions (sin, cos, tan, log, etc.)
- [ ] History and memory functions
- [ ] Keyboard shortcuts and accessibility improvements
