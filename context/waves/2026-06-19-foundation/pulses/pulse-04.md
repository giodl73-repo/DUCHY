# Pulse 04: Year Title-Path Query

## Goal

Implement `WP-003`: answer which higher-title path contained an area or title in
a specified year.

## Changes

- Add `TitlePathAnswer` and `TitlePathStep`.
- Add `title_path_for_area_in_year`.
- Add `title_path_for_title_in_year`.
- Include ordered continuity events in path answers.
- Add tests for area path resolution, temporal parentage, and missing
  year/area results.
- Update the CLI demo to print a seed area path.

## Role Review

| Role | Decision |
|---|---|
| Timeline Steward | Timeline-safe: query follows temporal parentage spans. |
| Territorial Lineage Reviewer | Ready for county-to-duchy-to-kingdom year lookup. |
| Query Interface Reviewer | Query has a typed answer and negative tests; richer statuses remain WP-005. |
| Source Custody Reviewer | Seed demo remains fictional/non-authoritative. |
| Game Systems Reviewer | No game mechanics were added. |

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `cargo run --quiet`
- `git diff --check`

## Status

Complete.
