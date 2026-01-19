## Brief overview

Project-specific security guidelines for the rust-calculator desktop application, focusing on memory safety, input validation, and secure coding practices to protect users and prevent common vulnerabilities.

## Memory safety and unsafe code

- Avoid `unsafe` blocks unless absolutely necessary for FFI or performance-critical sections
- When using `unsafe`, add comprehensive safety comments explaining why it's safe
- Use safe abstractions over raw pointers and manual memory management
- Leverage Rust's ownership system to prevent common memory vulnerabilities

## Input validation and sanitization

- Validate all user inputs before processing, especially numeric inputs and expressions
- Use bounds checking to prevent integer overflows and underflows
- Sanitize mathematical expressions to prevent injection-like attacks
- Implement length limits on input strings to prevent resource exhaustion

## Error handling and information leakage

- Use custom error types instead of exposing internal implementation details
- Avoid logging sensitive information in production builds
- Return generic error messages to users while logging detailed errors internally
- Handle division by zero and other mathematical edge cases gracefully

## Secure defaults and fail-safe behavior

- Default to secure configurations and require explicit opt-in for risky features
- Implement graceful degradation when security features fail
- Use principle of least privilege for file system and network access
- Validate configuration files and reject malformed inputs

## Dependency and supply chain security

- Audit third-party dependencies using `cargo audit` regularly
- Pin dependency versions to prevent unexpected updates
- Prefer well-maintained crates with active security disclosures
- Minimize dependency surface area by avoiding unnecessary crates

## Cryptographic practices

- Use established cryptographic libraries like `rust-crypto` or `ring` for security-sensitive operations
- Avoid implementing custom cryptographic algorithms
- Use secure random number generation for any randomization needs
- Store sensitive data encrypted and minimize time in decrypted state

## Desktop application security

- Request minimal system permissions required for calculator functionality
- Implement input sanitization to prevent command injection through expressions
- Use sandboxing features available in the desktop environment when possible
- Handle window management and UI events securely to prevent UI-based attacks

## Build and deployment security

- Enable security-relevant compiler flags and warnings
- Use reproducible builds for verification
- Implement code signing for releases
- Enable stack protection and other hardening techniques where available

## Testing and validation

- Include security-focused unit tests for input validation and edge cases
- Test against common attack vectors like buffer overflows and injection attacks
- Use fuzzing tools to discover unexpected input handling issues
- Regularly update dependencies and test against known vulnerabilities
