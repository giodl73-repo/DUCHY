# Pulse 140: County Title Harvest Batch 03

## Intent

Promote the remaining clean free-city title candidate from the county-scale farm
using corrected evidence from Britannica.

## Changes

- Add `data/staging/county-title-harvest-batch-03.sources`.
- Add `data/staging/county-title-harvest-batch-03.facts`.
- Add `data/staging/county-title-harvest-batch-03-review.md`.
- Promote Free imperial city of Dortmund title-only facts into accepted
  fixtures.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

No parentage facts are promoted. County of Foix, County of Forcalquier, County
of La Marche, and Komarom County remain deferred because the opened evidence has
approximate starts, unsupported farm spans, or unresolved span policy.

## Current State

- reviewed sources: 427
- accepted facts: 1295
- accepted titles: 345
- parentage facts: 260
- titles without parentage: 130
- parentage gap rows: 130
- parentage gap shards: 6

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/county-title-harvest-batch-03-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/county-title-harvest-batch-03.sources data/staging/county-title-harvest-batch-03.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/county-title-harvest-batch-03-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/county-title-harvest-batch-03.sources data/staging/county-title-harvest-batch-03.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
