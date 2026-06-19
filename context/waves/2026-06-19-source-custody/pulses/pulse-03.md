# Pulse 03: Source-Backed Fact Gate

## Goal

Validate future source-backed fact records against accepted source records and
confidence labels before any real historical data import.

## Changes

- Add `FactRecord`.
- Add `ClaimKind`.
- Add `ConfidenceLabel`.
- Add `SourceCatalog::validate_fact`.
- Require source-backed facts to cite reviewed sources whose allowed use permits
  fact claims.
- Reject seed, metadata-pointer, and unsupported confidence labels for
  source-backed fact imports.
- Reject bad confidence/source-count combinations.
- Reject contested facts without conflict groups.
- Add CLI smoke showing metadata-only policy records cannot import facts.

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `cargo run --quiet`
- `git diff --check`

## Status

Complete. No real historical facts have been imported.
