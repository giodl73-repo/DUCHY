# Pulse 141: County Title Harvest Batch 04

## Intent

Promote the next corrected county title from the county-scale farm while keeping
the title-only boundary intact.

## Changes

- Add `data/staging/county-title-harvest-batch-04.sources`.
- Add `data/staging/county-title-harvest-batch-04.facts`.
- Add `data/staging/county-title-harvest-batch-04-review.md`.
- Promote County of La Marche title-only facts into accepted fixtures.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

No parentage facts are promoted. The accepted start uses the first listed count
dated 958-988 because the prose start is approximate. Aquitaine, French crown,
appanage, territory, and post-1527 province claims remain unmodeled.

## Current State

- reviewed sources: 428
- accepted facts: 1298
- accepted titles: 346
- parentage facts: 260
- titles without parentage: 131
- parentage gap rows: 131
- parentage gap shards: 6

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/county-title-harvest-batch-04-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/county-title-harvest-batch-04.sources data/staging/county-title-harvest-batch-04.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/county-title-harvest-batch-04-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/county-title-harvest-batch-04.sources data/staging/county-title-harvest-batch-04.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
