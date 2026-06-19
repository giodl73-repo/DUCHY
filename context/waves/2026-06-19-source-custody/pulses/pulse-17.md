# Pulse 17: Apply-Mode Promotion

## Intent

Finish the staging-to-accepted workflow by allowing reviewed candidate batches
to be appended to accepted fixtures after the same validation used by dry-run.

## Changes

- Add `duchy-promote --apply`.
- Reuse candidate validation, merged fixture validation, and merged timeline
  materialization before writing accepted fixture files.
- Document dry-run-first and apply workflows.

## Review Boundary

`--apply` should be used only for reviewed batches. Validation protects
mechanical consistency; source-custody review still decides whether candidate
facts are acceptable.

## Validation

- `cargo test --quiet`
- `cargo run --quiet`
- `cargo run --bin duchy-import -- status`
- `cargo run --bin duchy-promote -- --dry-run fixtures/first-real.sources fixtures/first-real.facts data/staging/example.sources data/staging/example.facts`
- `cargo run --bin duchy-promote -- --apply <temp accepted sources> <temp accepted facts> data/staging/example.sources data/staging/example.facts`
- `cargo fmt --check`
- `git diff --check`
