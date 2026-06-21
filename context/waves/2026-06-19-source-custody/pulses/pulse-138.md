# Pulse 138: County Title Harvest Batch 01

## Intent

Promote the first safe title-only packets from the 500-row county-scale farm
without inferring parentage from incomplete evidence.

## Changes

- Add `data/staging/county-title-harvest-batch-01.sources`.
- Add `data/staging/county-title-harvest-batch-01.facts`.
- Add `data/staging/county-title-harvest-batch-01-review.md`.
- Promote Dorohoi County, County Palatine of Durham, and County of Girona
  title-only facts into accepted fixtures.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

No parentage facts are promoted. County of Foix, County of Forcalquier, and
Free imperial city of Dortmund remain deferred because the opened evidence did
not support clean fixture promotion in this pass.

## Current State

- reviewed sources: 422
- accepted facts: 1280
- accepted titles: 340
- parentage facts: 260
- titles without parentage: 125
- parentage gap rows: 125

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/county-title-harvest-batch-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/county-title-harvest-batch-01.sources data/staging/county-title-harvest-batch-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/county-title-harvest-batch-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/county-title-harvest-batch-01.sources data/staging/county-title-harvest-batch-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
