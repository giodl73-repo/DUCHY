# Pulse 89: Prince-Bishopric of Augsburg HRE Parentage

## Intent

Add a reviewed text-backed source for Prince-Bishopric of Augsburg parentage
and close its parentage gap under the Holy Roman Empire.

## Changes

- Add one reviewed Wikimedia text source for Prince-Bishopric of Augsburg.
- Promote Q173863 -> Q12548 for 962-1803.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No prince-bishop holder sequence, city of Augsburg Free Imperial City
separation mechanics, Swabian Circle administration, ecclesiastical diocese
extent, Bavarian mediatization details, borders, or later diocesan continuity
claims are imported in this packet.

## Current State

- sources: 348
- facts: 1190
- titles: 332
- parentage facts: 194
- titles with parentage: 150
- titles without parentage: 182

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/augsburg-prince-bishopric-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/augsburg-prince-bishopric-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
