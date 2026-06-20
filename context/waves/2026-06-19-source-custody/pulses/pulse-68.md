# Pulse 68: Commonwealth Boundary Parentage Packet

## Intent

Record the reviewed year-granular transition from Kingdom of Poland into the
Polish-Lithuanian Commonwealth at 1569.

## Changes

- Promote `title-q8890160` Kingdom of Poland -> `title-q172107`
  Polish-Lithuanian Commonwealth parentage for 1569..1569.
- Preserve the boundary review and relation rows in staging.
- Regenerate parentage coverage, gap TSV, gap shards, and gap reports.

## Boundary

This is intentionally a single-year parentage fact because the accepted child
span ends in 1569 and the accepted parent span begins in 1569. DUCHY currently
models spans at year precision.

## Current State

- sources: 336
- facts: 1146
- titles: 325
- parentage facts: 171
- titles with parentage: 128
- titles without parentage: 197

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/commonwealth-boundary-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/commonwealth-boundary-parentage-01.sources data/staging/commonwealth-boundary-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/commonwealth-boundary-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/commonwealth-boundary-parentage-01.sources data/staging/commonwealth-boundary-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
