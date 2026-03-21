---
trigger: always_on
---

# Conventional Commits & Versioning

All commits in this repository MUST follow the [Conventional Commits](https://www.conventionalcommits.org/) specification. This is not optional — the CI release pipeline (`release-please`) parses commit messages to automatically bump versions, generate changelogs, and create GitHub Releases.

## Commit Format

```
<type>(<optional scope>): <description>

[optional body]

[optional footer(s)]
```

## Allowed Types

| Type | When to Use | Semver Bump |
|---|---|---|
| `feat` | New feature, public API addition, new CLI command | **MINOR** (0.x → 0.x+1) |
| `fix` | Bug fix, corrected behavior | **PATCH** (0.0.x → 0.0.x+1) |
| `chore` | Maintenance: deps, CI, docs, configs, refactors with no API change | No bump |
| `docs` | Documentation-only changes | No bump |
| `test` | Adding or fixing tests (no production code change) | No bump |
| `refactor` | Code restructuring with no behavior change | No bump |
| `perf` | Performance improvement with no API change | No bump |
| `ci` | CI/CD pipeline changes | No bump |

## Breaking Changes

Append `!` after the type or add `BREAKING CHANGE:` in the footer:

```
feat!: replace ring buffer with bounded channel

BREAKING CHANGE: RingBuffer::new() now requires a capacity argument.
```

This triggers a **MAJOR** bump (x.0.0 → x+1.0.0).

## Scope (Optional)

Use the crate name as scope when the change is localized:

```
feat(chronos-core): add SemanticLog model
fix(chronos-capture): prevent overflow in ring buffer
chore(chronos-daemon): update sqlx to 0.8
```

## Examples

```bash
# Good ✅
git commit -m "feat(chronos-core): add VisionInference trait"
git commit -m "fix(chronos-daemon): handle empty query results"
git commit -m "chore: update CI to include codecov"
git commit -m "test(chronos-capture): add ring buffer overflow tests"
git commit -m "docs: update README with prerequisites"

# Bad ❌
git commit -m "updated stuff"
git commit -m "WIP"
git commit -m "fix bug"
git commit -m "Add new feature for the capture module"
```

## Rules

1. **ALWAYS** use lowercase type prefix (`feat:`, not `Feat:` or `FEAT:`)
2. **NEVER** capitalize the first letter of the description
3. **NO** period at the end of the description
4. **USE** imperative mood: "add feature" not "added feature" or "adds feature"
5. **SCOPE** is optional but encouraged for multi-crate workspaces
6. If a commit touches multiple crates, omit the scope and explain in the body
