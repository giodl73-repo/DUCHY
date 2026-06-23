# Pulse 177: Early Lorraine East Francia Parentage

## Intent

Import one bounded early parentage span for Duchy of Lorraine. The reviewed
source separates the early East Francia status from the later Holy Roman Empire
status, matching DUCHY's current 962 boundary convention.

## Changes

- Add `src-wikipedia-duchy-lorraine`.
- Add `fact-q155019-parent-q153080`.
- Parent Duchy of Lorraine under East Francia for `959..961`.
- Keep the existing Duchy of Lorraine -> Holy Roman Empire fact active from
  `962..1766`; no supersession is needed because the spans do not overlap.
- Regenerate active graph, coverage, gap, rank-skip, candidate, bridge, and
  change reports.

## Boundary

This pulse imports only the bounded East Francia parentage slice. It does not
import Lotharingian predecessor detail, Upper/Lower division mechanics,
Burgundian or French occupations, ducal succession, Barrois mouvant, or
annexation administration.

## Current State

- reviewed sources: 442
- reviewed facts: 1327
- materialized titles: 349
- active parentage facts: 278
- superseded parentage facts: 2
- active rank-skip rows: 220
- weighted span coverage: 59.44%
- temporal parent conflicts: 0
- snapshot cycle years: 0

## Validation

- `cargo test --quiet`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
