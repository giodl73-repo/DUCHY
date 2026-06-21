# Pulse 122: High-Priority Endpoint Parentage Batch 09

## Intent

Close two high-priority endpoint successor transitions with accepted higher-rank
parent titles.

## Changes

- Add two reviewed Wikimedia text sources for endpoint transition context.
- Promote:
  - Q104863335 -> Q8890160 for 1025-1025.
  - Q825902 -> Q172107 for 1569-1569.
- Preserve the source packet, dry-run report, and apply report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Piast genealogy, later Polish monarchy periods, feudal fragmentation, Sejm
mechanics, federal structure, elective monarchy detail, partitions, or
post-endpoint successor claims are imported in this batch.

## Current State

- sources: 408
- facts: 1256
- titles: 335
- parentage facts: 251
- titles with parentage: 206
- titles without parentage: 129
- high-priority parentage gaps remaining: 3

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/high-parentage-endpoint-batch-09-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/high-parentage-endpoint-batch-09-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
