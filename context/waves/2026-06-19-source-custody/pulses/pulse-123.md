# Pulse 123: Remaining High-Priority Blocker Review

## Intent

Record why the final high-priority parentage rows are not safely importable in
the current fixture/model state.

## Changes

- Add `data/staging/high-parentage-remaining-blockers.md`.
- Record three unresolved high-priority rows:
  - Q187035 Principality of Albania.
  - Q50625 Hordaland.
  - Q779011 Principality of Montenegro.

## Boundary

No parentage facts are imported in this pulse. The remaining rows stay in the
generated gap queue until modern Norway parent coverage or contested/de facto
relation modeling exists.

## Current State

- sources: 408
- facts: 1256
- titles: 335
- parentage facts: 251
- titles with parentage: 206
- titles without parentage: 129
- high-priority parentage gaps remaining: 3

## Validation

- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
