# Pulse 02: Metadata Source File Format

## Goal

Define and validate a dependency-free metadata source file format.

## Changes

- Add `SourceCatalog::from_metadata_text`.
- Add parser support for `key: value` records separated by `---`.
- Add parser tests for one record, multiple records, and invalid enum values.
- Add `fixtures/source-policy.sources`.
- Update the CLI demo to parse and validate the fixture file.

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `cargo run --quiet`
- `git diff --check`

## Status

Complete.
