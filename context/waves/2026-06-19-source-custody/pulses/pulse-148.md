# Pulse 148: Dortmund HRE Parentage

## Intent

Continue the medium-priority parentage pass with a direct Holy Roman Empire
parentage packet for the already accepted Free imperial city of Dortmund title.

## Changes

- Add `data/staging/dortmund-hre-parentage-01.sources`.
- Add `data/staging/dortmund-hre-parentage-01.facts`.
- Add `data/staging/dortmund-hre-parentage-01-review.md`.
- Promote Free imperial city of Dortmund parentage under the Holy Roman Empire
  for `1220..1803`.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

This packet imports only the bounded free-imperial-city parentage relation. No
city government, Hanseatic status, trade history, urban rights, diet vote,
constitutional mechanics, territorial extent, population, Grand Duchy of Berg,
Prussian administration, or modern municipal history is promoted.

## Current State

- reviewed sources: 432
- accepted facts: 1305
- accepted titles: 346
- parentage facts: 267
- titles with parentage: 222
- titles without parentage: 124
- parentage gap rows: 124

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/dortmund-hre-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/dortmund-hre-parentage-01.sources data/staging/dortmund-hre-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/dortmund-hre-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/dortmund-hre-parentage-01.sources data/staging/dortmund-hre-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
