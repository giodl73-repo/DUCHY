# Pulse 117: High-Priority Bohemian/Silesian Parentage Batch 04

## Intent

Close five high-priority Silesian duchy parentage gaps under the already
materialized Kingdom of Bohemia.

## Changes

- Add five reviewed Wikimedia text sources for Bohemian/Silesian parentage
  context.
- Promote:
  - Q566639 -> Q42585 for 1269-1918.
  - Q568638 -> Q42585 for 1335-1742.
  - Q671899 -> Q42585 for 1327-1918.
  - Q682001 -> Q42585 for 1289-1498.
  - Q702327 -> Q42585 for 1327-1457.
- Preserve the source packet, dry-run report, and apply report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No predecessor polities, Piast branch genealogy, partition inventory, Moravian
or Lesser Poland predecessor detail, sale mechanics, Habsburg administration,
Prussian annexation detail, Austrian Silesia residual claims, title continuity,
or post-parentage claims are imported in this batch.

## Current State

- sources: 391
- facts: 1239
- titles: 335
- parentage facts: 234
- titles with parentage: 190
- titles without parentage: 145
- high-priority parentage gaps remaining: 19

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/high-parentage-bohemia-batch-04-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/high-parentage-bohemia-batch-04-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
