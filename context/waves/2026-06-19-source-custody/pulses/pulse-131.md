# Pulse 131: Medium Bosnia Ottoman Parentage Batch 05

## Intent

Continue the medium-priority parentage queue with a narrow endpoint parentage
span for a documented conquest and title fall.

## Changes

- Add one reviewed Wikimedia text source for Kingdom of Bosnia endpoint
  parentage context.
- Import `title-q2980623 -> title-q12560` for 1463.
- Regenerate parentage coverage, parentage gap TSV, full gap report, shard TSVs,
  and shard reports.

## Boundary

No earlier Ottoman attacks, Hungarian nominal claims, Banate of Jajce
restoration, Herzegovina conquest, Bosnian sanjak/eyalet administration,
borders, rulers, or post-1463 legal-continuity claims are imported.

## Current State

- sources: 418
- facts: 1270
- titles: 337
- parentage facts: 259
- titles without parentage: 123
- active high-priority parentage gaps: 0
- medium-priority parentage gaps: 82
- root/successor review rows: 39
- reviewed blocked parentage gaps: 2

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/medium-parentage-bosnia-ottoman-batch-05-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/medium-parentage-bosnia-ottoman-batch-05.sources data/staging/medium-parentage-bosnia-ottoman-batch-05.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/medium-parentage-bosnia-ottoman-batch-05-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/medium-parentage-bosnia-ottoman-batch-05.sources data/staging/medium-parentage-bosnia-ottoman-batch-05.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-gap-report data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-report.md`
- `cargo fmt --check`
- `rg -n "title-q2980623|Kingdom of Bosnia|src-wikipedia-kingdom-bosnia-ottoman" src -S`
- `git diff --check`
- `cargo test --quiet -j 1`
