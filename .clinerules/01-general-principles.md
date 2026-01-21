## Brief overview

Project-specific code principles for the rust-calculator codebase, ensuring high-quality, performant, and maintainable code through strict compilation standards, comprehensive testing, and proactive performance evaluation.

Follow the best practices of the language, e.g., `03-rust-best-practices.md` for Rust.

## Before make changes

- Follow `06-github-workflow.md` to create and switch to a new local branch before making any code changes.

## Compilation standards

- Ensure zero compilation errors and warnings in all code changes
- Address any compiler warnings immediately before committing code
- Maintain clean compilation output across all supported Rust versions
- Ensure no unused code or imports

## Testing requirements

- Follow comprehensive test coverage guidelines outlined in `testing-coverage.md`
- Maintain overall line coverage above 80% across the entire codebase
- Add corresponding unit tests for every source code change
- Test both success and error cases comprehensively

## Performance evaluation

- Evaluate potential performance degradation before implementing requirements
- Raise performance concerns proactively during code review or implementation
- Use profiling tools to measure and validate performance impacts
- Prioritize performance optimizations for user-facing calculator operations
