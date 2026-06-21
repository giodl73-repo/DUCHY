# Pulse 115: High-Priority HRE County and Duchy Parentage Batch 02

## Intent

Close three more straightforward high-priority parentage gaps under the Holy
Roman Empire.

## Changes

- Add three reviewed Wikimedia text sources for HRE parentage context.
- Promote:
  - Q47261 -> Q12548 for 962-1805.
  - Q675363 -> Q12548 for 1032-1401.
  - Q660393 -> Q12548 for 962-1190.
- Preserve the source packet, dry-run report, and apply report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No predecessor polities, ducal partitions, dynastic succession, city-bishop
conflicts, Burgundian predecessor detail, Brabant inheritance, later titulature,
Napoleonic transition detail, or post-title successor claims are imported in
this batch.

## Current State

- sources: 382
- facts: 1230
- titles: 335
- parentage facts: 225
- titles with parentage: 181
- titles without parentage: 154
- high-priority parentage gaps remaining: 28

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/high-parentage-hre-batch-02-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/high-parentage-hre-batch-02-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
