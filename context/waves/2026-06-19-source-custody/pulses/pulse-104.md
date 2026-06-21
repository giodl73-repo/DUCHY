# Pulse 104: Prince-Bishopric of Toul HRE Parentage

## Intent

Close Prince-Bishopric of Toul's parentage gap under the Holy Roman Empire for
the reviewed imperial-state span.

## Changes

- Add one reviewed Wikimedia text source for Prince-Bishopric of Toul parentage
  context.
- Promote Q328001 -> Q12548 for 1048-1648.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No diocese foundation, city of Toul imperial-city status, Three Bishoprics
administration, French provincial governance, Treaty of Westphalia detail,
Concordat of 1802, Nancy-Toul merger, suffragan status, or territorial list
claims are imported in this packet.

## Current State

- sources: 366
- facts: 1214
- titles: 335
- parentage facts: 209
- titles with parentage: 165
- titles without parentage: 170

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/toul-prince-bishopric-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/toul-prince-bishopric-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
