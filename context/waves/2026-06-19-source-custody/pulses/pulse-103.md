# Pulse 103: Prince-Bishopric of Basel HRE Parentage

## Intent

Close Prince-Bishopric of Basel's parentage gap under the Holy Roman Empire for
the reviewed ecclesiastical-principality span.

## Changes

- Add one reviewed Wikimedia text source for Prince-Bishopric of Basel parentage
  context.
- Promote Q319586 -> Q12548 for 1032-1803.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Basel diocese origin, Moutier-Grandval transfer, Imperial Diet seat, Upper
Rhenish Circle, city of Basel independence, Swiss Confederacy membership,
Reformation residence shift, French annexation, Baden mediation, or territorial
list claims are imported in this packet.

## Current State

- sources: 365
- facts: 1213
- titles: 335
- parentage facts: 208
- titles with parentage: 164
- titles without parentage: 171

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/basel-prince-bishopric-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/basel-prince-bishopric-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
