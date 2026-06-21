# Pulse 121: High-Priority Mixed Parentage Batch 08

## Intent

Close two high-priority parentage gaps with accepted higher-rank endpoint
parent titles.

## Changes

- Add three reviewed Wikimedia text sources for parentage context.
- Promote:
  - Q1917014 -> Q188586 for 1130-1130.
  - Q706553 -> Q2577303 for 1814-1818.
- Preserve the source packet, dry-run report, and apply report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Norman conquest detail, Roger II biography, Apulia/Calabria relation, papal
recognition dispute, wider Savoyard State inventory, Genoa annexation,
Piedmont-Sardinia diplomatic history, Italian unification, or post-endpoint
successor claims are imported in this batch.

## Current State

- sources: 406
- facts: 1254
- titles: 335
- parentage facts: 249
- titles with parentage: 204
- titles without parentage: 131
- high-priority parentage gaps remaining: 5

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/high-parentage-mixed-batch-08-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/high-parentage-mixed-batch-08-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
