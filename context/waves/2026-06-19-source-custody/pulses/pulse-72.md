# Pulse 72: Gorizia and Gradisca Austrian Parentage

## Intent

Correct Gorizia and Gradisca rank semantics and close its parentage gap with
reviewed Austrian parent titles.

## Changes

- Correct `fact-q692946-rank` from Empire to County.
- Promote Q692946 -> Q131964 for 1804-1866.
- Promote Q692946 -> Q28513 for 1867-1918.
- Preserve the rank correction, relation screen, dry-run report, and apply
  report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No borders, crown-land administration details, holders, de facto control,
successor/predecessor transitions, or post-1918 transitions are imported in
this packet.

## Current State

- sources: 338
- facts: 1157
- titles: 327
- parentage facts: 176
- titles with parentage: 132
- titles without parentage: 195

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/gorizia-gradisca-austrian-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/gorizia-gradisca-austrian-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
