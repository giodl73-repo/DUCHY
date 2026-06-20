# Pulse 75: Electoral Palatinate HRE Parentage

## Intent

Correct Electoral Palatinate rank semantics and close its parentage gap with a
reviewed Holy Roman Empire relation.

## Changes

- Correct `fact-q22880-rank` from Empire to Duchy.
- Promote Q22880 -> Q12548 for 1085-1803.
- Preserve the rank correction, relation screen, dry-run report, and apply
  report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No electoral vote mechanics, rulers, territories, partition history, borders,
or post-1803 successor administration are imported in this packet.

## Current State

- sources: 338
- facts: 1160
- titles: 327
- parentage facts: 179
- titles with parentage: 135
- titles without parentage: 192

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/electoral-palatinate-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/electoral-palatinate-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
