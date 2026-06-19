# Pulse 16: Batch Import Staging Gate

## Intent

Prepare DUCHY for mass historical data intake by adding a staging gate before
accepted fixture promotion.

## Changes

- Add `duchy-import` status CLI for source/fact fixture batches.
- Add `duchy-promote --dry-run` to validate candidate staging batches merged
  with accepted fixtures without writing files.
- Add duplicate source ID, duplicate fact ID, and contradictory accepted-claim
  validation.
- Add synthetic `data/staging/` examples.

## Review Boundary

This pulse does not import new historical data. It adds the validation and
workflow surface required before larger candidate batches can be promoted.

## Validation

- `cargo test --quiet`
- `cargo run --quiet`
- `cargo run --bin duchy-import -- status`
- `cargo run --bin duchy-promote -- --dry-run fixtures/first-real.sources fixtures/first-real.facts data/staging/example.sources data/staging/example.facts`
- `cargo fmt --check`
- `git diff --check`
