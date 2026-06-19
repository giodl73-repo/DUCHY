# Pulse 26: Manifest Review Report

## Intent

Give reviewers a readable candidate packet for any staging manifest or shard.

## Changes

- Add `duchy-import manifest-report <manifest> <output.md>`.
- Validate the source manifest before report generation.
- Summarize total candidates and status counts.
- List every candidate grouped by `pending`, `reviewed`, `promoted`, and
  `rejected` queue state.

## Review Boundary

Manifest reports are review artifacts. They do not promote source records,
accept facts, or change reviewed fixture history.

## Validation

- `cargo test --quiet`
- `cargo run --bin duchy-import -- manifest-report data/staging/example.manifest <temp report>`
- inspect generated report
- `cargo fmt --check`
- `git diff --check`
