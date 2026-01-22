## GitHub & Issue Workflow (Mandatory for Every Task)

### Issue Creation Guidelines

When creating or being asked to create a GitHub issue:

- Always use the corresponding template in `.github/ISSUE_TEMPLATE/`
- Map user-provided information to the appropriate template sections
- Select correct issue type/label (bug, enhancement, documentation, etc.)
- For user stories / new features → `.github/ISSUE_TEMPLATE/story-template.md`
- For bug reports / defects → `.github/ISSUE_TEMPLATE/bug-report.md`
- For general feature requests → `.github/ISSUE_TEMPLATE/feature-request.md`
- Use descriptive, concise titles
- Preserve user's original wording and intent
- Ensure acceptance criteria are clear, testable, and ideally in GIVEN-WHEN-THEN format
- Include screenshots, logs, or reproduction steps for bugs
- Add relevant labels, milestones, and links

### Development Workflow

When working on a GitHub issue (#X):

- Rebase local `main` branch from remote before making any changes
- Always create a new branch from `main` or `master` branch: `personal/{your-username}-{issue_id}` (e.g., `personal/shenghu-42`).
- Rebase the newly created local branch from remote `main` branch
- Perform all development in this branch
- Use Conventional Commits prefixes:
  - `feat:` New feature (MINOR semver)
  - `fix:` Bug fix (PATCH semver)
  - `docs:` Documentation only
  - `style:` Formatting/whitespace (no logic change)
  - `refactor:` Code cleanup (no behavior change)
  - `perf:` Performance improvement
  - `test:` Adding/fixing tests
  - `build:` Build/dependency changes
  - `ci:` CI config changes
  - `chore:` Misc non-production changes
  - Add "BREAKING CHANGE:" footer for major semver bumps
  - Messages: Imperative, concise (e.g., "Add history feature")
- Before pushing: Rebase on main, run `cargo clippy -- -D warnings`, `cargo fmt`, `cargo test`
- Ensure all CI checks pass before creating or requesting review of a PR
- Create PR titled: `[{issue_id}] {short summary}`
- PR body must include: "Resolves #{issue_id}" and summary of all commits included
