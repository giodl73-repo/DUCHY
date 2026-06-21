# Pulse 86: Duchy of Warsaw French Empire Parentage

## Intent

Add a reviewed text-backed source for Duchy of Warsaw parentage and close the
Warsaw parentage gap under the First French Empire.

## Changes

- Add one reviewed Wikimedia text source for Duchy of Warsaw.
- Promote Q152115 -> Q71084 for 1807-1815.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Treaties of Tilsit detail, personal union with Saxony, military control,
departmental administration, territorial expansion in 1809, Congress of Vienna
partition, borders, or successor-title mechanics are imported in this packet.

## Current State

- sources: 345
- facts: 1184
- titles: 331
- parentage facts: 191
- titles with parentage: 147
- titles without parentage: 184

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/warsaw-french-empire-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/warsaw-french-empire-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
