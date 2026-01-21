## Brief overview

Project-specific guidelines for maintaining comprehensive test coverage in the rust-calculator codebase.

## Testing requirements

- For every source code change, add corresponding unit tests to ensure the new or modified functionality is covered
- Tests should be placed in the appropriate test files within the `tests/` directory following the existing naming convention (e.g., `calculator_tests.rs` for `calculator.rs`)

## Coverage goals

- Maintain overall line coverage above 80% across the entire codebase
- Use tarpaulin for generating coverage reports and monitoring progress
- Review coverage reports regularly to identify uncovered areas and prioritize test additions
