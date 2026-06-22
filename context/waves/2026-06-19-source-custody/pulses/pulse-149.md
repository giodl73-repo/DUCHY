# Pulse 149: Fulda HRE Parentage

## Intent

Continue the medium-priority parentage pass with a direct Holy Roman Empire
parentage packet for the already accepted Princely Abbey of Fulda title.

## Changes

- Add `data/staging/fulda-hre-parentage-01.sources`.
- Add `data/staging/fulda-hre-parentage-01.facts`.
- Add `data/staging/fulda-hre-parentage-01-review.md`.
- Promote Princely Abbey of Fulda parentage under the Holy Roman Empire for
  `1221..1802`.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

This packet imports only the bounded princely-abbey parentage relation. No
monastery foundation claim, abbey school history, offices, Imperial Diet voting
mechanics, Upper Rhenish Circle membership, prince-bishopric elevation,
Nassau-Orange-Fulda transfer, later French/Austrian/Prussian custody, or modern
diocese history is promoted.

## Current State

- reviewed sources: 432
- accepted facts: 1306
- accepted titles: 346
- parentage facts: 268
- titles with parentage: 223
- titles without parentage: 123
- parentage gap rows: 123

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/fulda-hre-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/fulda-hre-parentage-01.sources data/staging/fulda-hre-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/fulda-hre-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/fulda-hre-parentage-01.sources data/staging/fulda-hre-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
