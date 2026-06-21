# Pulse 85: County of Savoy HRE Parentage

## Intent

Add a reviewed text-backed source for County of Savoy parentage and close the
Savoy parentage gap under the Holy Roman Empire.

## Changes

- Add one reviewed Wikimedia text source for County of Savoy.
- Promote Q1232887 -> Q12548 for 1003-1416.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No House of Savoy holder sequence, Kingdom of Arles suzerainty mechanics,
Imperial-immediacy detail, March of Turin inheritance, County of Nice or Geneva
acquisitions, duchy elevation mechanics, or later Savoyard state transitions are
imported in this packet.

## Current State

- sources: 344
- facts: 1183
- titles: 331
- parentage facts: 190
- titles with parentage: 146
- titles without parentage: 185

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/savoy-county-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/savoy-county-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
