# Pulse 06: Answer Trace And Source Status

## Goal

Implement `WP-005`: make query answers explainable and distinguish answered,
empty, unknown, unsupported, seed, and reserved contested/source-backed states.

## Changes

- Add `QueryEnvelope<T>`.
- Add `QueryStatus`.
- Add `SourceClass`.
- Add `TraceNote`.
- Add status-aware title-path and transfer query methods.
- Add negative tests for unknown, empty, and unsupported answers.
- Update the CLI demo to print query status and trace code.

## Role Review

| Role | Decision |
|---|---|
| Timeline Steward | Status layer does not change timeline semantics. |
| Territorial Lineage Reviewer | Basic lineage and transfer answers now carry trace codes. |
| Query Interface Reviewer | Answered, empty, unknown, and unsupported states are implemented; contested remains reserved until fixtures require it. |
| Source Custody Reviewer | Seed answers are labeled seed; source-backed confidence remains blocked on WP-006. |
| Game Systems Reviewer | No game mechanics were added. |

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `cargo run --quiet`
- `git diff --check`

## Status

Complete.
