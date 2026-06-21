# Pulse 128: Medium Sicily Aragon Parentage Batch 02

## Intent

Continue the medium-priority parentage queue with a clear crown-level
relationship that fits the current source-backed parentage model.

## Changes

- Add one reviewed Wikimedia text source for Kingdom of Sicily parentage
  context.
- Import `title-q188586 -> title-q204920` for 1282-1707.
- Regenerate parentage coverage, parentage gap TSV, full gap report, shard TSVs,
  and shard reports.
- Remove obsolete generated batch-006 shard files after the queue shrinks to
  five full 25-row shards.

## Boundary

No Norman foundation history, Angevin split mechanics, Kingdom of Naples
relations, Spanish Habsburg succession, Savoyard period, Austrian rule, Malta
fief, borders, holders, or post-1707/1713 successor claims are imported.

## Current State

- sources: 414
- facts: 1264
- titles: 336
- parentage facts: 256
- titles without parentage: 125
- active high-priority parentage gaps: 0
- medium-priority parentage gaps: 85
- reviewed blocked parentage gaps: 2

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/medium-parentage-sicily-aragon-batch-02-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/medium-parentage-sicily-aragon-batch-02.sources data/staging/medium-parentage-sicily-aragon-batch-02.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/medium-parentage-sicily-aragon-batch-02-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/medium-parentage-sicily-aragon-batch-02.sources data/staging/medium-parentage-sicily-aragon-batch-02.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-gap-report data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-report.md`
- `cargo fmt --check`
- `rg -n "title-q188586|Kingdom of Sicily|src-wikipedia-kingdom-sicily-aragon" src -S`
- `git diff --check`
- `cargo test --quiet -j 1`
