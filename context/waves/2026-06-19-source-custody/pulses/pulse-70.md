# Pulse 70: Kingdom of Poland German Empire Parentage

## Intent

Close one accepted parentage gap using an already reviewed German Empire source
and an explicit structured relation from the 1916-1918 Kingdom of Poland.

## Changes

- Promote `title-q696908` Kingdom of Poland -> `title-q43287` German Empire
  parentage for 1916..1918.
- Preserve the reviewed relation row in staging.
- Regenerate parentage coverage, gap TSV, gap shards, and gap reports.

## Boundary

This packet imports only the reviewed `P463` relation for the accepted child and
parent titles. It does not import borders, occupation zones, holders,
Austria-Hungary involvement, de facto control, successor/predecessor relations,
or broader First World War dependency semantics.

## Current State

- sources: 337
- facts: 1151
- titles: 326
- parentage facts: 173
- titles with parentage: 130
- titles without parentage: 196

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/kingdom-poland-german-empire-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/kingdom-poland-german-empire-parentage-01.sources data/staging/kingdom-poland-german-empire-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/kingdom-poland-german-empire-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/kingdom-poland-german-empire-parentage-01.sources data/staging/kingdom-poland-german-empire-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
