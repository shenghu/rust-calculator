## Brief overview

Project-specific security guidelines for the rust-calculator desktop application, focusing on memory safety, input validation, and secure coding practices to protect users and prevent common vulnerabilities.

## Memory safety and unsafe code

- Avoid `unsafe` blocks unless absolutely necessary for FFI or performance-critical sections
- When using `unsafe`, add comprehensive safety comments explaining why it's safe
- Use safe abstractions over raw pointers and manual memory management
- Leverage Rust's ownership system to prevent common memory vulnerabilities

## Input validation and sanitization

- Validate all user inputs before processing, especially numeric inputs and expressions
- Use bounds checking to prevent integer overflows and underflows (consider checked arithmetic: `checked_add`, `checked_mul`, etc.)
- Sanitize mathematical expressions to prevent injection-like attacks or malformed input
- Implement reasonable length limits on input strings to prevent resource exhaustion (e.g., max 1024 chars for expressions)
- **Prefer a safe parser (pest, nom, or custom recursive descent) over eval-like execution** ← Added: This is the single most important defense for a calculator

## Error handling and information leakage

- Use custom error types instead of exposing internal implementation details
- Avoid logging sensitive information in production builds
- Return generic error messages to users ("Invalid expression") while logging detailed errors internally (for debugging)
- Handle division by zero and other mathematical edge cases gracefully (return custom `MathError::DivisionByZero`)

## Secure defaults and fail-safe behavior

- Default to secure configurations and require explicit opt-in for risky features
- Implement graceful degradation when security features fail
- Use principle of least privilege for file system and network access (our app currently needs none)
- Validate configuration files and reject malformed inputs (if we add config later)

## Dependency and supply chain security

- Audit third-party dependencies using `cargo audit` regularly (integrate into CI)
- Pin dependency versions in `Cargo.lock` to prevent unexpected updates
- Prefer well-maintained crates with active security disclosures (e.g., iced, num-traits)
- Minimize dependency surface area by avoiding unnecessary crates

## Cryptographic practices

- (Currently not applicable — no crypto needed in calculator)
- If future features require randomization or hashing, use established libraries like `ring` or `rand` with secure defaults
- Avoid implementing custom cryptographic algorithms
- Use secure random number generation for any randomization needs

## Desktop application security

- Request minimal system permissions required for calculator functionality (none for core features)
- Implement input sanitization to prevent command injection through expressions
- Use sandboxing features available in the desktop environment when possible (e.g., Flatpak sandbox on Linux, App Sandbox on macOS)
- Handle window management and UI events securely to prevent UI-based attacks (e.g., no eval of user-supplied strings in iced messages)

## Build and deployment security

- Enable security-relevant compiler flags and warnings (`-D warnings`, stack-protector)
- Use reproducible builds for verification (consider `cargo build --locked`)
- Implement code signing for releases (macOS/Windows)
- Enable stack protection and other hardening techniques where available

## Testing and validation

- Include security-focused unit tests for input validation and edge cases (huge expressions, malformed input, overflows)
- Test against common attack vectors like buffer overflows and injection attacks (though Rust mitigates many)
- Use fuzzing tools (e.g., `cargo fuzz`) to discover unexpected input handling issues
- Regularly update dependencies and test against known vulnerabilities (`cargo audit --ignore` only when justified)
