# Pulse 118: High-Priority Mixed Parentage Batch 05

## Intent

Close two high-priority parentage gaps with accepted higher-rank parent titles.

## Changes

- Add two reviewed Wikimedia text sources for parentage context.
- Promote:
  - Q736029 -> Q12548 for 1303-1328.
  - Q736029 -> Q12548 for 1606-1743.
  - Q83546 -> Q141472 for 1259-1389.
- Preserve the source packet, dry-run report, and apply report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Nassau-Dillenburg genealogy, Catholic/Protestant subdivision detail,
Orange-Nassau-Dietz succession, rank correction, initial Mongol Empire
parentage, appanage fragmentation, grand-princely appointment mechanics, Moscow
transfer, tax-system detail, or post-1389 Vladimir claims are imported in this
batch. Gascony and Nice were reviewed but excluded because their obvious
intermediate parents are not higher-rank parents under current validation.

## Current State

- sources: 393
- facts: 1242
- titles: 335
- parentage facts: 237
- titles with parentage: 192
- titles without parentage: 143
- high-priority parentage gaps remaining: 17

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/high-parentage-mixed-batch-05-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/high-parentage-mixed-batch-05-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
