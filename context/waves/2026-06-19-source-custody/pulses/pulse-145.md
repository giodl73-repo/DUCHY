# Pulse 145: La Marche French Crown Parentage

## Intent

Continue post-title-harvest parentage closure with a tightly bounded endpoint
relation for the County of La Marche.

## Changes

- Add `data/staging/la-marche-french-crown-parentage-01.sources`.
- Add `data/staging/la-marche-french-crown-parentage-01.facts`.
- Add `data/staging/la-marche-french-crown-parentage-01-review.md`.
- Promote County of La Marche parentage under the Kingdom of France for
  `1527..1527`.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

This packet promotes only the 1527 French crown confiscation endpoint. No
Aquitaine, Poitou, Bourbon, Armagnac, province, parlement, appanage,
title-holder sequence, territorial extent, or later honorary title claims are
promoted.

## Current State

- reviewed sources: 429
- accepted facts: 1301
- accepted titles: 346
- parentage facts: 263
- titles with parentage: 218
- titles without parentage: 128
- parentage gap rows: 128

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/la-marche-french-crown-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/la-marche-french-crown-parentage-01.sources data/staging/la-marche-french-crown-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/la-marche-french-crown-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/la-marche-french-crown-parentage-01.sources data/staging/la-marche-french-crown-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
