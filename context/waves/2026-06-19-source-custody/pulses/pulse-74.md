# Pulse 74: Illyrian Provinces French Empire Parentage

## Intent

Correct Illyrian Provinces rank semantics and close its parentage gap with a
reviewed First French Empire relation.

## Changes

- Add `Province` as a supported title rank below kingdom-level parents.
- Correct `fact-q699923-rank` from Empire to Province.
- Promote Q699923 -> Q71084 for 1809-1815.
- Preserve the rank correction, relation screen, dry-run report, and apply
  report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No borders, departments, governors, de facto control, successor administration,
or post-1815 Austrian transitions are imported in this packet.

## Current State

- sources: 338
- facts: 1159
- titles: 327
- parentage facts: 178
- titles with parentage: 134
- titles without parentage: 193

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/illyrian-provinces-french-empire-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/illyrian-provinces-french-empire-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
