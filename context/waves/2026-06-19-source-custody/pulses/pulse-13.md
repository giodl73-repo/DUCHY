# Pulse 13: Fixture-Canonical Import Path

## Intent

Prepare DUCHY to scale reviewed historical imports by making fixture files the
source of truth for source-backed historical records.

## Changes

- Make `first_real_source_catalog` parse `fixtures/first-real.sources`.
- Make `first_real_fact_records` parse `fixtures/first-real.facts`.
- Remove source-backed historical IDs, names, spans, and relations from Rust
  literals.
- Keep Rust tests focused on generic parser, validation, materialization, and
  query behavior.

## Review Boundary

Synthetic Rust test fixtures may still use invented IDs and values. Reviewed
historical data belongs in reviewed source and fact fixtures only.

## Validation

- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`
