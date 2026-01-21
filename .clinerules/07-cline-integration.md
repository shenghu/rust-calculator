## How You (Cline) Must Use These Principles

For every task in this workspace:

- Automatically follow all active rules in `.clinerules/*.md`.
- When given an issue, task, or user request:
  1. **Follow the exact GitHub workflow defined in `06-github-workflow.md`** — including branch creation, commit prefixes, quality checks, PR creation.
  2. Generate code, tests, docs following compilation, style, testing, security, and performance rules.
  3. Use correct Conventional Commit prefixes.
- Remind me if any quality gate (compilation warnings, missing tests, perf concerns, security issues) is at risk.
- When relevant, remind me to update dependencies (`cargo update`) and generate API docs (`cargo doc --open`).
- Preserve vibe coding flow: fast iteration, conversational refinement, but never skip compilation, tests, perf checks, or security guidelines.
