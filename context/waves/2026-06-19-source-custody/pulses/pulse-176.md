# Pulse 176: Early Lower Lotharingia East Francia Parentage

## Intent

Import one missing early parentage span already supported by an accepted text
source. Lower Lotharingia has an active Holy Roman Empire edge from 962 onward;
the reviewed source also identifies it as part of East Francia before that
boundary.

## Changes

- Add `fact-q660393-parent-q153080`.
- Parent Lower Lotharingia under East Francia for `959..961`.
- Keep the existing Lower Lotharingia -> Holy Roman Empire fact active from
  `962..1190`; no supersession is needed because the spans do not overlap.
- Regenerate active graph, coverage, gap, rank-skip, candidate, bridge, and
  change reports.

## Boundary

This pulse imports only the bounded East Francia parentage slice. It does not
import Middle Francia predecessor detail, Upper Lotharingian split mechanics,
ducal succession, Brabant inheritance, Lower Lorraine titulature, or post-1190
claims.

## Current State

- reviewed sources: 441
- reviewed facts: 1326
- materialized titles: 349
- active parentage facts: 277
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
