# Pulse 155: Parentage Graph Audit

## Intent

Add a graph-level audit for reviewed parentage so DUCHY can measure whether the
accepted hierarchy is behaving like a coherent temporal forest before the next
scale-up.

## Changes

- Add `duchy-import parentage-graph-report`.
- Generate `data/staging/parentage-graph-report.md`.
- Report title-level parentage fill, weighted span coverage, snapshot density,
  roots, orphans, depth distribution, rank skips, temporal parent conflicts,
  and snapshot cycles.
- Document the graph report command in `README.md`.
- Add VTRACE evidence for the graph audit baseline.

## Boundary

This pulse adds measurement only. It does not promote new source records, facts,
titles, parentage spans, territorial geometry, holder history, de facto
relations, or same-rank successor semantics.

## Current State

- reviewed sources: 438
- accepted facts: 1321
- accepted titles: 349
- parentage facts: 274
- parentable titles: 308
- titles with parentage: 229
- title-level parentage fill: 74.35%
- weighted span coverage: 59.27%
- rank-skip facts: 222
- temporal parent conflicts: 0
- snapshot years: 544
- snapshot years with cycles: 0

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md`
