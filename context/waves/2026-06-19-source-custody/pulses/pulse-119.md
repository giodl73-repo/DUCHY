# Pulse 119: High-Priority Mixed Parentage Batch 06

## Intent

Close five high-priority parentage gaps with accepted higher-rank parent titles.

## Changes

- Add five reviewed Wikimedia text sources for parentage context.
- Promote:
  - Q2183293 -> Q171348 for 1387-1462.
  - Q6673921 -> Q153080 for 880-959.
  - Q684030 -> Q12560 for 1815-1867.
  - Q958291 -> Q12560 for 1859-1877.
  - Q3324486 -> Q12560 for 1516-1696.
- Preserve the source packet, dry-run report, and apply report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Kievan Rus, Galicia-Volhynia, Lithuanian, Hungarian, Masovian, independent
Lotharingian kingdom, West Frankish division, Middle Francia predecessor,
post-959 successor, Serbian revolutionary predecessor, post-1867 Serbia
independence debate, Moldavia or Wallachia predecessor, post-1877 Romania
claims, Zeta predecessor, or post-1696 Montenegro independence claims are
imported in this batch.

## Current State

- sources: 398
- facts: 1247
- titles: 335
- parentage facts: 242
- titles with parentage: 197
- titles without parentage: 138
- high-priority parentage gaps remaining: 12

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/high-parentage-mixed-batch-06-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/high-parentage-mixed-batch-06-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
