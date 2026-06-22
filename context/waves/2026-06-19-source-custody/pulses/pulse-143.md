# Pulse 143: Dorohoi Romania Parentage

## Intent

Start the post-title-harvest parentage milestone by importing the cleanest
high-priority county parentage row.

## Changes

- Add `data/staging/dorohoi-romania-parentage-01.sources`.
- Add `data/staging/dorohoi-romania-parentage-01.facts`.
- Add `data/staging/dorohoi-romania-parentage-01-review.md`.
- Promote Dorohoi County parentage under the Kingdom of Romania for
  `1881..1947`.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

The Dorohoi title exists for `1859..1950`, but this packet promotes only the
overlap with the accepted Kingdom of Romania title. No United Principalities,
Romanian People's Republic, wartime occupation, Hertsa region, Holocaust,
district organization, demographics, or successor county claims are promoted.

## Current State

- reviewed sources: 428
- accepted facts: 1299
- accepted titles: 346
- parentage facts: 261
- county parentage titles: 31
- titles without parentage: 130
- parentage gap rows: 130

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/dorohoi-romania-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/dorohoi-romania-parentage-01.sources data/staging/dorohoi-romania-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/dorohoi-romania-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/dorohoi-romania-parentage-01.sources data/staging/dorohoi-romania-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
