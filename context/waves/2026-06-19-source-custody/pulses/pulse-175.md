# Pulse 175: Early Saxony East Francia Parentage

## Intent

Import one source-backed parentage span for an early German stem duchy without
changing the existing Holy Roman Empire edge. The reviewed Duchy of Saxony
source supports Saxony's placement under East Francia after the Treaty of
Verdun and before the HRE span begins.

## Changes

- Add `src-wikipedia-duchy-saxony`.
- Add `fact-q164092-parent-q153080`.
- Parent Duchy of Saxony under East Francia for `843..961`.
- Keep the existing Duchy of Saxony -> Holy Roman Empire fact active from
  `962..1296`; no supersession is needed because the spans do not overlap.
- Regenerate active graph, coverage, gap, rank-skip, candidate, and change
  reports.

## Boundary

This pulse imports only the bounded parentage relationship needed for the
temporal hierarchy. It does not import Old Saxony prehistory, ducal succession,
Ottonian dynastic claims, Ascanian partition detail, electorate transfer, or
post-1296 successor-state claims.

## Current State

- reviewed sources: 441
- reviewed facts: 1325
- materialized titles: 349
- active parentage facts: 276
- superseded parentage facts: 2
- active rank-skip rows: 220
- weighted span coverage: 59.43%
- temporal parent conflicts: 0
- snapshot cycle years: 0

## Validation

- `cargo test --quiet`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
