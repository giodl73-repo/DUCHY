# Pulse 150: Ireland UKGBI Parentage

## Intent

Promote the bounded United Kingdom of Great Britain and Ireland title and import
Kingdom of Ireland endpoint parentage into it for 1801.

## Changes

- Add `data/staging/ireland-ukgbi-parentage-01.sources`.
- Add `data/staging/ireland-ukgbi-parentage-01.facts`.
- Add `data/staging/ireland-ukgbi-parentage-01-review.md`.
- Promote United Kingdom of Great Britain and Ireland as a composite `Crown`
  title for `1801..1922`.
- Promote Kingdom of Ireland endpoint parentage under the United Kingdom of
  Great Britain and Ireland for `1801..1801`.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

This packet imports only the bounded UKGBI title and Ireland endpoint parentage
relation. Great Britain is intentionally deferred because DUCHY currently needs
successor/replacement semantics for same-rank `Crown -> Crown` transitions. No
parliamentary mechanics, Catholic emancipation, representation, churches, army,
flag, monarchy, empire, dominions, Irish Free State, Northern Ireland, modern
United Kingdom identity, or post-1922/1927 continuity claims are promoted.

## Current State

- reviewed sources: 434
- accepted facts: 1310
- accepted titles: 347
- parentage facts: 269
- titles with parentage: 224
- titles without parentage: 123
- parentage gap rows: 123

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/ireland-ukgbi-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ireland-ukgbi-parentage-01.sources data/staging/ireland-ukgbi-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/ireland-ukgbi-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ireland-ukgbi-parentage-01.sources data/staging/ireland-ukgbi-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
