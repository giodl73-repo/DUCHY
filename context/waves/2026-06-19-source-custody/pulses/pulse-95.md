# Pulse 95: Grand Duchy of Tuscany HRE Parentage

## Intent

Add a reviewed text-backed source for Grand Duchy of Tuscany parentage and close
its parentage gap under the Holy Roman Empire.

## Changes

- Add one reviewed Wikimedia text source for Grand Duchy of Tuscany parentage
  context.
- Promote Q154849 -> Q12548 for 1575-1801.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No papal-bull validity dispute, Medici succession, Spanish claims,
Habsburg-Lorraine inheritance, secundogeniture, Napoleonic client-state,
restoration after 1814, Italian unification, borders, title-holder, or later
Tuscany claims are imported in this packet.

## Current State

- sources: 355
- facts: 1199
- titles: 333
- parentage facts: 200
- titles with parentage: 156
- titles without parentage: 177

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/tuscany-grand-duchy-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/tuscany-grand-duchy-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
