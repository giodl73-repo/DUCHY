# Pulse 127: Medium Napoleonic Parentage Batch 01

## Intent

Start the medium-priority parentage queue with clear Napoleonic client-kingdom
relations that fit the current source-backed parentage model.

## Changes

- Add three reviewed Wikimedia text sources for parentage context.
- Import three parentage facts under `title-q71084` First French Empire:
  Kingdom of Holland, Kingdom of Etruria, and Napoleonic Kingdom of Italy.
- Regenerate parentage coverage, parentage gap TSV, full gap report, shard TSVs,
  and all shard reports.

## Boundary

No Napoleonic administrative detail, treaty terms, personal-union mechanics,
borders, holders, annexation events, or post-1814 settlement claims are
imported.

## Current State

- sources: 413
- facts: 1263
- titles: 336
- parentage facts: 255
- titles without parentage: 126
- active high-priority parentage gaps: 0
- medium-priority parentage gaps: 86
- reviewed blocked parentage gaps: 2

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/medium-parentage-napoleonic-batch-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/medium-parentage-napoleonic-batch-01.sources data/staging/medium-parentage-napoleonic-batch-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/medium-parentage-napoleonic-batch-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/medium-parentage-napoleonic-batch-01.sources data/staging/medium-parentage-napoleonic-batch-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-gap-report data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-report.md`
- `cargo fmt --check`
- `rg -n "title-q212278|title-q223793|title-q223936|Kingdom of Holland|Kingdom of Etruria|Kingdom of Italy \\(Napoleonic\\)" src -S`
- `git diff --check`
- `cargo test --quiet -j 1`
