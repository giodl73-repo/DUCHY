# Pulse 01: Metadata-Only Source Records

## Goal

Implement the first source-custody intake layer: metadata-only source records
and review decisions.

## Changes

- Add `SourceRecord`.
- Add `SourceKind`.
- Add `AllowedUse`.
- Add `SourceReview` and `SourceReviewDecision`.
- Add `SourceCatalog` validation.
- Add a policy-source catalog for the licensing/copyright pages already used by
  the source-custody package.
- Add tests for valid metadata-only records, missing required metadata, and
  orphan reviews.
- Update the CLI demo to validate the source policy catalog.

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `cargo run --quiet`
- `git diff --check`

## Status

Complete.
