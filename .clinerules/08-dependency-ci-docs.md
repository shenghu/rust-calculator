## Dependency, CI & Documentation Guidelines

### Dependency Management

- **Only upgrade or update dependencies when explicitly requested by a GitHub issue** (e.g., "Upgrade iced to latest version", "Fix security vulnerability in crate X", "Add new crate Y").
- Do NOT run `cargo update`, `cargo upgrade`, or suggest dependency changes on unrelated tasks — this avoids unnecessary changes to `Cargo.lock`.
- When dependency work **is** required:
  - Run `cargo update` only for the affected crates if possible (e.g., `cargo update -p iced`)
  - Always run `cargo audit` to check for vulnerabilities before proposing upgrades
  - Commit dependency changes as a separate `build(deps):` or `chore(deps):` commit
  - Pin exact versions in `Cargo.toml` when stability is critical
- Regularly (but not per-issue) run `cargo audit` in CI to catch issues early

### CI Guidelines

- Ensure the project has a GitHub Actions workflow that runs on every PR:
  - `cargo fmt -- --check`
  - `cargo clippy -- -D warnings`
  - `cargo test`
  - `cargo tarpaulin --out Lcov` (for coverage reporting)
- Do NOT modify CI configuration unless the issue is about CI/CD

### Documentation Guidelines

- When adding or changing public APIs, generate and preview docs locally with `cargo doc --open`
- Keep `README.md` up-to-date with build/test instructions and contribution guidelines
- Consider enabling GitHub Pages or docs.rs for public crates when the project is ready
