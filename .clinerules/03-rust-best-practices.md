## Brief overview

Project-specific guidelines for applying Rust best practices and coding principles in the rust-calculator codebase, focusing on idiomatic Rust patterns, memory safety, and performance.

## Error handling

- Use `Result<T, E>` for operations that can fail, rather than panicking
- Leverage the `?` operator for early returns on errors
- Define custom error types using `thiserror` for better error messages
- Avoid `.unwrap()` and `.expect()` in production code; use proper error propagation

## Ownership and borrowing

- Follow ownership rules: each value has exactly one owner, borrowing is immutable by default
- Use references (`&`) when ownership transfer is not needed
- Prefer `&str` over `String` for string slices when possible
- Use `Rc<RefCell<T>>` or `Arc<Mutex<T>>` only when single ownership is insufficient

## Memory safety

- Embrace Rust's borrow checker; don't fight against lifetime rules
- Use smart pointers (`Box`, `Rc`, `Arc`) appropriately for heap allocation
- Avoid raw pointers and unsafe code unless absolutely necessary
- Implement `Drop` trait for custom cleanup when RAII is needed

## Idiomatic patterns

- Use `match` expressions over multiple `if-else` chains
- Implement common traits (`Debug`, `Clone`, `PartialEq`, etc.) where appropriate
- Use iterator methods (`map`, `filter`, `fold`) instead of manual loops
- Prefer `struct` with named fields over tuples for complex data

## Performance considerations

- Avoid unnecessary allocations; reuse buffers when possible
- Use `Vec::with_capacity()` when the size is known in advance
- Consider `Cow<T>` for conditionally owned data
- Profile with `cargo flamegraph` or similar tools before optimizing

## Code organization

- Group related functionality into modules
- Use `pub(crate)` for internal APIs, `pub` only for public interfaces
- Separate concerns: business logic, I/O, error handling
- Keep functions focused and under 50 lines when possible
