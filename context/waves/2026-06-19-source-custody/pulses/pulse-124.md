# Pulse 124: High-Priority Norway Parentage Batch 10

## Intent

Close Hordaland's high-priority parentage gap by adding reviewed modern Kingdom
of Norway parent coverage.

## Changes

- Add two reviewed sources for modern Norway parent coverage.
- Promote:
  - Q20 identity, rank, and bounded existence facts for 1905-2026.
  - Q50625 -> Q20 for 1919-2019.
- Preserve the source packet, dry-run report, and apply report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No constitutional detail, medieval continuity claim, union-with-Sweden detail,
occupation status, overseas territories, current-officeholder claims, or
post-2026 continuation claims are imported in this batch.

## Current State

- sources: 410
- facts: 1260
- titles: 336
- parentage facts: 252
- titles with parentage: 207
- titles without parentage: 129
- high-priority parentage gaps remaining: 2

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/high-parentage-norway-batch-10-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/high-parentage-norway-batch-10-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
