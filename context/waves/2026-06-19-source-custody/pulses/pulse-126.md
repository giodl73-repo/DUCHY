# Pulse 126: Parentage Gap Blocker Queue

## Intent

Close the active high-priority parentage queue without importing overstated
parentage for historically ambiguous Albania and Montenegro cases.

## Changes

- Add `data/staging/parentage-gap-blockers.tsv`.
- Extend `duchy-import parentage-gap-tsv` with an optional blocker TSV input.
- Mark reviewed blocker rows as `blocked_parentage_review` while keeping them
  visible in generated parentage gap queue artifacts.
- Regenerate `data/staging/parentage-gap-targets.tsv` and changed shard reports.

## Boundary

No additional source records or parentage facts are imported. Normal
source-backed lineage materialization still rejects contested parentage facts.

## Current State

- sources: 410
- facts: 1260
- titles: 336
- parentage facts: 252
- parentage gap rows: 129
- active high-priority parentage gaps: 0
- reviewed blocked parentage gaps: 2

## Validation

- `cargo fmt --check`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-gap-report data/staging/parentage-gap-shards/batch-002.tsv data/staging/parentage-gap-shards/batch-002-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-report data/staging/parentage-gap-shards/batch-005.tsv data/staging/parentage-gap-shards/batch-005-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-report data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-report.md`
- `rg -n "title-q187035|title-q779011|Principality of Albania|Principality of Montenegro" src -S`
- `git diff --check`
- `cargo test --quiet -j 1`
