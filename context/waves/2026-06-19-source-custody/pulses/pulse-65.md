# Pulse 65: Crown Bridge Parentage Packet

## Intent

Use the newly materialized Crown rank to close a reviewed set of unparented
kingdom gaps whose parent crowns are already accepted.

## Changes

- Promote 10 parentage-only facts using accepted source records.
- Link accepted kingdoms under Crown of Aragon or Crown of Castile.
- Preserve the reviewed relation rows in
  `data/staging/parentage-gap-crown-bridges-01-relations.tsv`.
- Regenerate parentage coverage, gap TSV, gap shards, and gap reports.

## Current State

- sources: 336
- facts: 1142
- titles: 325
- parentage facts: 167
- titles with parentage: 124
- titles without parentage: 201

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/parentage-gap-crown-bridges-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-crown-bridges-01.sources data/staging/parentage-gap-crown-bridges-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/parentage-gap-crown-bridges-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-crown-bridges-01.sources data/staging/parentage-gap-crown-bridges-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
