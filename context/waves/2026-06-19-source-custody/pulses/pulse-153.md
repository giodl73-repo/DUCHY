# Pulse 153: Poland Commonwealth Parentage

## Intent

Import the endpoint transition from the Jagiellon Kingdom of Poland into the
Polish-Lithuanian Commonwealth for 1569.

## Changes

- Add `data/staging/poland-commonwealth-parentage-01.sources`.
- Add `data/staging/poland-commonwealth-parentage-01.facts`.
- Add `data/staging/poland-commonwealth-parentage-01-review.md`.
- Promote Kingdom of Poland endpoint parentage under the Polish-Lithuanian
  Commonwealth for `1569..1569`.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

This packet imports only the reviewed 1569 endpoint transition from the
Jagiellon Kingdom of Poland to the Polish-Lithuanian Commonwealth. No longer
Commonwealth child span, Crown of Poland territorial semantics, Jagiellonian
dynastic claims, Sejm mechanics, federal structure, partitions, or later Polish
state claims are promoted.

## Current State

- reviewed sources: 436
- accepted facts: 1317
- accepted titles: 348
- parentage facts: 273
- titles with parentage: 228
- titles without parentage: 120
- parentage gap rows: 120

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/poland-commonwealth-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/poland-commonwealth-parentage-01.sources data/staging/poland-commonwealth-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/poland-commonwealth-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/poland-commonwealth-parentage-01.sources data/staging/poland-commonwealth-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
