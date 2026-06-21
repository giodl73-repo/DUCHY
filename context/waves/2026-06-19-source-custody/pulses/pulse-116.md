# Pulse 116: High-Priority Mixed Parentage Batch 03

## Intent

Close four high-priority parentage gaps whose accepted parents are already
materialized.

## Changes

- Add four reviewed Wikimedia text sources for parentage context.
- Promote:
  - Q836937 -> Q12548 for 1003-1794.
  - Q698089 -> Q154741 for 1806-1813.
  - Q704312 -> Q154741 for 1810-1813.
  - Q842091 -> Q70972 for 987-1204.
- Preserve the source packet, dry-run report, and apply report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No predecessor polities, title-elevation details, personal unions, dynastic
succession, French annexation detail, Napoleonic constitutional detail, Congress
of Vienna settlement, royal-domain administration, English or Angevin claims,
Channel Islands continuation, or post-1204 Normandy disputed-claim parentage are
imported in this batch.

## Current State

- sources: 386
- facts: 1234
- titles: 335
- parentage facts: 229
- titles with parentage: 185
- titles without parentage: 150
- high-priority parentage gaps remaining: 24

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/high-parentage-mixed-batch-03-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/high-parentage-mixed-batch-03-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
