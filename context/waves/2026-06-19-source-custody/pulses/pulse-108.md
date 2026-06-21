# Pulse 108: County of Wurttemberg HRE Parentage

## Intent

Close County of Wurttemberg's parentage gap under the Holy Roman Empire for
its reviewed county span.

## Changes

- Add one reviewed Wikimedia text source for County of Wurttemberg parentage
  context.
- Promote Q2991474 -> Q12548 for 1083-1495.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No House of Wurttemberg genealogy, Duchy of Swabia succession, Treaty of
Nurtingen division, Treaty of Munsingen reunification, Duchy of Wurttemberg
continuation, electorate, kingdom, or post-HRE claims are imported in this
packet.

## Current State

- sources: 370
- facts: 1218
- titles: 335
- parentage facts: 213
- titles with parentage: 169
- titles without parentage: 166

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/wurttemberg-county-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/wurttemberg-county-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
