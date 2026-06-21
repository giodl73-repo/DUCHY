# Pulse 139: County Title Harvest Batch 02

## Intent

Promote the next safe title-only packets from the county-scale farm and correct
farm-suggested spans where opened source evidence differs.

## Changes

- Add `data/staging/county-title-harvest-batch-02.sources`.
- Add `data/staging/county-title-harvest-batch-02.facts`.
- Add `data/staging/county-title-harvest-batch-02-review.md`.
- Promote Duchy of Benevento, Taifa of Denia, Kingdom of Desmond, and Princely
  Abbey of Fulda title-only facts into accepted fixtures.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

No parentage facts are promoted. Duchy of Teschen, County of Forez, and County
of Furstenberg remain deferred because the opened source evidence requires span
or rank-policy review before safe fixture promotion.

## Current State

- reviewed sources: 426
- accepted facts: 1292
- accepted titles: 344
- parentage facts: 260
- titles without parentage: 129
- parentage gap rows: 129
- parentage gap shards: 6

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/county-title-harvest-batch-02-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/county-title-harvest-batch-02.sources data/staging/county-title-harvest-batch-02.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/county-title-harvest-batch-02-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/county-title-harvest-batch-02.sources data/staging/county-title-harvest-batch-02.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
