# Pulse 77: Burgraviate of Nuremberg HRE Parentage

## Intent

Correct Burgraviate of Nuremberg rank semantics and close its parentage gap with
a reviewed Holy Roman Empire relation.

## Changes

- Correct `fact-q568473-rank` from Empire to County.
- Promote Q568473 -> Q12548 for 1105-1440.
- Preserve the rank correction, relation screen, dry-run report, and apply
  report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No burgrave office history, dynastic succession, castle holdings, city
government, Franconian circle mechanics, or post-1440 successor territories are
imported in this packet.

## Current State

- sources: 338
- facts: 1162
- titles: 327
- parentage facts: 181
- titles with parentage: 137
- titles without parentage: 190

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/nuremberg-burgraviate-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/nuremberg-burgraviate-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
