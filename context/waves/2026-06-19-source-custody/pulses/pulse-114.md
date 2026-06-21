# Pulse 114: High-Priority HRE County and Duchy Parentage Batch 01

## Intent

Close four straightforward high-priority parentage gaps under the Holy Roman
Empire.

## Changes

- Add four reviewed Wikimedia text sources for HRE parentage context.
- Promote:
  - Q657241 -> Q12548 for 1180-1803.
  - Q573290 -> Q12548 for 1140-1806.
  - Q589251 -> Q12548 for 1042-1793.
  - Q599613 -> Q12548 for 981-1795.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No predecessor polities, dynastic genealogies, administrative-circle details,
personal unions, French Revolutionary occupation detail, post-HRE successor
settlements, modern regional claims, or province boundaries are imported in
this batch.

## Current State

- sources: 379
- facts: 1227
- titles: 335
- parentage facts: 222
- titles with parentage: 178
- titles without parentage: 157
- high-priority parentage gaps remaining: 31

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/high-parentage-hre-batch-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/high-parentage-hre-batch-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
