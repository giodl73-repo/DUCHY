# Pulse 84: Prince-Bishopric of Trent HRE Parentage

## Intent

Add a reviewed text-backed source for Prince-Bishopric of Trent parentage and
close the Trent parentage gap under the Holy Roman Empire.

## Changes

- Add one reviewed Wikimedia text source for Prince-Bishopric of Trent.
- Promote Q1231403 -> Q12548 for 1027-1803.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Trentino border geometry, prince-bishop holder sequence, Tyrol succession,
Napoleonic secularization detail, Austrian crown-land transition, or later
Italian territorial claims are imported in this packet.

## Current State

- sources: 343
- facts: 1182
- titles: 331
- parentage facts: 189
- titles with parentage: 145
- titles without parentage: 186

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/trent-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/trent-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
