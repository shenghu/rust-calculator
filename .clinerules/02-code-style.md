## Brief overview

Project-specific code style guidelines for the rust-calculator codebase, derived from existing code patterns and Rust best practices for consistent, readable, and maintainable code.

## Documentation standards

- Use triple slash `///` comments for all public APIs (structs, enums, functions, methods)
- Include parameter and return value descriptions in doc comments
- Add code examples in doc comments for complex public functions using triple backticks
- Use single line `//` comments for implementation details and TODOs
- Document struct fields with `///` when they are part of the public API

## Naming conventions

- Use PascalCase for types (structs, enums, traits): `Calculator`, `Operation`, `CalculatorUIState`
- Use snake_case for functions, methods, and variables: `evaluate()`, `process_message()`, `display_string`
- Use UPPER_SNAKE_CASE for constants and statics: `DISPLAY_SCROLL_ID`
- Use descriptive names that clearly indicate purpose and type

## Module organization

- Group related functionality into separate modules: `calculator`, `display`, `input`, `ui`
- Use `pub mod` declarations in lib.rs for module exposure
- Re-export main types in lib.rs with `pub use` for convenient importing
- Keep module files focused on single responsibilities

## Import style

- Group imports by source: standard library, external crates, internal modules
- Use selective imports for external crates: `use iced::widget::{button, column, container}`
- Use full paths for internal imports: `use crate::calculator::{Calculator, Operation}`
- Avoid wildcard imports (`use crate::*`) except for prelude modules

## Struct and enum patterns

- Derive common traits explicitly: `#[derive(Default, Debug, Clone)]` for structs
- Use `#[derive(Debug, Clone, Copy, PartialEq)]` for enums with copy semantics
- Implement `Default` trait manually when initialization logic is complex
- Use struct field visibility appropriately: `pub` for public APIs, private for internal state

## Function design

- Keep functions focused and under 50 lines when possible
- Use descriptive parameter names that indicate purpose
- Return `Result<T, String>` for operations that can fail
- Implement comprehensive error handling with meaningful error messages
- Use early returns with `?` operator for error propagation

## Code formatting

- Use consistent indentation and spacing throughout the codebase
- Align related code elements for readability (match arms, struct fields)
- Break long lines appropriately, especially for method chains
- Use trailing commas in multi-line structures for easier diffing

## Testing patterns

- Place unit tests in corresponding test files within the `tests/` directory
- Use descriptive test function names: `calculator_tests.rs`, `ui_tests.rs`
- Include examples in doc comments that serve as executable tests
- Test both success and error cases comprehensively
