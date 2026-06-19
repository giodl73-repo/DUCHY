# Pulse 05: Duchy-Transfer Query

## Goal

Implement `WP-004`: answer when an area or title moved between duchies across a
date range.

## Changes

- Add `TransferAnswer` and `TransferStep`.
- Add `transfers_for_area_between`.
- Add `transfers_for_title_between`.
- Include event kind and note when a continuity event occurs in the transfer
  year.
- Add tests for no-transfer, single-transfer, multi-transfer, and range-filtered
  cases.
- Update the CLI demo to print seed area transfers.

## Role Review

| Role | Decision |
|---|---|
| Timeline Steward | Timeline-safe for clear ordered parentage changes. |
| Territorial Lineage Reviewer | Ready for basic "moved between duchies" questions. |
| Query Interface Reviewer | Clear-transfer cases are covered; contested/split status remains WP-005. |
| Source Custody Reviewer | Seed demo remains fictional/non-authoritative. |
| Game Systems Reviewer | No game mechanics were added. |

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `cargo run --quiet`
- `git diff --check`

## Status

Complete.
