# Pulse 129: Medium Eastern Hungary Ottoman Parentage Batch 03

## Intent

Continue the medium-priority parentage queue with a bounded vassal-state
relationship that fits the current source-backed parentage model.

## Changes

- Add one reviewed Wikimedia text source for Eastern Hungarian Kingdom
  parentage context.
- Import `title-q625380 -> title-q12560` for 1529-1570.
- Regenerate parentage coverage, parentage gap TSV, full gap report, shard TSVs,
  and shard reports.

## Boundary

No Habsburg counterclaim, unsettled territorial extent, Royal Hungary division,
Transylvanian continuity, Ottoman Hungary province detail, borders, holders, or
post-1570 Principality of Transylvania claims are imported.

## Current State

- sources: 415
- facts: 1265
- titles: 336
- parentage facts: 257
- titles without parentage: 124
- active high-priority parentage gaps: 0
- medium-priority parentage gaps: 84
- reviewed blocked parentage gaps: 2

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/medium-parentage-eastern-hungary-ottoman-batch-03-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/medium-parentage-eastern-hungary-ottoman-batch-03.sources data/staging/medium-parentage-eastern-hungary-ottoman-batch-03.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/medium-parentage-eastern-hungary-ottoman-batch-03-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/medium-parentage-eastern-hungary-ottoman-batch-03.sources data/staging/medium-parentage-eastern-hungary-ottoman-batch-03.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-gap-report data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-report.md`
- `cargo fmt --check`
- `rg -n "title-q625380|Eastern Hungarian Kingdom|src-wikipedia-eastern-hungarian-kingdom" src -S`
- `git diff --check`
- `cargo test --quiet -j 1`
