# Pulse 130: Medium Livonia Russia Parentage Batch 04

## Intent

Continue the medium-priority parentage queue with a bounded client-state
relationship, promoting the needed accepted higher-rank parent title.

## Changes

- Promote Tsardom of Russia as `title-q186096`.
- Add one reviewed Wikimedia text source for Kingdom of Livonia parentage
  context.
- Import `title-q2346056 -> title-q186096` for 1570-1578.
- Regenerate parentage coverage, parentage gap TSV, full gap report, shard TSVs,
  and shard reports.

## Boundary

No Livonian War campaign detail, Danish support mechanics, Reval siege,
territorial control claims, Magnus biography, Swedish or Polish-Lithuanian
counterclaims, or post-1578 settlement claims are imported.

## Current State

- sources: 417
- facts: 1269
- titles: 337
- parentage facts: 258
- titles without parentage: 124
- active high-priority parentage gaps: 0
- medium-priority parentage gaps: 83
- root/successor review rows: 39
- reviewed blocked parentage gaps: 2

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/medium-parentage-livonia-russia-batch-04-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/medium-parentage-livonia-russia-batch-04.sources data/staging/medium-parentage-livonia-russia-batch-04.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/medium-parentage-livonia-russia-batch-04-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/medium-parentage-livonia-russia-batch-04.sources data/staging/medium-parentage-livonia-russia-batch-04.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-gap-report data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-report.md`
- `cargo fmt --check`
- `rg -n "title-q186096|title-q2346056|Tsardom of Russia|Kingdom of Livonia|src-wikipedia-kingdom-livonia" src -S`
- `git diff --check`
- `cargo test --quiet -j 1`
