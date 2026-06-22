# Pulse 159: Rank Skip Bridge Queue

## Intent

Distill the broad rank-skip candidate report into a compact queue of skipped
edges with an accepted same-current-parent bridge candidate.

## Changes

- Add `duchy-import parentage-rank-skip-bridges-tsv`.
- Generate `data/staging/parentage-rank-skip-bridges.tsv`.
- Export each skipped edge with its strongest accepted expected-rank bridge
  candidate, bridge fact ID, overlap years, and review priority.
- Document the command and bridge-queue semantics in `README.md`.
- Add VTRACE evidence for the bridge queue baseline.

## Boundary

Bridge rows are review leads only. They show that DUCHY already has an accepted
candidate title of the expected rank under the same current parent during an
overlapping span; they do not prove the child belongs under that candidate.
This pulse does not promote sources, facts, titles, parentage spans, de facto
relations, successor semantics, or territorial geometry.

## Current State

- rank-skip rows: 222
- bridge queue rows: 162
- source catalog: 438 reviewed sources
- accepted facts: 1321
- accepted titles: 349
- parentage facts: 274

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridges-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-bridges.tsv`
