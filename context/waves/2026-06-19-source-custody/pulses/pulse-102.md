# Pulse 102: Prince-Bishopric of Freising HRE Parentage

## Intent

Close Prince-Bishopric of Freising's parentage gap under the Holy Roman Empire
for the reviewed ecclesiastical-principality span.

## Changes

- Add one reviewed Wikimedia text source for Prince-Bishopric of Freising
  parentage context.
- Promote Q259511 -> Q12548 for 1294-1802.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Freising diocese, abbey foundation, dispersed lordships, Yserrain,
Werdenfels, Reichstag, Bavarian Circle, Wittelsbach, witch trial, Thirty Years'
War, secularisation implementation details, or later Munich-Freising archdiocese
claims are imported in this packet.

## Current State

- sources: 364
- facts: 1212
- titles: 335
- parentage facts: 207
- titles with parentage: 163
- titles without parentage: 172

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/freising-prince-bishopric-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/freising-prince-bishopric-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
