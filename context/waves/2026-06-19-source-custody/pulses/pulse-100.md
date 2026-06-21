# Pulse 100: Duchy of Bohemia HRE Parentage

## Intent

Close Duchy of Bohemia's parentage gap under the Holy Roman Empire for the
reviewed imperial-state span.

## Changes

- Add one reviewed Wikimedia text source for Duchy of Bohemia parentage context.
- Promote Q2162698 -> Q12548 for 1002-1198.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Great Moravian formation, Premyslid genealogy, fealty to East Francia,
Polish occupation, diocesan subordination, royal-title episodes before 1198,
Golden Bull of Sicily, Kingdom of Bohemia continuation, electoral status, Crown
of Bohemia lands, or Habsburg-period claims are imported in this packet.

## Current State

- sources: 362
- facts: 1210
- titles: 335
- parentage facts: 205
- titles with parentage: 161
- titles without parentage: 174

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/bohemia-duchy-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/bohemia-duchy-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
