# Pulse 18: Promotion Review Reports

## Intent

Make staged batch promotion reviewable before accepted fixture files are
rewritten.

## Changes

- Add `duchy-promote --report <path>`.
- Include promotion mode, candidate and merged counts, candidate titles,
  candidate parentage, and candidate fact IDs in the report.
- Document report usage for staging review.

## Review Boundary

Reports summarize candidate fixture rows after parser and validation checks.
They do not replace source-custody review of the original sources.

## Validation

- `cargo test --quiet`
- `cargo run --bin duchy-promote -- --dry-run --report <temp report> fixtures/first-real.sources fixtures/first-real.facts data/staging/example.sources data/staging/example.facts`
- `cargo fmt --check`
- `git diff --check`
