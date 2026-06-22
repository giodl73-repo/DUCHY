# Pulse 156: Rank Skip Review Queue

## Intent

Turn the parentage graph audit's rank-skip finding into an actionable review
queue for filling missing intermediate hierarchy layers.

## Changes

- Add `duchy-import parentage-rank-skip-tsv`.
- Generate `data/staging/parentage-rank-skip-targets.tsv`.
- Export fact ID, child ID/name/rank, expected immediate parent rank, current
  parent ID/name/rank, span, review priority, and review notes for each
  rank-skipped parentage fact.
- Document the command and current queue shape in `README.md`.
- Add VTRACE evidence for the rank-skip queue baseline.

## Boundary

This pulse exports review targets only. It does not promote any new source
records, facts, titles, parentage spans, successor semantics, de facto relation
claims, or territorial geometry.

## Current State

- reviewed sources: 438
- accepted facts: 1321
- accepted titles: 349
- parentage facts: 274
- rank-skip rows: 222
- high-priority intermediate-parent rows: 160
- medium-priority intermediate-parent rows: 23
- low-priority intermediate-parent rows: 39
- largest class: duchy-to-empire parentage, 137 rows
- second-largest class: kingdom-to-empire parentage, 39 rows

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv`
