# Pulse 71: Russian Empire Finland Parentage

## Intent

Resolve the Grand Duchy of Finland parentage gap by importing a reviewed Russian
Empire parent source and the explicit structured relation between them.

## Changes

- Promote Russian Empire (Q34266) as `title-q34266` with 1721..1917 existence.
- Promote `title-q62633` Grand Duchy of Finland -> `title-q34266` Russian
  Empire parentage for 1809..1917.
- Preserve the reviewed relation row, dry-run report, and apply report in
  staging.
- Regenerate parentage coverage, gap TSV, gap shards, and gap reports.

## Boundary

This packet imports only the Russian Empire title identity facts and the reviewed
Grand Duchy of Finland parentage relation. It does not import borders, holders,
autonomy semantics, de facto control, successor/predecessor relations, the
Russian Republic, or Finnish independence transition details.

## Current State

- sources: 338
- facts: 1155
- titles: 327
- parentage facts: 174
- titles with parentage: 131
- titles without parentage: 196

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/russian-empire-finland-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/russian-empire-finland-parentage-01.sources data/staging/russian-empire-finland-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/russian-empire-finland-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/russian-empire-finland-parentage-01.sources data/staging/russian-empire-finland-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
