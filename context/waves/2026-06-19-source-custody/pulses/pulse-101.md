# Pulse 101: Duchy of Florence HRE Parentage

## Intent

Close Duchy of Florence's parentage gap under the Holy Roman Empire for its
accepted duchy span.

## Changes

- Add one reviewed Wikimedia text source for Duchy of Florence parentage
  context.
- Promote Q2252973 -> Q12548 for 1532-1569.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Medici genealogy, republican constitution, siege operations, Siena conquest,
Elba purchase, naval base, succession dispute, Grand Duchy of Tuscany title
continuation, papal-imperial title dispute, or later Tuscany-Habsburg claims are
imported in this packet.

## Current State

- sources: 363
- facts: 1211
- titles: 335
- parentage facts: 206
- titles with parentage: 162
- titles without parentage: 173

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/florence-duchy-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/florence-duchy-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
