# Pulse 120: High-Priority Mixed Parentage Batch 07

## Intent

Close five high-priority parentage gaps with explicit endpoint or vassalage
source claims and accepted higher-rank parent titles.

## Changes

- Add five reviewed Wikimedia text sources for parentage context.
- Promote:
  - Q1991540 -> Q43287 for 1918-1918.
  - Q2273304 -> Q12560 for 1390-1402.
  - Q686312 -> Q12544 for 839-958.
  - Q693570 -> Q170174 for 1597-1597.
  - Q2295939 -> Q70972 for 1453-1453.
- Preserve the source packet, dry-run report, and apply report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No earlier Courland duchy, United Baltic Duchy successor, Latvian state
transition, Serbian Empire predecessor, independent Moravian Serbia period,
Duchy of Naples intermediate parentage, later Amalfi independence, Norman
conquest, Este rule, papal-fief analysis, Aquitaine intermediate parentage,
English rule, or post-endpoint successor claims are imported in this batch.

## Current State

- sources: 403
- facts: 1252
- titles: 335
- parentage facts: 247
- titles with parentage: 202
- titles without parentage: 133
- high-priority parentage gaps remaining: 7

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/high-parentage-mixed-batch-07-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/high-parentage-mixed-batch-07-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
