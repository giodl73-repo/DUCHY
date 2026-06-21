# Pulse 110: Duchy of Athens Thessalonica Parentage

## Intent

Close the first reviewed Duchy of Athens parentage span under the Kingdom of
Thessalonica.

## Changes

- Add one reviewed Wikimedia text source for Duchy of Athens parentage context.
- Promote Q334714 -> Q325461 for 1205-1224.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Fourth Crusade narrative, Byzantine predecessor, Achaea suzerainty dispute,
Sicilian or Aragonese suzerainty, Venetian control, Morea tributary status,
Ottoman conquest, titular claims, or post-1224 parentage is imported in this
packet.

## Current State

- sources: 372
- facts: 1220
- titles: 335
- parentage facts: 215
- titles with parentage: 171
- titles without parentage: 164

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/athens-thessalonica-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/athens-thessalonica-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
