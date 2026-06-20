# Pulse 78: Courland Commonwealth Parentage

## Intent

Close the Duchy of Courland and Semigallia parentage gap with a reviewed
Polish-Lithuanian Commonwealth relation.

## Changes

- Promote Q156038 -> Q172107 for 1569-1795.
- Preserve the relation screen, dry-run report, and apply report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No ducal succession, colonial holdings, border geometry, Polish-Lithuanian
internal offices, religious policy, or post-1795 partition successor claims are
imported in this packet.

## Current State

- sources: 338
- facts: 1163
- titles: 327
- parentage facts: 182
- titles with parentage: 138
- titles without parentage: 189

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/courland-commonwealth-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/courland-commonwealth-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
