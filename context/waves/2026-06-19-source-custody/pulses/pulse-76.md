# Pulse 76: Free Imperial City of Aachen HRE Parentage

## Intent

Correct Free Imperial City of Aachen rank semantics and close its parentage gap
with a reviewed Holy Roman Empire relation.

## Changes

- Add `FreeCity` rank support.
- Correct `fact-q2629137-rank` from Empire to FreeCity.
- Promote Q2629137 -> Q12548 for 1306-1801.
- Preserve the rank correction, relation screen, dry-run report, and apply
  report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No city government details, imperial immediacy mechanics, spa/city status,
territorial borders, rulers, or post-1801 successor administration are imported
in this packet.

## Current State

- sources: 338
- facts: 1161
- titles: 327
- parentage facts: 180
- titles with parentage: 136
- titles without parentage: 191

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/aachen-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/aachen-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
