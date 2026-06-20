# Pulse 73: Duchy of Urbino Papal States Parentage

## Intent

Close a reviewed parentage gap for Duchy of Urbino using an accepted structured
relation to the Papal States.

## Changes

- Promote Q649202 -> Q170174 for 1443-1631.
- Preserve the relation screen, review note, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No borders, holders, de facto control, succession events, papal annexation
mechanics, or post-1631 territorial administration are imported in this packet.

## Current State

- sources: 338
- facts: 1158
- titles: 327
- parentage facts: 177
- titles with parentage: 133
- titles without parentage: 194

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/urbino-papal-states-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/urbino-papal-states-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
