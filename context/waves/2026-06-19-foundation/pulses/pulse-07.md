# Pulse 07: Source-Custody Gate

## Goal

Implement `WP-006`: define the source-custody gate for future real European
title data without importing data.

## Changes

- Add `docs/vtrace/source-custody/`.
- Add source inventory and rights posture.
- Add import policy for allowed, blocked, and future-gated use.
- Add confidence model for seed, metadata pointer, single-source,
  multi-source, contested, and unsupported facts.
- Add source/fact schema sketch for future fixtures.
- Add source-custody review gate.

## Role Review

| Role | Decision |
|---|---|
| Source Custody Reviewer | Custody-safe as a policy package; concrete imports remain blocked until source records pass the gate. |
| Timeline Steward | Source policy preserves separate claims for parentage, control, holder, area identity, and events. |
| Territorial Lineage Reviewer | Real lineage coverage can proceed only through source-backed area/title claims. |
| Query Interface Reviewer | Future source-backed answers must carry confidence and source trace. |
| Game Systems Reviewer | Commercial-game data remains forbidden. |

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `cargo run --quiet`
- `git diff --check`

## Status

Complete for policy. No real historical data has been imported.
