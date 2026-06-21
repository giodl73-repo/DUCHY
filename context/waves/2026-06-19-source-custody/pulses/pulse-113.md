# Pulse 113: County of Luxembourg HRE Parentage

## Intent

Close County of Luxembourg's parentage gap under the Holy Roman Empire for its
reviewed county span.

## Changes

- Add one reviewed Wikimedia text source for County of Luxembourg parentage
  context.
- Promote Q5177890 -> Q12548 for 963-1354.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Ardennes genealogy, Luxembourg dynasty, imperial elections, Duchy of
Luxembourg continuation, Burgundian or Habsburg Netherlands, French annexation,
grand-duchy, or later Luxembourg claims are imported in this packet.

## Current State

- sources: 375
- facts: 1223
- titles: 335
- parentage facts: 218
- titles with parentage: 174
- titles without parentage: 161

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/luxembourg-county-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/luxembourg-county-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
