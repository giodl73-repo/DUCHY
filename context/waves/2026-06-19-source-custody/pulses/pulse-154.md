# Pulse 154: Burgundians Western Roman Parentage

## Intent

Promote Western Roman Empire as a bounded parent title and import the clipped
Kingdom of the Burgundians parentage context under it for `443..476`.

## Changes

- Add `data/staging/burgundians-western-roman-parentage-01.sources`.
- Add `data/staging/burgundians-western-roman-parentage-01.facts`.
- Add `data/staging/burgundians-western-roman-parentage-01-review.md`.
- Promote Western Roman Empire as an `Empire` title for `395..476`.
- Promote Kingdom of the Burgundians parentage under the Western Roman Empire
  for `443..476`.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

This packet imports only the Western Roman Empire title identity and the clipped
`443..476` Burgundian parentage context. No Burgundian royal genealogy, battle
narrative, territorial geometry, legal-code history, Roman constitutional theory,
Frankish conquest, post-476 continuity, or later Burgundy claims are promoted.

## Current State

- reviewed sources: 438
- accepted facts: 1321
- accepted titles: 349
- parentage facts: 274
- titles with parentage: 229
- titles without parentage: 120
- parentage gap rows: 120

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/burgundians-western-roman-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/burgundians-western-roman-parentage-01.sources data/staging/burgundians-western-roman-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/burgundians-western-roman-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/burgundians-western-roman-parentage-01.sources data/staging/burgundians-western-roman-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
