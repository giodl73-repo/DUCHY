# Pulse 82: Hispanic Monarchy Crown Parentage

## Intent

Promote Hispanic Monarchy as a reviewed parent title and close the Crown of
Aragon and Crown of Castile parentage gaps with reviewed relations.

## Changes

- Promote Q766543 Hispanic Monarchy as an `Empire` super-composite parent title.
- Promote Q204920 -> Q766543 for 1479-1700.
- Promote Q217196 -> Q766543 for 1479-1700.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No full Spanish succession, colonial administration, personal-union mechanics,
Portuguese or Iberian Union handling, post-1700 Bourbon succession, borders, or
component-title inventory is imported in this packet.

## Current State

- sources: 342
- facts: 1180
- titles: 331
- parentage facts: 187
- titles with parentage: 143
- titles without parentage: 188

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/hispanic-monarchy-crown-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/hispanic-monarchy-crown-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
