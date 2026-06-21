# Pulse 132: Medium Albanian Kingdom Italian Empire Parentage Batch 06

## Intent

Continue the medium-priority parentage queue with a narrow endpoint parentage
span for a documented occupation and imperial incorporation.

## Changes

- Add one reviewed Wikimedia text source for Albanian Kingdom endpoint parentage
  context.
- Import `title-q1048340 -> title-q926295` for 1939.
- Regenerate parentage coverage, parentage gap TSV, full gap report, shard TSVs,
  and shard reports.

## Boundary

No invasion order of battle, Italian military administration, League of Nations
withdrawal, later Italian protectorate title facts, Greater Albania changes,
borders, resistance history, or post-1939 governance claims are imported.

## Current State

- sources: 419
- facts: 1271
- titles: 337
- parentage facts: 260
- titles without parentage: 122
- active high-priority parentage gaps: 0
- medium-priority parentage gaps: 81
- root/successor review rows: 39
- reviewed blocked parentage gaps: 2

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/medium-parentage-albanian-kingdom-italian-empire-batch-06-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/medium-parentage-albanian-kingdom-italian-empire-batch-06.sources data/staging/medium-parentage-albanian-kingdom-italian-empire-batch-06.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/medium-parentage-albanian-kingdom-italian-empire-batch-06-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/medium-parentage-albanian-kingdom-italian-empire-batch-06.sources data/staging/medium-parentage-albanian-kingdom-italian-empire-batch-06.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-gap-report data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-report.md`
- `cargo fmt --check`
- `rg -n "title-q1048340|Albanian Kingdom|src-wikipedia-italian-invasion-albania|fact-q1048340-parent-q926295" src -S`
- `git diff --check`
- `cargo test --quiet -j 1`
