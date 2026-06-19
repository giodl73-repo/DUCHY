# Pulse 03: Temporal Area/Title Model

## Goal

Implement `WP-002`: make DUCHY capable of representing areas that move between
duchies before adding user-facing transfer queries.

## Changes

- Add stable `Area` identity.
- Add `AreaTitleSpan` links from areas to title identities over time.
- Add `ParentageSpan` for temporal de jure parentage.
- Add seed fixtures for no-transfer, single-transfer, and multi-transfer
  county-to-duchy cases.
- Add tests for area identity, temporal parentage, and parent-rank validation.

## Role Review

| Role | Decision |
|---|---|
| Timeline Steward | Timeline-safe for temporal parentage spans and rank validation. |
| Territorial Lineage Reviewer | Lineage fixture baseline is ready for future range-transfer queries. |
| Query Interface Reviewer | Fixture coverage now includes no-transfer, single-transfer, and multi-transfer cases. |
| Source Custody Reviewer | Seed fixtures remain fictional/non-authoritative; real source import remains deferred. |
| Game Systems Reviewer | No game mechanics were added to the core lineage model. |

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `cargo run --quiet`
- `git diff --check`

## Status

Complete.
